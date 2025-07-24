//! Returns API client implementation

use crate::Client;
use stateset_core::{Result, types::ResourceId};
use stateset_models::returns::{CreateReturnRequest, Return, UpdateReturnRequest};

/// Returns API client
pub struct ReturnsClient {
    client: Client,
}

impl ReturnsClient {
    /// Create a new returns client
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a new return
    pub async fn create(&self, request: CreateReturnRequest) -> Result<Return> {
        self.client.post("/api/v1/returns", &request).await
    }

    /// Get a return by ID
    pub async fn get(&self, id: impl Into<ResourceId>) -> Result<Return> {
        let path = format!("/api/v1/returns/{}", id.into());
        self.client.get(&path).await
    }

    /// Update a return
    pub async fn update(
        &self,
        id: impl Into<ResourceId>,
        request: UpdateReturnRequest,
    ) -> Result<Return> {
        let path = format!("/api/v1/returns/{}", id.into());
        self.client.patch(&path, &request).await
    }

    /// Approve a return
    pub async fn approve(&self, id: impl Into<ResourceId>) -> Result<Return> {
        let path = format!("/api/v1/returns/{}/approve", id.into());
        self.client
            .post::<Return, _>(&path, &serde_json::json!({}))
            .await
    }

    /// Reject a return
    pub async fn reject(&self, id: impl Into<ResourceId>, reason: &str) -> Result<Return> {
        let path = format!("/api/v1/returns/{}/reject", id.into());
        self.client
            .post::<Return, _>(&path, &serde_json::json!({ "reason": reason }))
            .await
    }

    /// Mark a return as received
    pub async fn receive(&self, id: impl Into<ResourceId>) -> Result<Return> {
        let path = format!("/api/v1/returns/{}/receive", id.into());
        self.client
            .post::<Return, _>(&path, &serde_json::json!({}))
            .await
    }

    /// Process a return (issue refund/exchange)
    pub async fn process(&self, id: impl Into<ResourceId>) -> Result<Return> {
        let path = format!("/api/v1/returns/{}/process", id.into());
        self.client
            .post::<Return, _>(&path, &serde_json::json!({}))
            .await
    }

    /// List returns
    pub async fn list(&self) -> Result<Vec<Return>> {
        self.client.get("/api/v1/returns").await
    }

    /// List returns for a specific order
    pub async fn list_by_order(&self, order_id: impl Into<ResourceId>) -> Result<Vec<Return>> {
        let path = format!("/api/v1/orders/{}/returns", order_id.into());
        self.client.get(&path).await
    }

    /// List returns for a specific customer
    pub async fn list_by_customer(
        &self,
        customer_id: impl Into<ResourceId>,
    ) -> Result<Vec<Return>> {
        self.client
            .get_with_query(
                "/api/v1/returns",
                &serde_json::json!({ "customer_id": customer_id.into() }),
            )
            .await
    }
} 