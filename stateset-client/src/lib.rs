//! HTTP client implementation for StateSet SDK

use reqwest::{Client as ReqwestClient, Method, RequestBuilder, Response};
use serde::{de::DeserializeOwned, Serialize};
use stateset_auth::Credentials;
use stateset_core::{Config, Error, Result};
use std::sync::Arc;
use std::time::{Duration, Instant};
use url::Url;

pub mod request;
pub mod resources;
pub mod retry;
pub mod middleware;

use retry::RetryPolicy;

/// StateSet HTTP client
#[derive(Clone)]
pub struct Client {
    inner: Arc<ClientInner>,
}

struct ClientInner {
    http: ReqwestClient,
    config: Config,
    credentials: Option<Credentials>,
    retry_policy: RetryPolicy,
}

impl Client {
    /// Create a new client with the default configuration
    pub fn new(base_url: impl AsRef<str>) -> Result<Self> {
        let config = Config::with_base_url(base_url)?;
        Self::with_config(config)
    }

    /// Create a new client with a custom configuration
    pub fn with_config(config: Config) -> Result<Self> {
        let mut builder = ReqwestClient::builder()
            .timeout(config.timeout)
            .connect_timeout(config.connect_timeout)
            .user_agent(&config.user_agent)
            .gzip(config.compression)
            .deflate(config.compression)
            .brotli(config.compression);

        // Configure connection pooling
        builder = builder
            .pool_max_idle_per_host(config.pool_settings.max_connections_per_host)
            .pool_idle_timeout(Some(config.pool_settings.idle_timeout));

        // Configure keep-alive
        if let Some(keep_alive) = config.keep_alive {
            builder = builder.tcp_keepalive(Some(keep_alive));
        }

        // Configure redirects
        builder = builder.redirect(reqwest::redirect::Policy::limited(config.max_redirects));

        // Configure TLS
        if !config.tls_verification {
            builder = builder.danger_accept_invalid_certs(true);
        }

        let http = builder
            .build()
            .map_err(|e| Error::config_with_hint(
                format!("Failed to create HTTP client: {}", e),
                "Check your configuration settings",
            ))?;

        let retry_policy = RetryPolicy::new(
            config.retry_attempts,
            config.retry_delay,
            config.max_retry_delay,
            config.retry_multiplier,
        );

        Ok(Self {
            inner: Arc::new(ClientInner {
                http,
                config,
                credentials: None,
                retry_policy,
            }),
        })
    }

    /// Authenticate the client with credentials
    pub fn authenticate(&self, credentials: Credentials) -> Self {
        let inner = ClientInner {
            http: self.inner.http.clone(),
            config: self.inner.config.clone(),
            credentials: Some(credentials),
            retry_policy: self.inner.retry_policy.clone(),
        };
        
        Self {
            inner: Arc::new(inner),
        }
    }

    /// Get the current configuration
    pub fn config(&self) -> &Config {
        &self.inner.config
    }

    /// Check if the client is authenticated
    pub fn is_authenticated(&self) -> bool {
        self.inner.credentials.is_some()
    }

    /// Build a request URL
    fn build_url(&self, path: &str) -> Result<Url> {
        self.inner
            .config
            .base_url
            .join(path)
            .map_err(|e| Error::config_with_hint(
                format!("Invalid URL path '{}': {}", path, e),
                "Ensure the path starts with '/' and contains valid characters",
            ))
    }

    /// Create a request builder with authentication and default headers
    fn request(&self, method: Method, path: &str) -> Result<RequestBuilder> {
        let url = self.build_url(path)?;
        let mut request = self.inner.http.request(method, url);

        // Add authentication header if available
        if let Some(credentials) = &self.inner.credentials {
            request = request.header("Authorization", credentials.authorization_header());
        }

        // Add default headers
        for (key, value) in &self.inner.config.default_headers {
            request = request.header(key, value);
        }

        // Add request ID for tracing with better format
        let request_id = format!("stateset-{}-{}", 
            chrono::Utc::now().timestamp_millis(),
            uuid::Uuid::new_v4().simple()
        );
        request = request.header("X-Request-ID", request_id);

        // Add client version for debugging
        request = request.header("X-Client-Version", env!("CARGO_PKG_VERSION"));

        Ok(request)
    }

    /// Execute a request with automatic retries and enhanced error handling
    async fn execute<T: DeserializeOwned>(&self, request: RequestBuilder) -> Result<T> {
        let operation = "execute_request";
        let start_time = Instant::now();

        let mut last_error = None;
        
        for attempt in 0..=self.inner.retry_policy.max_attempts {
            let request_clone = match request.try_clone() {
                Some(req) => req,
                None => {
                    return Err(Error::network("Request body is not cloneable for retries"));
                }
            };

            match self.execute_once(request_clone).await {
                Ok(response) => {
                    // Log successful request metrics
                    let duration = start_time.elapsed();
                    log::debug!(
                        "Request completed successfully in {:?} after {} attempt(s)",
                        duration,
                        attempt + 1
                    );
                    return Ok(response);
                }
                Err(error) => {
                    last_error = Some(error.clone());
                    
                    // Don't retry on the last attempt or non-retryable errors
                    if attempt >= self.inner.retry_policy.max_attempts || !error.is_retryable() {
                        break;
                    }

                    // Calculate delay for next attempt
                    let delay = self.inner.retry_policy.delay_for_attempt(attempt);
                    
                    // Respect retry-after header if present
                    let actual_delay = error.retry_after().unwrap_or(delay);
                    
                    log::debug!(
                        "Request failed (attempt {}/{}), retrying in {:?}: {}",
                        attempt + 1,
                        self.inner.retry_policy.max_attempts + 1,
                        actual_delay,
                        error
                    );

                    tokio::time::sleep(actual_delay).await;
                }
            }
        }

        // Return the last error, wrapped in a retry exhausted error with context
        let total_duration = start_time.elapsed();
        Err(Error::RetryExhausted {
            attempts: self.inner.retry_policy.max_attempts,
            operation: operation.to_string(),
            last_error: Box::new(last_error.unwrap()),
        }.with_context(format!("Total operation duration: {:?}", total_duration)))
    }

    /// Execute a single request attempt
    async fn execute_once<T: DeserializeOwned>(&self, request: RequestBuilder) -> Result<T> {
        let response = request
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    Error::timeout(self.inner.config.timeout, "http_request")
                } else if e.is_connect() {
                    Error::network("Connection failed")
                } else {
                    Error::network(e.to_string())
                }
            })?;

        self.handle_response(response).await
    }

    /// Handle the HTTP response with enhanced error processing
    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        let status = response.status();
        let request_id = response
            .headers()
            .get("x-request-id")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        if status.is_success() {
            response
                .json::<T>()
                .await
                .map_err(|e| Error::network(format!("Failed to parse JSON response: {}", e)))
        } else {
            let status_code = status.as_u16();
            
            // Extract retry-after header
            let retry_after = self.extract_retry_after(&response);
            
            let error_body = response.text().await.unwrap_or_default();

            // Try to parse as JSON error response
            let mut api_error = if let Ok(json_error) = serde_json::from_str::<serde_json::Value>(&error_body) {
                if let Some(message) = json_error.get("message").and_then(|v| v.as_str()) {
                    Error::api_with_details(status_code, message.to_string(), json_error)
                } else {
                    Error::api(status_code, error_body)
                }
            } else {
                Error::api(status_code, error_body)
            };

            // Add request ID if available
            if let Error::Api { request_id: ref mut req_id, .. } = api_error {
                *req_id = request_id;
            }

            // Return specific error types for common status codes
            match status_code {
                401 => Err(Error::Authentication {
                    message: "Unauthorized - check your API credentials".to_string(),
                }),
                403 => Err(Error::Authorization {
                    message: "Forbidden - insufficient permissions".to_string(),
                }),
                404 => Err(Error::NotFound),
                409 => Err(Error::Conflict {
                    message: "Resource conflict".to_string(),
                    retry_after,
                }),
                422 => {
                    // Try to extract validation errors
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&error_body) {
                        if let Some(errors) = json.get("errors").and_then(|e| e.as_array()) {
                            let field = errors.first()
                                .and_then(|e| e.get("field"))
                                .and_then(|f| f.as_str())
                                .map(|s| s.to_string());
                            let message = errors.first()
                                .and_then(|e| e.get("message"))
                                .and_then(|m| m.as_str())
                                .unwrap_or("Validation failed")
                                .to_string();
                            
                            return Err(Error::Validation {
                                message,
                                field,
                                code: None,
                            });
                        }
                    }
                    Err(api_error)
                }
                429 => Err(Error::RateLimit { retry_after }),
                503 => Err(Error::ServiceUnavailable {
                    message: "Service temporarily unavailable".to_string(),
                    retry_after,
                }),
                _ => Err(api_error),
            }
        }
    }

    /// Extract retry-after header from response
    fn extract_retry_after(&self, response: &Response) -> Option<Duration> {
        response
            .headers()
            .get("Retry-After")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok())
            .map(Duration::from_secs)
    }

    /// GET request
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let request = self.request(Method::GET, path)?;
        self.execute(request).await
    }

    /// GET request with query parameters
    pub async fn get_with_query<T, Q>(&self, path: &str, query: &Q) -> Result<T>
    where
        T: DeserializeOwned,
        Q: Serialize + ?Sized,
    {
        let request = self.request(Method::GET, path)?.query(query);
        self.execute(request).await
    }

    /// POST request
    pub async fn post<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let request = self.request(Method::POST, path)?.json(body);
        self.execute(request).await
    }

    /// PUT request
    pub async fn put<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let request = self.request(Method::PUT, path)?.json(body);
        self.execute(request).await
    }

    /// PATCH request
    pub async fn patch<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let request = self.request(Method::PATCH, path)?.json(body);
        self.execute(request).await
    }

    /// DELETE request
    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let request = self.request(Method::DELETE, path)?;
        self.execute(request).await
    }

    /// DELETE request without response body
    pub async fn delete_no_content(&self, path: &str) -> Result<()> {
        let request = self.request(Method::DELETE, path)?;
        let response = request
            .send()
            .await
            .map_err(|e| Error::network(e.to_string()))?;

        if response.status().is_success() {
            Ok(())
        } else {
            self.handle_response::<serde_json::Value>(response)
                .await
                .map(|_| ())
        }
    }

    /// Stream a paginated endpoint
    pub fn stream<T>(&self, path: &str) -> impl futures::Stream<Item = Result<T>>
    where
        T: DeserializeOwned + Send + 'static,
    {
        use futures::stream::{self, StreamExt, TryStreamExt};

        let client = self.clone();
        let path = path.to_string();

        stream::try_unfold(Some(path), move |state| {
            let client = client.clone();
            async move {
                match state {
                    Some(url) => {
                        match client.get::<serde_json::Value>(&url).await {
                            Ok(response) => {
                                // Extract data and next page information
                                let data = response.get("data")
                                    .and_then(|d| d.as_array())
                                    .cloned()
                                    .unwrap_or_default();

                                // Support both cursor-based and offset-based pagination
                                let next_url = response.get("next_page")
                                    .or_else(|| response.get("next"))
                                    .and_then(|n| n.as_str())
                                    .map(|s| s.to_string());

                                // Convert data to stream of individual items
                                let items: Vec<Result<T>> = data
                                    .into_iter()
                                    .map(|item| {
                                        serde_json::from_value(item)
                                            .map_err(|e| Error::network(format!("Failed to parse item: {}", e)))
                                    })
                                    .collect();

                                if items.is_empty() {
                                    // No more data
                                    Ok(None)
                                } else {
                                    Ok(Some((stream::iter(items), next_url)))
                                }
                            }
                            Err(e) => Err(e),
                        }
                    }
                    None => Ok(None),
                }
            }
        })
        .try_flatten()
    }

    /// Stream with custom query parameters
    pub fn stream_with_query<T, Q>(&self, path: &str, query: &Q) -> impl futures::Stream<Item = Result<T>>
    where
        T: DeserializeOwned + Send + 'static,
        Q: serde::Serialize + Clone + Send + 'static,
    {
        use futures::stream::{self, StreamExt, TryStreamExt};

        let client = self.clone();
        let path = path.to_string();
        let query = query.clone();

        stream::try_unfold(Some((path, query, true)), move |state| {
            let client = client.clone();
            async move {
                match state {
                    Some((url, query_params, is_first)) => {
                        let response = if is_first {
                            client.get_with_query::<serde_json::Value, _>(&url, &query_params).await
                        } else {
                            // For subsequent requests, the URL already contains query params
                            client.get::<serde_json::Value>(&url).await
                        };

                        match response {
                            Ok(response) => {
                                let data = response.get("data")
                                    .and_then(|d| d.as_array())
                                    .cloned()
                                    .unwrap_or_default();

                                let next_url = response.get("next_page")
                                    .or_else(|| response.get("next"))
                                    .and_then(|n| n.as_str())
                                    .map(|s| s.to_string());

                                let items: Vec<Result<T>> = data
                                    .into_iter()
                                    .map(|item| {
                                        serde_json::from_value(item)
                                            .map_err(|e| Error::network(format!("Failed to parse item: {}", e)))
                                    })
                                    .collect();

                                if items.is_empty() {
                                    Ok(None)
                                } else {
                                    let next_state = next_url.map(|url| (url, query_params, false));
                                    Ok(Some((stream::iter(items), next_state)))
                                }
                            }
                            Err(e) => Err(e),
                        }
                    }
                    None => Ok(None),
                }
            }
        })
        .try_flatten()
    }
}

// Resource accessors
impl Client {
    /// Access the Orders API
    pub fn orders(&self) -> resources::orders::OrdersClient {
        resources::orders::OrdersClient::new(self.clone())
    }

    /// Access the Inventory API
    pub fn inventory(&self) -> resources::inventory::InventoryClient {
        resources::inventory::InventoryClient::new(self.clone())
    }

    /// Access the Returns API
    pub fn returns(&self) -> resources::returns::ReturnsClient {
        resources::returns::ReturnsClient::new(self.clone())
    }

    /// Access the Shipments API
    pub fn shipments(&self) -> resources::shipments::ShipmentsClient {
        resources::shipments::ShipmentsClient::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = Client::new("https://api.stateset.io").unwrap();
        assert!(!client.is_authenticated());

        let authenticated = client.authenticate(Credentials::bearer("test-token"));
        assert!(authenticated.is_authenticated());
    }
    
    #[test]
    fn test_url_building() {
        let client = Client::new("https://api.stateset.io").unwrap();
        let url = client.build_url("/orders").unwrap();
        assert_eq!(url.as_str(), "https://api.stateset.io/orders");
    }
    
    #[test]
    fn test_config_validation() {
        let config = Config::builder()
            .base_url("https://api.stateset.io")
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .retry_attempts(3)
            .build()
            .unwrap();
            
        assert_eq!(config.base_url.as_str(), "https://api.stateset.io/");
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert_eq!(config.retry_attempts, 3);
    }
} 