//! Configuration types for StateSet SDK

use std::time::Duration;
use url::Url;

/// Configuration for the StateSet client
#[derive(Debug, Clone)]
pub struct Config {
    /// Base URL for the StateSet API
    pub base_url: Url,
    /// Request timeout
    pub timeout: Duration,
    /// Number of retry attempts
    pub retry_attempts: u32,
    /// Retry delay
    pub retry_delay: Duration,
    /// Maximum retry delay
    pub max_retry_delay: Duration,
    /// Rate limit per minute (if enabled)
    pub rate_limit: Option<(u32, Duration)>,
    /// User agent string
    pub user_agent: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_url: Url::parse("https://api.stateset.io").unwrap(),
            timeout: Duration::from_secs(30),
            retry_attempts: 3,
            retry_delay: Duration::from_millis(1000),
            max_retry_delay: Duration::from_secs(60),
            rate_limit: None,
            user_agent: format!("stateset-rust-sdk/{}", env!("CARGO_PKG_VERSION")),
        }
    }
}

impl Config {
    /// Create a new configuration builder
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    /// Create a new configuration with a custom base URL
    pub fn with_base_url(base_url: impl AsRef<str>) -> crate::Result<Self> {
        let url = Url::parse(base_url.as_ref())
            .map_err(|e| crate::Error::Configuration(format!("Invalid base URL: {}", e)))?;
        
        Ok(Self {
            base_url: url,
            ..Default::default()
        })
    }
}

/// Builder for creating Config instances
#[derive(Debug, Default)]
pub struct ConfigBuilder {
    base_url: Option<Url>,
    timeout: Option<Duration>,
    retry_attempts: Option<u32>,
    retry_delay: Option<Duration>,
    max_retry_delay: Option<Duration>,
    rate_limit: Option<(u32, Duration)>,
    user_agent: Option<String>,
}

impl ConfigBuilder {
    /// Set the base URL
    pub fn base_url(mut self, url: impl AsRef<str>) -> Self {
        self.base_url = Url::parse(url.as_ref()).ok();
        self
    }

    /// Set the request timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set the number of retry attempts
    pub fn retry_attempts(mut self, attempts: u32) -> Self {
        self.retry_attempts = Some(attempts);
        self
    }

    /// Set the retry delay
    pub fn retry_delay(mut self, delay: Duration) -> Self {
        self.retry_delay = Some(delay);
        self
    }

    /// Set the maximum retry delay
    pub fn max_retry_delay(mut self, delay: Duration) -> Self {
        self.max_retry_delay = Some(delay);
        self
    }

    /// Set rate limiting (requests per duration)
    pub fn rate_limit(mut self, requests: u32, per: Duration) -> Self {
        self.rate_limit = Some((requests, per));
        self
    }

    /// Set a custom user agent
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Build the configuration
    pub fn build(self) -> crate::Result<Config> {
        let base_url = self.base_url
            .ok_or_else(|| crate::Error::Configuration("Base URL is required".into()))?;

        Ok(Config {
            base_url,
            timeout: self.timeout.unwrap_or(Duration::from_secs(30)),
            retry_attempts: self.retry_attempts.unwrap_or(3),
            retry_delay: self.retry_delay.unwrap_or(Duration::from_millis(1000)),
            max_retry_delay: self.max_retry_delay.unwrap_or(Duration::from_secs(60)),
            rate_limit: self.rate_limit,
            user_agent: self.user_agent.unwrap_or_else(|| {
                format!("stateset-rust-sdk/{}", env!("CARGO_PKG_VERSION"))
            }),
        })
    }
} 