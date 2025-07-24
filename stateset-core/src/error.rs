//! Error types for StateSet SDK

use std::time::Duration;
use thiserror::Error;

/// Result type alias for StateSet operations
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for StateSet SDK
#[derive(Debug, Error)]
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
    },

    /// Validation error
    #[error("Validation error: {message}")]
    Validation { message: String },

    /// Network error
    #[error("Network error: {0}")]
    Network(String),

    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Timeout error
    #[error("Request timed out")]
    Timeout,

    /// WebSocket error
    #[cfg(feature = "realtime")]
    #[error("WebSocket error: {0}")]
    WebSocket(String),

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
        }
    }

    /// Check if the error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::RateLimit { .. } | Self::Network(_) | Self::Timeout
        )
    }

    /// Get the status code if this is an API error
    pub fn status_code(&self) -> Option<u16> {
        match self {
            Self::Api { code, .. } => Some(*code),
            Self::NotFound => Some(404),
            Self::Authentication { .. } => Some(401),
            Self::Authorization { .. } => Some(403),
            Self::RateLimit { .. } => Some(429),
            Self::Validation { .. } => Some(400),
            Self::Timeout => Some(408),
            _ => None,
        }
    }
} 