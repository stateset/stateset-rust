//! Middleware for HTTP requests and responses

use std::time::{Duration, Instant};
use reqwest::{Request, Response};
use stateset_core::{Error, Result};
use std::collections::HashSet;

/// Trait for request middleware
pub trait RequestMiddleware: Send + Sync {
    /// Process the request before sending
    fn process_request(&self, request: &mut Request) -> Result<()>;
}

/// Trait for response middleware
pub trait ResponseMiddleware: Send + Sync {
    /// Process the response after receiving
    fn process_response(&self, response: &Response, duration: Duration) -> Result<()>;
}

/// Logging middleware for requests and responses
#[derive(Debug)]
pub struct LoggingMiddleware {
    pub log_requests: bool,
    pub log_responses: bool,
    pub log_request_bodies: bool,
    pub log_response_bodies: bool,
    pub max_body_size: usize,
    pub sensitive_fields: HashSet<String>,
}

impl Default for LoggingMiddleware {
    fn default() -> Self {
        let mut sensitive_fields = HashSet::new();
        sensitive_fields.insert("password".to_string());
        sensitive_fields.insert("token".to_string());
        sensitive_fields.insert("api_key".to_string());
        sensitive_fields.insert("secret".to_string());
        sensitive_fields.insert("authorization".to_string());
        sensitive_fields.insert("credit_card".to_string());
        sensitive_fields.insert("ssn".to_string());
        
        Self {
            log_requests: false,
            log_responses: false,
            log_request_bodies: false,
            log_response_bodies: false,
            max_body_size: 8192, // 8KB max body logging
            sensitive_fields,
        }
    }
}

impl LoggingMiddleware {
    /// Create a new logging middleware with all logging enabled
    pub fn all() -> Self {
        Self {
            log_requests: true,
            log_responses: true,
            log_request_bodies: true,
            log_response_bodies: true,
            ..Default::default()
        }
    }

    /// Create a new logging middleware with only basic logging
    pub fn basic() -> Self {
        Self {
            log_requests: true,
            log_responses: true,
            log_request_bodies: false,
            log_response_bodies: false,
            ..Default::default()
        }
    }

    /// Enable or disable request logging
    pub fn requests(mut self, enabled: bool) -> Self {
        self.log_requests = enabled;
        self
    }

    /// Enable or disable response logging
    pub fn responses(mut self, enabled: bool) -> Self {
        self.log_responses = enabled;
        self
    }

    /// Enable or disable request body logging
    pub fn request_bodies(mut self, enabled: bool) -> Self {
        self.log_request_bodies = enabled;
        self
    }

    /// Enable or disable response body logging
    pub fn response_bodies(mut self, enabled: bool) -> Self {
        self.log_response_bodies = enabled;
        self
    }

    /// Set maximum body size for logging
    pub fn max_body_size(mut self, size: usize) -> Self {
        self.max_body_size = size;
        self
    }

    /// Add a sensitive field name to redact from logs
    pub fn add_sensitive_field(mut self, field: impl Into<String>) -> Self {
        self.sensitive_fields.insert(field.into());
        self
    }

    /// Redact sensitive information from a JSON string
    fn redact_sensitive_json(&self, json: &str) -> String {
        if let Ok(mut value) = serde_json::from_str::<serde_json::Value>(json) {
            self.redact_value(&mut value);
            serde_json::to_string(&value).unwrap_or_else(|_| "[INVALID JSON]".to_string())
        } else {
            // If it's not valid JSON, just truncate if too long
            if json.len() > self.max_body_size {
                format!("{}... [TRUNCATED]", &json[..self.max_body_size])
            } else {
                json.to_string()
            }
        }
    }

    /// Recursively redact sensitive fields in JSON value
    fn redact_value(&self, value: &mut serde_json::Value) {
        match value {
            serde_json::Value::Object(map) => {
                for (key, val) in map.iter_mut() {
                    if self.is_sensitive_field(key) {
                        *val = serde_json::Value::String("[REDACTED]".to_string());
                    } else {
                        self.redact_value(val);
                    }
                }
            }
            serde_json::Value::Array(arr) => {
                for item in arr.iter_mut() {
                    self.redact_value(item);
                }
            }
            _ => {}
        }
    }

    /// Check if a field name is sensitive
    fn is_sensitive_field(&self, field_name: &str) -> bool {
        let field_lower = field_name.to_lowercase();
        self.sensitive_fields.iter().any(|sensitive| {
            field_lower.contains(&sensitive.to_lowercase())
        })
    }
}

impl RequestMiddleware for LoggingMiddleware {
    fn process_request(&self, request: &mut Request) -> Result<()> {
        if self.log_requests {
            log::info!(
                "HTTP Request: {} {}",
                request.method(),
                request.url()
            );

            // Log headers (excluding sensitive ones)
            for (name, value) in request.headers() {
                let name_str = name.as_str();
                if !is_sensitive_header(name_str) {
                    if let Ok(value_str) = value.to_str() {
                        log::debug!("Request header: {}: {}", name_str, value_str);
                    }
                } else {
                    log::debug!("Request header: {}: [REDACTED]", name_str);
                }
            }

            // Log request body if enabled
            if self.log_request_bodies {
                if let Some(body) = request.body() {
                    if let Some(bytes) = body.as_bytes() {
                        if bytes.len() <= self.max_body_size {
                            if let Ok(body_str) = std::str::from_utf8(bytes) {
                                let redacted_body = self.redact_sensitive_json(body_str);
                                log::debug!("Request body: {}", redacted_body);
                            } else {
                                log::debug!("Request body: [BINARY DATA, {} bytes]", bytes.len());
                            }
                        } else {
                            log::debug!("Request body: [TOO LARGE, {} bytes]", bytes.len());
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

impl ResponseMiddleware for LoggingMiddleware {
    fn process_response(&self, response: &Response, duration: Duration) -> Result<()> {
        if self.log_responses {
            log::info!(
                "HTTP Response: {} {} in {:?}",
                response.status().as_u16(),
                response.url(),
                duration
            );

            // Log response headers
            for (name, value) in response.headers() {
                let name_str = name.as_str();
                if let Ok(value_str) = value.to_str() {
                    log::debug!("Response header: {}: {}", name_str, value_str);
                }
            }

            // Log response body if enabled
            // Note: This is a limitation of the current design - we can't easily access
            // the response body here without consuming it. In a real implementation,
            // we'd need to restructure the middleware to work with a copy of the body.
            if self.log_response_bodies {
                log::debug!("Response body logging requires middleware restructuring");
            }
        }
        Ok(())
    }
}

/// Metrics middleware for collecting request statistics
#[derive(Debug, Default)]
pub struct MetricsMiddleware {
    // In a real implementation, this would contain metrics collectors
    // For now, we'll just log metrics
}

impl MetricsMiddleware {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ResponseMiddleware for MetricsMiddleware {
    fn process_response(&self, response: &Response, duration: Duration) -> Result<()> {
        let status = response.status().as_u16();
        let method = response.url().as_str(); // In a real implementation, get method from request
        
        // In a real implementation, these would be sent to a metrics system
        log::trace!(
            "Request metrics: method={}, status={}, duration_ms={}",
            method,
            status,
            duration.as_millis()
        );

        // Track error rates
        if status >= 400 {
            log::trace!("Request error: status={}", status);
        }

        Ok(())
    }
}

/// User agent middleware for adding custom user agent headers
#[derive(Debug)]
pub struct UserAgentMiddleware {
    user_agent: String,
}

impl UserAgentMiddleware {
    pub fn new(user_agent: impl Into<String>) -> Self {
        Self {
            user_agent: user_agent.into(),
        }
    }
}

impl RequestMiddleware for UserAgentMiddleware {
    fn process_request(&self, request: &mut Request) -> Result<()> {
        // Note: In a real implementation, we'd need mutable access to headers
        // This is a simplified example
        log::debug!("Would set User-Agent to: {}", self.user_agent);
        Ok(())
    }
}

/// Rate limiting middleware with token bucket implementation
#[derive(Debug)]
pub struct RateLimitMiddleware {
    pub requests_per_minute: u32,
    // In a real implementation, we'd use a proper token bucket or rate limiter
    last_reset: std::time::Instant,
    tokens_remaining: std::sync::atomic::AtomicU32,
}

impl RateLimitMiddleware {
    pub fn new(requests_per_minute: u32) -> Self {
        Self {
            requests_per_minute,
            last_reset: std::time::Instant::now(),
            tokens_remaining: std::sync::atomic::AtomicU32::new(requests_per_minute),
        }
    }

    /// Check if a request should be allowed
    pub fn check_rate_limit(&self) -> bool {
        // Simplified rate limiting - in production use a proper rate limiter library
        let elapsed = self.last_reset.elapsed();
        if elapsed >= Duration::from_secs(60) {
            // Reset tokens
            self.tokens_remaining.store(
                self.requests_per_minute,
                std::sync::atomic::Ordering::Relaxed
            );
            return true;
        }

        let current = self.tokens_remaining.load(std::sync::atomic::Ordering::Relaxed);
        if current > 0 {
            self.tokens_remaining.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
            true
        } else {
            false
        }
    }
}

impl RequestMiddleware for RateLimitMiddleware {
    fn process_request(&self, _request: &mut Request) -> Result<()> {
        if !self.check_rate_limit() {
            return Err(Error::RateLimit {
                retry_after: Some(Duration::from_secs(60)),
            });
        }
        log::trace!("Rate limit check passed: {} requests/minute", self.requests_per_minute);
        Ok(())
    }
}

/// Circuit breaker middleware for handling service failures
#[derive(Debug)]
pub struct CircuitBreakerMiddleware {
    pub failure_threshold: u32,
    pub recovery_timeout: Duration,
    failure_count: std::sync::atomic::AtomicU32,
    last_failure: std::sync::Mutex<Option<std::time::Instant>>,
    state: std::sync::atomic::AtomicU8, // 0 = Closed, 1 = Open, 2 = HalfOpen
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CircuitState {
    Closed = 0,
    Open = 1,
    HalfOpen = 2,
}

impl From<u8> for CircuitState {
    fn from(value: u8) -> Self {
        match value {
            0 => CircuitState::Closed,
            1 => CircuitState::Open,
            2 => CircuitState::HalfOpen,
            _ => CircuitState::Closed,
        }
    }
}

impl CircuitBreakerMiddleware {
    pub fn new(failure_threshold: u32, recovery_timeout: Duration) -> Self {
        Self {
            failure_threshold,
            recovery_timeout,
            failure_count: std::sync::atomic::AtomicU32::new(0),
            last_failure: std::sync::Mutex::new(None),
            state: std::sync::atomic::AtomicU8::new(CircuitState::Closed as u8),
        }
    }

    fn get_state(&self) -> CircuitState {
        self.state.load(std::sync::atomic::Ordering::Relaxed).into()
    }

    fn can_execute(&self) -> bool {
        match self.get_state() {
            CircuitState::Closed => true,
            CircuitState::Open => {
                // Check if we should transition to half-open
                if let Ok(last_failure) = self.last_failure.lock() {
                    if let Some(last) = *last_failure {
                        if last.elapsed() >= self.recovery_timeout {
                            self.state.store(CircuitState::HalfOpen as u8, std::sync::atomic::Ordering::Relaxed);
                            return true;
                        }
                    }
                }
                false
            }
            CircuitState::HalfOpen => true,
        }
    }

    fn record_success(&self) {
        self.failure_count.store(0, std::sync::atomic::Ordering::Relaxed);
        self.state.store(CircuitState::Closed as u8, std::sync::atomic::Ordering::Relaxed);
    }

    fn record_failure(&self) {
        let failures = self.failure_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
        
        if let Ok(mut last_failure) = self.last_failure.lock() {
            *last_failure = Some(std::time::Instant::now());
        }

        if failures >= self.failure_threshold {
            self.state.store(CircuitState::Open as u8, std::sync::atomic::Ordering::Relaxed);
        }
    }
}

impl RequestMiddleware for CircuitBreakerMiddleware {
    fn process_request(&self, _request: &mut Request) -> Result<()> {
        if !self.can_execute() {
            return Err(Error::ServiceUnavailable {
                message: "Circuit breaker is open".to_string(),
                retry_after: Some(self.recovery_timeout),
            });
        }
        Ok(())
    }
}

impl ResponseMiddleware for CircuitBreakerMiddleware {
    fn process_response(&self, response: &Response, _duration: Duration) -> Result<()> {
        let status = response.status().as_u16();
        
        if status >= 500 {
            self.record_failure();
            log::debug!("Circuit breaker: server error detected ({})", status);
        } else if status < 400 {
            self.record_success();
        }
        
        Ok(())
    }
}

/// Check if a header name is sensitive and should be redacted
fn is_sensitive_header(name: &str) -> bool {
    let name_lower = name.to_lowercase();
    matches!(
        name_lower.as_str(),
        "authorization" | "cookie" | "set-cookie" | "x-api-key" | "x-auth-token"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensitive_header_detection() {
        assert!(is_sensitive_header("Authorization"));
        assert!(is_sensitive_header("authorization"));
        assert!(is_sensitive_header("Cookie"));
        assert!(is_sensitive_header("X-API-Key"));
        assert!(!is_sensitive_header("Content-Type"));
        assert!(!is_sensitive_header("Accept"));
    }

    #[test]
    fn test_logging_middleware_creation() {
        let middleware = LoggingMiddleware::all();
        assert!(middleware.log_requests);
        assert!(middleware.log_responses);
        assert!(middleware.log_request_bodies);
        assert!(middleware.log_response_bodies);

        let middleware = LoggingMiddleware::basic();
        assert!(middleware.log_requests);
        assert!(middleware.log_responses);
        assert!(!middleware.log_request_bodies);
        assert!(!middleware.log_response_bodies);
    }

    #[test]
    fn test_middleware_configuration() {
        let middleware = LoggingMiddleware::default()
            .requests(true)
            .responses(false)
            .request_bodies(true);
            
        assert!(middleware.log_requests);
        assert!(!middleware.log_responses);
        assert!(middleware.log_request_bodies);
        assert!(!middleware.log_response_bodies);
    }

    #[test]
    fn test_sensitive_field_redaction() {
        let middleware = LoggingMiddleware::default();
        let json = r#"{"name": "John", "password": "secret123", "api_key": "key123"}"#;
        let redacted = middleware.redact_sensitive_json(json);
        
        assert!(redacted.contains("John"));
        assert!(redacted.contains("[REDACTED]"));
        assert!(!redacted.contains("secret123"));
        assert!(!redacted.contains("key123"));
    }

    #[test]
    fn test_circuit_breaker_states() {
        let cb = CircuitBreakerMiddleware::new(3, Duration::from_secs(30));
        
        // Initially closed
        assert_eq!(cb.get_state(), CircuitState::Closed);
        assert!(cb.can_execute());
        
        // Record failures
        cb.record_failure();
        cb.record_failure();
        assert_eq!(cb.get_state(), CircuitState::Closed);
        
        cb.record_failure();
        assert_eq!(cb.get_state(), CircuitState::Open);
        assert!(!cb.can_execute());
        
        // Success should close the circuit
        cb.record_success();
        assert_eq!(cb.get_state(), CircuitState::Closed);
    }

    #[test]
    fn test_rate_limiter() {
        let limiter = RateLimitMiddleware::new(2);
        
        assert!(limiter.check_rate_limit());
        assert!(limiter.check_rate_limit());
        assert!(!limiter.check_rate_limit()); // Should be rate limited
    }
}