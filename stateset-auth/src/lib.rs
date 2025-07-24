//! Authentication handling for StateSet SDK

use serde::{Deserialize, Serialize};
use stateset_core::Error;

/// Authentication credentials
#[derive(Debug, Clone)]
pub enum Credentials {
    /// Bearer token authentication
    Bearer(String),
    /// API key authentication
    ApiKey(String),
    /// OAuth2 credentials
    OAuth2 {
        client_id: String,
        client_secret: String,
        redirect_uri: Option<String>,
    },
}

impl Credentials {
    /// Create bearer token credentials
    pub fn bearer(token: impl Into<String>) -> Self {
        Self::Bearer(token.into())
    }

    /// Create API key credentials
    pub fn api_key(key: impl Into<String>) -> Self {
        Self::ApiKey(key.into())
    }

    /// Create OAuth2 credentials
    pub fn oauth2(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        Self::OAuth2 {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            redirect_uri: None,
        }
    }

    /// Get the authorization header value
    pub fn authorization_header(&self) -> String {
        match self {
            Self::Bearer(token) => format!("Bearer {}", token),
            Self::ApiKey(key) => format!("ApiKey {}", key),
            Self::OAuth2 { .. } => {
                // OAuth2 flow would need to be completed first
                panic!("OAuth2 authentication requires token exchange")
            }
        }
    }

    /// Check if the credentials are for OAuth2
    pub fn is_oauth2(&self) -> bool {
        matches!(self, Self::OAuth2 { .. })
    }
}

/// OAuth2 token response
#[derive(Debug, Clone, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: Option<u64>,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
}

/// OAuth2 client for token exchange
pub struct OAuth2Client {
    client_id: String,
    client_secret: String,
    auth_url: String,
    token_url: String,
    redirect_uri: Option<String>,
}

impl OAuth2Client {
    /// Create a new OAuth2 client
    pub fn new(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        auth_url: impl Into<String>,
        token_url: impl Into<String>,
    ) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            auth_url: auth_url.into(),
            token_url: token_url.into(),
            redirect_uri: None,
        }
    }

    /// Set the redirect URI
    pub fn with_redirect_uri(mut self, uri: impl Into<String>) -> Self {
        self.redirect_uri = Some(uri.into());
        self
    }

    /// Generate the authorization URL
    pub fn authorize_url(&self, state: &str, scopes: &[&str]) -> String {
        let mut url = url::Url::parse(&self.auth_url).unwrap();
        {
            let mut query = url.query_pairs_mut();
            query.append_pair("client_id", &self.client_id);
            query.append_pair("response_type", "code");
            query.append_pair("state", state);
            
            if let Some(redirect_uri) = &self.redirect_uri {
                query.append_pair("redirect_uri", redirect_uri);
            }
            
            if !scopes.is_empty() {
                query.append_pair("scope", &scopes.join(" "));
            }
        }
        url.to_string()
    }

    /// Exchange authorization code for tokens
    pub async fn exchange_code(&self, code: &str) -> Result<TokenResponse, Error> {
        let client = reqwest::Client::new();
        
        let mut params = vec![
            ("grant_type", "authorization_code"),
            ("code", code),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
        ];

        if let Some(redirect_uri) = &self.redirect_uri {
            params.push(("redirect_uri", redirect_uri));
        }

        let response = client
            .post(&self.token_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| Error::Network(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let text = response.text().await.unwrap_or_default();
            return Err(Error::api(status, format!("Token exchange failed: {}", text)));
        }

        response
            .json::<TokenResponse>()
            .await
            .map_err(|e| Error::Network(e.to_string()))
    }

    /// Refresh an access token
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<TokenResponse, Error> {
        let client = reqwest::Client::new();
        
        let params = vec![
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
        ];

        let response = client
            .post(&self.token_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| Error::Network(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let text = response.text().await.unwrap_or_default();
            return Err(Error::api(status, format!("Token refresh failed: {}", text)));
        }

        response
            .json::<TokenResponse>()
            .await
            .map_err(|e| Error::Network(e.to_string()))
    }
}

/// JWT claims for token validation
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: Option<u64>,
    pub iat: Option<u64>,
    pub iss: Option<String>,
    pub aud: Option<String>,
    #[serde(flatten)]
    pub custom: serde_json::Value,
}

/// Token validator for JWT tokens
#[allow(dead_code)]
pub struct TokenValidator {
    secret: Option<String>,
    public_key: Option<String>,
}

impl TokenValidator {
    /// Create a validator with a shared secret
    pub fn with_secret(secret: impl Into<String>) -> Self {
        Self {
            secret: Some(secret.into()),
            public_key: None,
        }
    }

    /// Create a validator with a public key
    pub fn with_public_key(key: impl Into<String>) -> Self {
        Self {
            secret: None,
            public_key: Some(key.into()),
        }
    }

    /// Validate and decode a JWT token
    pub fn validate(&self, token: &str) -> Result<Claims, Error> {
        // For now, we'll use a simplified validation
        // In a real implementation, we'd use the jsonwebtoken crate
        
        // Split the token
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(Error::auth("Invalid token format"));
        }

        // Decode the payload
        use base64::{Engine as _, engine::general_purpose};
        let payload = general_purpose::URL_SAFE_NO_PAD
            .decode(parts[1])
            .map_err(|_| Error::auth("Failed to decode token payload"))?;
        
        let claims: Claims = serde_json::from_slice(&payload)
            .map_err(|_| Error::auth("Failed to parse token claims"))?;

        // Check expiration
        if let Some(exp) = claims.exp {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            if now > exp {
                return Err(Error::auth("Token has expired"));
            }
        }

        Ok(claims)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credentials() {
        let bearer = Credentials::bearer("test-token");
        assert_eq!(bearer.authorization_header(), "Bearer test-token");

        let api_key = Credentials::api_key("test-key");
        assert_eq!(api_key.authorization_header(), "ApiKey test-key");
    }

    #[test]
    fn test_oauth2_client() {
        let client = OAuth2Client::new(
            "client_id",
            "client_secret",
            "https://auth.example.com/authorize",
            "https://auth.example.com/token",
        )
        .with_redirect_uri("https://app.example.com/callback");

        let auth_url = client.authorize_url("state123", &["read", "write"]);
        assert!(auth_url.contains("client_id=client_id"));
        assert!(auth_url.contains("state=state123"));
        assert!(auth_url.contains("scope=read+write"));
    }
} 