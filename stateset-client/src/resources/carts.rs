//! Carts API client implementation

use crate::{Client, request::{ListRequestBuilder, SortOrder}};
use stateset_core::{Error, Result, traits::ListResponse, types::ResourceId};
use stateset_models::cart::{
    CreateCartRequest, Cart, CartListFilters, CartStatus, CartType, UpdateCartRequest,
    AddCartItemRequest, UpdateCartItemRequest, ApplyCouponRequest, CartAnalytics,
};

/// Carts API client
pub struct CartsClient {
    client: Client,
}

impl CartsClient {
    /// Create a new carts client
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a new cart
    pub async fn create(&self, request: CreateCartRequest) -> Result<Cart> {
        self.client.post("/api/v1/carts", &request).await
    }

    /// Get a cart by ID
    pub async fn get(&self, id: impl Into<ResourceId>) -> Result<Cart> {
        let path = format!("/api/v1/carts/{}", id.into());
        self.client.get(&path).await
    }

    /// Get a cart by token
    pub async fn get_by_token(&self, token: &str) -> Result<Cart> {
        let path = format!("/api/v1/carts/token/{}", token);
        self.client.get(&path).await
    }

    /// Update a cart
    pub async fn update(&self, id: impl Into<ResourceId>, request: UpdateCartRequest) -> Result<Cart> {
        let path = format!("/api/v1/carts/{}", id.into());
        self.client.patch(&path, &request).await
    }

    /// Delete a cart
    pub async fn delete(&self, id: impl Into<ResourceId>) -> Result<()> {
        let path = format!("/api/v1/carts/{}", id.into());
        self.client.delete_no_content(&path).await
    }

    /// Add item to cart
    pub async fn add_item(&self, cart_id: impl Into<ResourceId>, request: AddCartItemRequest) -> Result<Cart> {
        let path = format!("/api/v1/carts/{}/items", cart_id.into());
        self.client.post(&path, &request).await
    }

    /// Update cart item
    pub async fn update_item(&self, cart_id: impl Into<ResourceId>, item_id: impl Into<ResourceId>, request: UpdateCartItemRequest) -> Result<Cart> {
        let path = format!("/api/v1/carts/{}/items/{}", cart_id.into(), item_id.into());
        self.client.patch(&path, &request).await
    }

    /// Remove item from cart
    pub async fn remove_item(&self, cart_id: impl Into<ResourceId>, item_id: impl Into<ResourceId>) -> Result<Cart> {
        let path = format!("/api/v1/carts/{}/items/{}", cart_id.into(), item_id.into());
        self.client.delete(&path).await
    }

    /// Clear cart
    pub async fn clear(&self, cart_id: impl Into<ResourceId>) -> Result<Cart> {
        let path = format!("/api/v1/carts/{}/clear", cart_id.into());
        self.client.post::<Cart, _>(&path, &serde_json::json!({})).await
    }

    /// Apply coupon to cart
    pub async fn apply_coupon(&self, cart_id: impl Into<ResourceId>, request: ApplyCouponRequest) -> Result<Cart> {
        let path = format!("/api/v1/carts/{}/coupons", cart_id.into());
        self.client.post(&path, &request).await
    }

    /// Remove coupon from cart
    pub async fn remove_coupon(&self, cart_id: impl Into<ResourceId>, coupon_code: &str) -> Result<Cart> {
        let path = format!("/api/v1/carts/{}/coupons/{}", cart_id.into(), coupon_code);
        self.client.delete(&path).await
    }

    /// Convert cart to order
    pub async fn convert_to_order(&self, cart_id: impl Into<ResourceId>) -> Result<serde_json::Value> {
        let path = format!("/api/v1/carts/{}/convert", cart_id.into());
        self.client.post::<serde_json::Value, _>(&path, &serde_json::json!({})).await
    }

    /// List carts
    pub fn list(&self) -> CartListBuilder {
        CartListBuilder::new(self.client.clone())
    }

    /// Get cart analytics
    pub async fn analytics(&self, date_range: Option<(String, String)>) -> Result<CartAnalytics> {
        let mut path = "/api/v1/carts/analytics".to_string();
        if let Some((start, end)) = date_range {
            path.push_str(&format!("?start_date={}&end_date={}", start, end));
        }
        self.client.get(&path).await
    }

    /// Get abandoned carts
    pub async fn abandoned(&self) -> Result<Vec<Cart>> {
        self.client.get("/api/v1/carts/abandoned").await
    }
}

pub struct CartListBuilder {
    client: Client,
    builder: ListRequestBuilder<CartListFilters>,
}

impl CartListBuilder {
    fn new(client: Client) -> Self {
        Self {
            client,
            builder: ListRequestBuilder::new(),
        }
    }

    pub fn status(mut self, status: CartStatus) -> Self {
        self.builder.filters.status = Some(status);
        self
    }

    pub fn cart_type(mut self, cart_type: CartType) -> Self {
        self.builder.filters.cart_type = Some(cart_type);
        self
    }

    pub fn customer_id(mut self, customer_id: impl Into<ResourceId>) -> Self {
        self.builder.filters.customer_id = Some(customer_id.into());
        self
    }

    pub async fn execute(self) -> Result<ListResponse<Cart>> {
        self.client
            .get_with_query("/api/v1/carts", &self.builder.build())
            .await
    }
}