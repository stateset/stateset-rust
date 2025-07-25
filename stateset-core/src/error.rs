//! Error types for StateSet SDK

use std::time::Duration;
use thiserror::Error;

/// Result type alias for StateSet operations
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for StateSet SDK
#[derive(Debug, Error, Clone, serde::Serialize)]
pub enum Error {
    /// Resource not found
    #[error("Resource not found")]
    NotFound,

    /// Authentication error
    #[error("Authentication failed: {message}")]
    Authentication { message: String },

    /// Authorization error
    #[error("Authorization failed: {message}")]
    Authorization { message: String },

    /// Rate limit exceeded
    #[error("Rate limit exceeded. Retry after {retry_after:?}")]
    RateLimit { retry_after: Option<Duration> },

    /// API error with status code and message
    #[error("API error {code}: {message}")]
    Api {
        code: u16,
        message: String,
        details: Option<serde_json::Value>,
        request_id: Option<String>,
    },

    /// Validation error with field-specific details
    #[error("Validation error: {message}")]
    Validation { 
        message: String,
        field: Option<String>,
        code: Option<String>,
    },

    /// Network error with retry information
    #[error("Network error: {message}")]
    Network { 
        message: String,
        is_timeout: bool,
        can_retry: bool,
    },

    /// Serialization/deserialization error
    #[error("Serialization error: {message}")]
    Serialization { message: String },

    /// Configuration error with helpful hints
    #[error("Configuration error: {message}")]
    Configuration { 
        message: String,
        hint: Option<String>,
    },

    /// Timeout error with operation context
    #[error("Request timed out after {duration:?} for operation: {operation}")]
    Timeout { 
        duration: Duration,
        operation: String,
    },

    /// WebSocket error
    #[cfg(feature = "realtime")]
    #[error("WebSocket error: {message}")]
    WebSocket { 
        message: String,
        can_reconnect: bool,
    },

    /// Retry exhausted error
    #[error("Maximum retry attempts ({attempts}) exceeded for operation: {operation}")]
    RetryExhausted {
        attempts: u32,
        operation: String,
        last_error: Box<Error>,
    },

    /// Connection pool error
    #[error("Connection pool error: {message}")]
    ConnectionPool { message: String },

    /// Resource conflict (409)
    #[error("Resource conflict: {message}")]
    Conflict { 
        message: String,
        retry_after: Option<Duration>,
    },

    /// Service unavailable (503)
    #[error("Service temporarily unavailable: {message}")]
    ServiceUnavailable { 
        message: String,
        retry_after: Option<Duration>,
    },

    /// Invalid request format
    #[error("Invalid request: {message}")]
    InvalidRequest { 
        message: String,
        parameter: Option<String>,
    },

    /// Resource quota exceeded
    #[error("Quota exceeded for {resource}: {message}")]
    QuotaExceeded {
        resource: String,
        message: String,
        reset_time: Option<chrono::DateTime<chrono::Utc>>,
    },

    /// Other errors
    #[error("{0}")]
    Other(String),
}

impl Error {
    /// Create a new API error
    pub fn api(code: u16, message: impl Into<String>) -> Self {
        Self::Api {
            code,
            message: message.into(),
            details: None,
            request_id: None,
        }
    }

    /// Create a new API error with details
    pub fn api_with_details(
        code: u16,
        message: impl Into<String>,
        details: serde_json::Value,
    ) -> Self {
        Self::Api {
            code,
            message: message.into(),
            details: Some(details),
            request_id: None,
        }
    }

    /// Create a new API error with request ID
    pub fn api_with_request_id(
        code: u16,
        message: impl Into<String>,
        request_id: impl Into<String>,
    ) -> Self {
        Self::Api {
            code,
            message: message.into(),
            details: None,
            request_id: Some(request_id.into()),
        }
    }

    /// Create a new authentication error
    pub fn auth(message: impl Into<String>) -> Self {
        Self::Authentication {
            message: message.into(),
        }
    }

    /// Create a new validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
            field: None,
            code: None,
        }
    }

    /// Create a validation error with field information
    pub fn validation_field(
        message: impl Into<String>,
        field: impl Into<String>,
    ) -> Self {
        Self::Validation {
            message: message.into(),
            field: Some(field.into()),
            code: None,
        }
    }

    /// Create a network error
    pub fn network(message: impl Into<String>) -> Self {
        Self::Network {
            message: message.into(),
            is_timeout: false,
            can_retry: true,
        }
    }

    /// Create a timeout error
    pub fn timeout(duration: Duration, operation: impl Into<String>) -> Self {
        Self::Timeout {
            duration,
            operation: operation.into(),
        }
    }

    /// Create a configuration error with hint
    pub fn config_with_hint(
        message: impl Into<String>,
        hint: impl Into<String>,
    ) -> Self {
        Self::Configuration {
            message: message.into(),
            hint: Some(hint.into()),
        }
    }

    /// Check if the error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::RateLimit { .. } 
            | Self::ServiceUnavailable { .. }
            | Self::Timeout { .. } => true,
            Self::Network { can_retry, .. } => *can_retry,
            Self::Api { code, .. } => matches!(*code, 500..=599),
            _ => false,
        }
    }

    /// Get the suggested retry delay
    pub fn retry_after(&self) -> Option<Duration> {
        match self {
            Self::RateLimit { retry_after, .. }
            | Self::Conflict { retry_after, .. }
            | Self::ServiceUnavailable { retry_after, .. } => *retry_after,
            Self::Network { .. } => Some(Duration::from_secs(1)),
            _ => None,
        }
    }

    /// Get the status code if this is an API error
    pub fn status_code(&self) -> Option<u16> {
        match self {
            Self::Api { code, .. } => Some(*code),
            Self::NotFound => Some(404),
            Self::Authentication { .. } => Some(401),
            Self::Authorization { .. } => Some(403),
            Self::RateLimit { .. } => Some(429),
            Self::Validation { .. } | Self::InvalidRequest { .. } => Some(400),
            Self::Conflict { .. } => Some(409),
            Self::ServiceUnavailable { .. } => Some(503),
            Self::Timeout { .. } => Some(408),
            _ => None,
        }
    }

    /// Get the request ID if available
    pub fn request_id(&self) -> Option<&str> {
        match self {
            Self::Api { request_id, .. } => request_id.as_deref(),
            _ => None,
        }
    }

    /// Check if this is a client error (4xx)
    pub fn is_client_error(&self) -> bool {
        self.status_code()
            .map(|code| (400..500).contains(&code))
            .unwrap_or(false)
    }

    /// Check if this is a server error (5xx)
    pub fn is_server_error(&self) -> bool {
        self.status_code()
            .map(|code| (500..600).contains(&code))
            .unwrap_or(false)
    }

    /// Add context to the error
    pub fn with_context(self, context: impl Into<String>) -> Self {
        match self {
            Self::Other(msg) => Self::Other(format!("{}: {}", context.into(), msg)),
            _ => self,
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Serialization {
            message: err.to_string(),
        }
    }
} 