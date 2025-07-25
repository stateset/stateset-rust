//! Warranties API client implementation

use crate::{Client, request::{ListRequestBuilder, SortOrder}};
use stateset_core::{Error, Result, ListResponse, types::ResourceId};
use stateset_models::warranty::{
    CreateWarrantyRequest, Warranty, WarrantyListFilters, WarrantyStatus, WarrantyType,
    UpdateWarrantyRequest, CreateWarrantyClaimRequest, WarrantyClaim, WarrantyClaimListFilters,
    UpdateWarrantyClaimRequest,
};
use futures::stream::{Stream, StreamExt};
use std::pin::Pin;

/// Warranties API client
pub struct WarrantiesClient {
    client: Client,
}

impl WarrantiesClient {
    /// Create a new warranties client
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a new warranty
    pub async fn create(&self, request: CreateWarrantyRequest) -> Result<Warranty> {
        self.client.post("/api/v1/warranties", &request).await
    }

    /// Get a warranty by ID
    pub async fn get(&self, id: impl Into<ResourceId>) -> Result<Warranty> {
        let path = format!("/api/v1/warranties/{}", id.into());
        self.client.get(&path).await
    }

    /// Update a warranty
    pub async fn update(
        &self,
        id: impl Into<ResourceId>,
        request: UpdateWarrantyRequest,
    ) -> Result<Warranty> {
        let path = format!("/api/v1/warranties/{}", id.into());
        self.client.patch(&path, &request).await
    }

    /// Delete a warranty
    pub async fn delete(&self, id: impl Into<ResourceId>) -> Result<()> {
        let path = format!("/api/v1/warranties/{}", id.into());
        self.client.delete_no_content(&path).await
    }

    /// Activate a warranty
    pub async fn activate(&self, id: impl Into<ResourceId>) -> Result<Warranty> {
        let path = format!("/api/v1/warranties/{}/activate", id.into());
        self.client.post::<Warranty, _>(&path, &serde_json::json!({})).await
    }

    /// Void a warranty
    pub async fn void(&self, id: impl Into<ResourceId>, reason: Option<String>) -> Result<Warranty> {
        let path = format!("/api/v1/warranties/{}/void", id.into());
        let body = if let Some(reason) = reason {
            serde_json::json!({"reason": reason})
        } else {
            serde_json::json!({})
        };
        self.client.post::<Warranty, _>(&path, &body).await
    }

    /// Transfer a warranty to another customer
    pub async fn transfer(&self, id: impl Into<ResourceId>, new_customer_id: impl Into<ResourceId>) -> Result<Warranty> {
        let path = format!("/api/v1/warranties/{}/transfer", id.into());
        let body = serde_json::json!({"new_customer_id": new_customer_id.into()});
        self.client.post::<Warranty, _>(&path, &body).await
    }

    /// Create a warranty claim
    pub async fn create_claim(&self, request: CreateWarrantyClaimRequest) -> Result<WarrantyClaim> {
        self.client.post("/api/v1/warranty-claims", &request).await
    }

    /// Get a warranty claim by ID
    pub async fn get_claim(&self, id: impl Into<ResourceId>) -> Result<WarrantyClaim> {
        let path = format!("/api/v1/warranty-claims/{}", id.into());
        self.client.get(&path).await
    }

    /// Update a warranty claim
    pub async fn update_claim(
        &self,
        id: impl Into<ResourceId>,
        request: UpdateWarrantyClaimRequest,
    ) -> Result<WarrantyClaim> {
        let path = format!("/api/v1/warranty-claims/{}", id.into());
        self.client.patch(&path, &request).await
    }

    /// Approve a warranty claim
    pub async fn approve_claim(&self, id: impl Into<ResourceId>, approved_amount: Option<serde_json::Value>) -> Result<WarrantyClaim> {
        let path = format!("/api/v1/warranty-claims/{}/approve", id.into());
        let body = if let Some(amount) = approved_amount {
            serde_json::json!({"approved_amount": amount})
        } else {
            serde_json::json!({})
        };
        self.client.post::<WarrantyClaim, _>(&path, &body).await
    }

    /// Deny a warranty claim
    pub async fn deny_claim(&self, id: impl Into<ResourceId>, reason: String) -> Result<WarrantyClaim> {
        let path = format!("/api/v1/warranty-claims/{}/deny", id.into());
        let body = serde_json::json!({"denial_reason": reason});
        self.client.post::<WarrantyClaim, _>(&path, &body).await
    }

    /// List warranties with a builder pattern
    pub fn list(&self) -> WarrantyListBuilder {
        WarrantyListBuilder::new(self.client.clone())
    }

    /// List warranty claims with a builder pattern
    pub fn list_claims(&self) -> WarrantyClaimListBuilder {
        WarrantyClaimListBuilder::new(self.client.clone())
    }

    /// Get warranties by customer
    pub async fn by_customer(&self, customer_id: impl Into<ResourceId>) -> Result<Vec<Warranty>> {
        let path = format!("/api/v1/warranties/by-customer/{}", customer_id.into());
        self.client.get(&path).await
    }

    /// Get warranties by product
    pub async fn by_product(&self, product_id: impl Into<ResourceId>) -> Result<Vec<Warranty>> {
        let path = format!("/api/v1/warranties/by-product/{}", product_id.into());
        self.client.get(&path).await
    }

    /// Get warranty analytics
    pub async fn analytics(&self, date_range: Option<(String, String)>) -> Result<serde_json::Value> {
        let mut path = "/api/v1/warranties/analytics".to_string();
        if let Some((start, end)) = date_range {
            path.push_str(&format!("?start_date={}&end_date={}", start, end));
        }
        self.client.get(&path).await
    }
}

/// Builder for listing warranties
pub struct WarrantyListBuilder {
    client: Client,
    builder: ListRequestBuilder<WarrantyListFilters>,
}

impl WarrantyListBuilder {
    fn new(client: Client) -> Self {
        Self {
            client,
            builder: ListRequestBuilder::new(),
        }
    }

    /// Filter by warranty status
    pub fn status(mut self, status: WarrantyStatus) -> Self {
        self.builder.filters_mut().status = Some(status);
        self
    }

    /// Filter by warranty type
    pub fn warranty_type(mut self, warranty_type: WarrantyType) -> Self {
        self.builder.filters_mut().warranty_type = Some(warranty_type);
        self
    }

    /// Filter by customer
    pub fn customer_id(mut self, customer_id: impl Into<ResourceId>) -> Self {
        self.builder.filters_mut().customer_id = Some(customer_id.into());
        self
    }

    /// Filter by product
    pub fn product_id(mut self, product_id: impl Into<ResourceId>) -> Self {
        self.builder.filters_mut().product_id = Some(product_id.into());
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.builder = self.builder.limit(limit);
        self
    }

    pub async fn execute(self) -> Result<ListResponse<Warranty>> {
        self.client
            .get_with_query("/api/v1/warranties", &self.builder.build())
            .await
    }
}

/// Builder for listing warranty claims
pub struct WarrantyClaimListBuilder {
    client: Client,
    builder: ListRequestBuilder<WarrantyClaimListFilters>,
}

impl WarrantyClaimListBuilder {
    fn new(client: Client) -> Self {
        Self {
            client,
            builder: ListRequestBuilder::new(),
        }
    }

    /// Filter by warranty ID (useful for related lookups)
    pub fn warranty_id(mut self, warranty_id: impl Into<ResourceId>) -> Self {
        self.builder.filters_mut().warranty_id = Some(warranty_id.into());
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.builder = self.builder.limit(limit);
        self
    }

    pub async fn execute(self) -> Result<ListResponse<WarrantyClaim>> {
        self.client
            .get_with_query("/api/v1/warranty-claims", &self.builder.build())
            .await
    }
}