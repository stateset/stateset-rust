//! HTTP client implementation for StateSet SDK

use reqwest::{Client as ReqwestClient, Method, RequestBuilder, Response};
use serde::{de::DeserializeOwned, Serialize};
use stateset_auth::Credentials;
use stateset_core::{Config, Error, Result};
use std::sync::Arc;
use url::Url;

pub mod request;
pub mod resources;



/// StateSet HTTP client
#[derive(Clone)]
pub struct Client {
    inner: Arc<ClientInner>,
}

struct ClientInner {
    http: ReqwestClient,
    config: Config,
    credentials: Option<Credentials>,
}

impl Client {
    /// Create a new client with the default configuration
    pub fn new(base_url: impl AsRef<str>) -> Result<Self> {
        let config = Config::with_base_url(base_url)?;
        Self::with_config(config)
    }

    /// Create a new client with a custom configuration
    pub fn with_config(config: Config) -> Result<Self> {
        let http = ReqwestClient::builder()
            .timeout(config.timeout)
            .user_agent(&config.user_agent)
            .build()
            .map_err(|e| Error::Configuration(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            inner: Arc::new(ClientInner {
                http,
                config,
                credentials: None,
            }),
        })
    }

    /// Authenticate the client with credentials
    pub fn authenticate(&self, credentials: Credentials) -> Self {
        let inner = ClientInner {
            http: self.inner.http.clone(),
            config: self.inner.config.clone(),
            credentials: Some(credentials),
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
            .map_err(|e| Error::Configuration(format!("Invalid URL path: {}", e)))
    }

    /// Create a request builder
    fn request(&self, method: Method, path: &str) -> Result<RequestBuilder> {
        let url = self.build_url(path)?;
        let mut request = self.inner.http.request(method, url);

        // Add authentication header if available
        if let Some(credentials) = &self.inner.credentials {
            request = request.header("Authorization", credentials.authorization_header());
        }

        // Add common headers
        request = request
            .header("Accept", "application/json")
            .header("Content-Type", "application/json");

        Ok(request)
    }

    /// Execute a request and handle the response
    async fn execute<T: DeserializeOwned>(&self, request: RequestBuilder) -> Result<T> {
        let response = request
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    Error::Timeout
                } else {
                    Error::Network(e.to_string())
                }
            })?;

        self.handle_response(response).await
    }

    /// Handle the HTTP response
    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        let status = response.status();

        if status.is_success() {
            response
                .json::<T>()
                .await
                .map_err(|e| Error::Network(format!("Failed to parse JSON: {}", e)))
        } else {
            let status_code = status.as_u16();
            
            // Extract retry-after header before consuming response
            let retry_after = if status_code == 429 {
                response
                    .headers()
                    .get("Retry-After")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse::<u64>().ok())
                    .map(std::time::Duration::from_secs)
            } else {
                None
            };
            
            let error_body = response.text().await.unwrap_or_default();

            // Try to parse as JSON error response
            if let Ok(json_error) = serde_json::from_str::<serde_json::Value>(&error_body) {
                if let Some(message) = json_error.get("message").and_then(|v| v.as_str()) {
                    return Err(Error::api_with_details(status_code, message.to_string(), json_error));
                }
            }

            // Handle specific status codes
            match status_code {
                401 => Err(Error::Authentication {
                    message: "Unauthorized".to_string(),
                }),
                403 => Err(Error::Authorization {
                    message: "Forbidden".to_string(),
                }),
                404 => Err(Error::NotFound),
                429 => Err(Error::RateLimit { retry_after }),
                _ => Err(Error::api(status_code, error_body)),
            }
        }
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
            .map_err(|e| Error::Network(e.to_string()))?;

        if response.status().is_success() {
            Ok(())
        } else {
            self.handle_response::<serde_json::Value>(response)
                .await
                .map(|_| ())
        }
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
} 