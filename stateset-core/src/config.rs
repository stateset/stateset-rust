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
    /// Connection timeout
    pub connect_timeout: Duration,
    /// Number of retry attempts
    pub retry_attempts: u32,
    /// Initial retry delay
    pub retry_delay: Duration,
    /// Maximum retry delay
    pub max_retry_delay: Duration,
    /// Retry multiplier for exponential backoff
    pub retry_multiplier: f64,
    /// Rate limit per minute (if enabled)
    pub rate_limit: Option<(u32, Duration)>,
    /// User agent string
    pub user_agent: String,
    /// Connection pool settings
    pub pool_settings: PoolSettings,
    /// Request compression
    pub compression: bool,
    /// Keep-alive settings
    pub keep_alive: Option<Duration>,
    /// Maximum redirects to follow
    pub max_redirects: u32,
    /// Default request headers
    pub default_headers: std::collections::HashMap<String, String>,
    /// TLS verification (should only be disabled for testing)
    pub tls_verification: bool,
}

/// Connection pool settings
#[derive(Debug, Clone)]
pub struct PoolSettings {
    /// Maximum number of connections per host
    pub max_connections_per_host: usize,
    /// Maximum total number of connections
    pub max_total_connections: usize,
    /// Connection idle timeout
    pub idle_timeout: Duration,
    /// Connection keep-alive timeout
    pub keep_alive_timeout: Duration,
}

impl Default for PoolSettings {
    fn default() -> Self {
        Self {
            max_connections_per_host: 10,
            max_total_connections: 100,
            idle_timeout: Duration::from_secs(30),
            keep_alive_timeout: Duration::from_secs(90),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut default_headers = std::collections::HashMap::new();
        default_headers.insert(
            "Accept".to_string(),
            "application/json".to_string(),
        );
        default_headers.insert(
            "Accept-Encoding".to_string(),
            "gzip, deflate, br".to_string(),
        );

        Self {
            base_url: Url::parse("https://api.stateset.io").unwrap(),
            timeout: Duration::from_secs(30),
            connect_timeout: Duration::from_secs(10),
            retry_attempts: 3,
            retry_delay: Duration::from_millis(1000),
            max_retry_delay: Duration::from_secs(60),
            retry_multiplier: 2.0,
            rate_limit: None,
            user_agent: format!("stateset-rust-sdk/{}", env!("CARGO_PKG_VERSION")),
            pool_settings: PoolSettings::default(),
            compression: true,
            keep_alive: Some(Duration::from_secs(90)),
            max_redirects: 10,
            default_headers,
            tls_verification: true,
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
            .map_err(|e| crate::Error::config_with_hint(
                format!("Invalid base URL: {}", e),
                "Ensure the URL starts with http:// or https://",
            ))?;
        
        Ok(Self {
            base_url: url,
            ..Default::default()
        })
    }

    /// Validate the configuration
    pub fn validate(&self) -> crate::Result<()> {
        // Validate URL scheme
        if !matches!(self.base_url.scheme(), "http" | "https") {
            return Err(crate::Error::config_with_hint(
                "Invalid URL scheme",
                "Only HTTP and HTTPS are supported",
            ));
        }

        // Validate timeouts
        if self.timeout.as_millis() == 0 {
            return Err(crate::Error::config_with_hint(
                "Timeout cannot be zero",
                "Set a reasonable timeout like Duration::from_secs(30)",
            ));
        }

        if self.connect_timeout > self.timeout {
            return Err(crate::Error::config_with_hint(
                "Connect timeout cannot be greater than request timeout",
                "Ensure connect_timeout <= timeout",
            ));
        }

        // Validate retry settings
        if self.retry_multiplier <= 1.0 {
            return Err(crate::Error::config_with_hint(
                "Retry multiplier must be greater than 1.0",
                "Use a value like 2.0 for exponential backoff",
            ));
        }

        // Validate pool settings
        if self.pool_settings.max_connections_per_host == 0 {
            return Err(crate::Error::config_with_hint(
                "Max connections per host cannot be zero",
                "Set a reasonable value like 10",
            ));
        }

        Ok(())
    }

    /// Get the effective timeout for retries
    pub fn total_timeout(&self) -> Duration {
        let mut total = Duration::from_millis(0);
        let mut delay = self.retry_delay;

        for _ in 0..self.retry_attempts {
            total += self.timeout + delay;
            delay = std::cmp::min(
                Duration::from_millis((delay.as_millis() as f64 * self.retry_multiplier) as u64),
                self.max_retry_delay,
            );
        }

        total
    }
}

/// Builder for creating Config instances
#[derive(Debug, Default)]
pub struct ConfigBuilder {
    base_url: Option<Url>,
    timeout: Option<Duration>,
    connect_timeout: Option<Duration>,
    retry_attempts: Option<u32>,
    retry_delay: Option<Duration>,
    max_retry_delay: Option<Duration>,
    retry_multiplier: Option<f64>,
    rate_limit: Option<(u32, Duration)>,
    user_agent: Option<String>,
    pool_settings: Option<PoolSettings>,
    compression: Option<bool>,
    keep_alive: Option<Duration>,
    max_redirects: Option<u32>,
    default_headers: Option<std::collections::HashMap<String, String>>,
    tls_verification: Option<bool>,
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

    /// Set the connection timeout
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = Some(timeout);
        self
    }

    /// Set the number of retry attempts
    pub fn retry_attempts(mut self, attempts: u32) -> Self {
        self.retry_attempts = Some(attempts);
        self
    }

    /// Set the initial retry delay
    pub fn retry_delay(mut self, delay: Duration) -> Self {
        self.retry_delay = Some(delay);
        self
    }

    /// Set the maximum retry delay
    pub fn max_retry_delay(mut self, delay: Duration) -> Self {
        self.max_retry_delay = Some(delay);
        self
    }

    /// Set the retry multiplier for exponential backoff
    pub fn retry_multiplier(mut self, multiplier: f64) -> Self {
        self.retry_multiplier = Some(multiplier);
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

    /// Set connection pool settings
    pub fn pool_settings(mut self, settings: PoolSettings) -> Self {
        self.pool_settings = Some(settings);
        self
    }

    /// Enable or disable compression
    pub fn compression(mut self, enabled: bool) -> Self {
        self.compression = Some(enabled);
        self
    }

    /// Set keep-alive timeout
    pub fn keep_alive(mut self, timeout: Option<Duration>) -> Self {
        self.keep_alive = timeout;
        self
    }

    /// Set maximum redirects to follow
    pub fn max_redirects(mut self, redirects: u32) -> Self {
        self.max_redirects = Some(redirects);
        self
    }

    /// Add a default header
    pub fn default_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.default_headers
            .get_or_insert_with(std::collections::HashMap::new)
            .insert(key.into(), value.into());
        self
    }

    /// Enable or disable TLS verification (for testing only)
    pub fn tls_verification(mut self, enabled: bool) -> Self {
        self.tls_verification = Some(enabled);
        self
    }

    /// Build the configuration
    pub fn build(self) -> crate::Result<Config> {
        let base_url = self.base_url
            .ok_or_else(|| crate::Error::config_with_hint(
                "Base URL is required",
                "Use .base_url(\"https://api.stateset.io\")",
            ))?;

        let default_config = Config::default();
        
        let config = Config {
            base_url,
            timeout: self.timeout.unwrap_or(default_config.timeout),
            connect_timeout: self.connect_timeout.unwrap_or(default_config.connect_timeout),
            retry_attempts: self.retry_attempts.unwrap_or(default_config.retry_attempts),
            retry_delay: self.retry_delay.unwrap_or(default_config.retry_delay),
            max_retry_delay: self.max_retry_delay.unwrap_or(default_config.max_retry_delay),
            retry_multiplier: self.retry_multiplier.unwrap_or(default_config.retry_multiplier),
            rate_limit: self.rate_limit.or(default_config.rate_limit),
            user_agent: self.user_agent.unwrap_or(default_config.user_agent),
            pool_settings: self.pool_settings.unwrap_or(default_config.pool_settings),
            compression: self.compression.unwrap_or(default_config.compression),
            keep_alive: self.keep_alive.or(default_config.keep_alive),
            max_redirects: self.max_redirects.unwrap_or(default_config.max_redirects),
            default_headers: self.default_headers.unwrap_or(default_config.default_headers),
            tls_verification: self.tls_verification.unwrap_or(default_config.tls_verification),
        };

        config.validate()?;
        Ok(config)
    }
} 