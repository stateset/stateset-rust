//! Middleware for HTTP requests and responses

use std::time::{Duration, Instant};
use reqwest::{Request, Response};
use stateset_core::{Error, Result};

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
#[derive(Debug, Default)]
pub struct LoggingMiddleware {
    pub log_requests: bool,
    pub log_responses: bool,
    pub log_request_bodies: bool,
    pub log_response_bodies: bool,
}

impl LoggingMiddleware {
    /// Create a new logging middleware with all logging enabled
    pub fn all() -> Self {
        Self {
            log_requests: true,
            log_responses: true,
            log_request_bodies: true,
            log_response_bodies: true,
        }
    }

    /// Create a new logging middleware with only basic logging
    pub fn basic() -> Self {
        Self {
            log_requests: true,
            log_responses: true,
            log_request_bodies: false,
            log_response_bodies: false,
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
}

impl RequestMiddleware for LoggingMiddleware {
    fn process_request(&self, request: &Request) -> Result<()> {
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

            // TODO: Log request body if enabled and available
            if self.log_request_bodies {
                log::debug!("Request body logging not yet implemented");
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

            // TODO: Log response body if enabled
            if self.log_response_bodies {
                log::debug!("Response body logging not yet implemented");
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

/// Rate limiting middleware
#[derive(Debug)]
pub struct RateLimitMiddleware {
    // In a real implementation, this would contain rate limiting logic
    pub requests_per_minute: u32,
}

impl RateLimitMiddleware {
    pub fn new(requests_per_minute: u32) -> Self {
        Self {
            requests_per_minute,
        }
    }
}

impl RequestMiddleware for RateLimitMiddleware {
    fn process_request(&self, _request: &Request) -> Result<()> {
        // In a real implementation, this would check and enforce rate limits
        log::trace!("Rate limit check: {} requests/minute", self.requests_per_minute);
        Ok(())
    }
}

/// Circuit breaker middleware for handling service failures
#[derive(Debug)]
pub struct CircuitBreakerMiddleware {
    // In a real implementation, this would contain circuit breaker state
    pub failure_threshold: u32,
    pub recovery_timeout: Duration,
}

impl CircuitBreakerMiddleware {
    pub fn new(failure_threshold: u32, recovery_timeout: Duration) -> Self {
        Self {
            failure_threshold,
            recovery_timeout,
        }
    }
}

impl ResponseMiddleware for CircuitBreakerMiddleware {
    fn process_response(&self, response: &Response, _duration: Duration) -> Result<()> {
        let status = response.status().as_u16();
        
        // In a real implementation, this would track failures and open/close the circuit
        if status >= 500 {
            log::debug!("Circuit breaker: server error detected ({})", status);
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
}