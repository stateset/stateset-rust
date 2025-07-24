//! Checkouts API client implementation

use crate::{Client, request::{ListRequestBuilder, SortOrder}};
use stateset_core::{Error, Result, traits::ListResponse, types::ResourceId};
use stateset_models::checkout::{
    CreateCheckoutRequest, Checkout, CheckoutListFilters, CheckoutStatus, CheckoutStep,
    UpdateCheckoutRequest, CompleteCheckoutRequest, CheckoutCompletionResult,
    ApplyCheckoutCouponRequest, ApplyGiftCardRequest, CheckoutAbandonmentAnalysis,
};

/// Checkouts API client
pub struct CheckoutsClient {
    client: Client,
}

impl CheckoutsClient {
    /// Create a new checkouts client
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a new checkout
    pub async fn create(&self, request: CreateCheckoutRequest) -> Result<Checkout> {
        self.client.post("/api/v1/checkouts", &request).await
    }

    /// Get a checkout by ID
    pub async fn get(&self, id: impl Into<ResourceId>) -> Result<Checkout> {
        let path = format!("/api/v1/checkouts/{}", id.into());
        self.client.get(&path).await
    }

    /// Get a checkout by token
    pub async fn get_by_token(&self, token: &str) -> Result<Checkout> {
        let path = format!("/api/v1/checkouts/token/{}", token);
        self.client.get(&path).await
    }

    /// Update a checkout
    pub async fn update(&self, id: impl Into<ResourceId>, request: UpdateCheckoutRequest) -> Result<Checkout> {
        let path = format!("/api/v1/checkouts/{}", id.into());
        self.client.patch(&path, &request).await
    }

    /// Complete a checkout
    pub async fn complete(&self, id: impl Into<ResourceId>, request: CompleteCheckoutRequest) -> Result<CheckoutCompletionResult> {
        let path = format!("/api/v1/checkouts/{}/complete", id.into());
        self.client.post(&path, &request).await
    }

    /// Cancel a checkout
    pub async fn cancel(&self, id: impl Into<ResourceId>) -> Result<Checkout> {
        let path = format!("/api/v1/checkouts/{}/cancel", id.into());
        self.client.post::<Checkout, _>(&path, &serde_json::json!({})).await
    }

    /// Apply coupon to checkout
    pub async fn apply_coupon(&self, id: impl Into<ResourceId>, request: ApplyCheckoutCouponRequest) -> Result<Checkout> {
        let path = format!("/api/v1/checkouts/{}/coupons", id.into());
        self.client.post(&path, &request).await
    }

    /// Apply gift card to checkout
    pub async fn apply_gift_card(&self, id: impl Into<ResourceId>, request: ApplyGiftCardRequest) -> Result<Checkout> {
        let path = format!("/api/v1/checkouts/{}/gift-cards", id.into());
        self.client.post(&path, &request).await
    }

    /// Get shipping rates for checkout
    pub async fn get_shipping_rates(&self, id: impl Into<ResourceId>) -> Result<serde_json::Value> {
        let path = format!("/api/v1/checkouts/{}/shipping-rates", id.into());
        self.client.get(&path).await
    }

    /// Update checkout step
    pub async fn update_step(&self, id: impl Into<ResourceId>, step: CheckoutStep) -> Result<Checkout> {
        let path = format!("/api/v1/checkouts/{}/step", id.into());
        let body = serde_json::json!({"step": step});
        self.client.post(&path, &body).await
    }

    /// List checkouts
    pub fn list(&self) -> CheckoutListBuilder {
        CheckoutListBuilder::new(self.client.clone())
    }

    /// Get checkout abandonment analysis
    pub async fn abandonment_analysis(&self, date_range: Option<(String, String)>) -> Result<CheckoutAbandonmentAnalysis> {
        let mut path = "/api/v1/checkouts/abandonment-analysis".to_string();
        if let Some((start, end)) = date_range {
            path.push_str(&format!("?start_date={}&end_date={}", start, end));
        }
        self.client.get(&path).await
    }

    /// Get abandoned checkouts
    pub async fn abandoned(&self) -> Result<Vec<Checkout>> {
        self.client.get("/api/v1/checkouts/abandoned").await
    }
}

pub struct CheckoutListBuilder {
    client: Client,
    builder: ListRequestBuilder<CheckoutListFilters>,
}

impl CheckoutListBuilder {
    fn new(client: Client) -> Self {
        Self {
            client,
            builder: ListRequestBuilder::new(),
        }
    }

    pub fn status(mut self, status: CheckoutStatus) -> Self {
        self.builder.filters.status = Some(status);
        self
    }

    pub fn current_step(mut self, step: CheckoutStep) -> Self {
        self.builder.filters.current_step = Some(step);
        self
    }

    pub fn customer_id(mut self, customer_id: impl Into<ResourceId>) -> Self {
        self.builder.filters.customer_id = Some(customer_id.into());
        self
    }

    pub async fn execute(self) -> Result<ListResponse<Checkout>> {
        self.client
            .get_with_query("/api/v1/checkouts", &self.builder.build())
            .await
    }
}