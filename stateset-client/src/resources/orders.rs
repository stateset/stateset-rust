//! Orders API client implementation

use crate::{Client, request::{ListRequestBuilder, SortOrder}};
use stateset_core::{Error, Result, ListResponse, types::{ResourceId, Timestamp}};
use stateset_models::order::{
    CreateOrderRequest, Order, OrderListFilters, OrderStatus, UpdateOrderRequest,
};
use futures::stream::{Stream, StreamExt};
use std::pin::Pin;

/// Orders API client
pub struct OrdersClient {
    client: Client,
}

impl OrdersClient {
    /// Create a new orders client
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a new order
    pub async fn create(&self, request: CreateOrderRequest) -> Result<Order> {
        self.client.post("/api/v1/orders", &request).await
    }

    /// Get an order by ID
    pub async fn get(&self, id: impl Into<ResourceId>) -> Result<Order> {
        let path = format!("/api/v1/orders/{}", id.into());
        self.client.get(&path).await
    }

    /// Update an order
    pub async fn update(
        &self,
        id: impl Into<ResourceId>,
        request: UpdateOrderRequest,
    ) -> Result<Order> {
        let path = format!("/api/v1/orders/{}", id.into());
        self.client.patch(&path, &request).await
    }

    /// Delete an order
    pub async fn delete(&self, id: impl Into<ResourceId>) -> Result<()> {
        let path = format!("/api/v1/orders/{}", id.into());
        self.client.delete_no_content(&path).await
    }

    /// Cancel an order
    pub async fn cancel(&self, id: impl Into<ResourceId>) -> Result<Order> {
        let path = format!("/api/v1/orders/{}/cancel", id.into());
        self.client.post::<Order, _>(&path, &serde_json::json!({})).await
    }

    /// Fulfill an order
    pub async fn fulfill(&self, id: impl Into<ResourceId>) -> Result<Order> {
        let path = format!("/api/v1/orders/{}/fulfill", id.into());
        self.client.post::<Order, _>(&path, &serde_json::json!({})).await
    }

    /// Refund an order
    pub async fn refund(&self, id: impl Into<ResourceId>, amount: Option<f64>) -> Result<Order> {
        let path = format!("/api/v1/orders/{}/refund", id.into());
        let body = if let Some(amount) = amount {
            serde_json::json!({"amount": amount})
        } else {
            serde_json::json!({})
        };
        self.client.post::<Order, _>(&path, &body).await
    }

    /// List orders with a builder pattern
    pub fn list(&self) -> OrderListBuilder {
        OrderListBuilder::new(self.client.clone())
    }

    /// Create multiple orders in batch
    pub async fn create_batch(&self, orders: Vec<CreateOrderRequest>) -> Result<Vec<Order>> {
        self.client.post("/api/v1/orders/batch", &orders).await
    }

    /// Get order analytics
    pub async fn analytics(&self, date_range: Option<(String, String)>) -> Result<serde_json::Value> {
        let mut path = "/api/v1/orders/analytics".to_string();
        if let Some((start, end)) = date_range {
            path.push_str(&format!("?start_date={}&end_date={}", start, end));
        }
        self.client.get(&path).await
    }
}

/// Builder for listing orders with enhanced filtering and pagination
pub struct OrderListBuilder {
    client: Client,
    builder: ListRequestBuilder<OrderListFilters>,
}

impl OrderListBuilder {
    fn new(client: Client) -> Self {
        Self {
            client,
            builder: ListRequestBuilder::new(),
        }
    }

    /// Filter by order status
    pub fn status(mut self, status: OrderStatus) -> Self {
        self.builder.filters_mut().status = Some(status);
        self
    }

    /// Filter by customer ID
    pub fn customer(mut self, customer_id: impl Into<ResourceId>) -> Self {
        self.builder.filters_mut().customer_id = Some(customer_id.into());
        self
    }

    /// Filter by date range
    pub fn date_range(mut self, start: Timestamp, end: Timestamp) -> Self {
        let filters = self.builder.filters_mut();
        filters.created_after = Some(start);
        filters.created_before = Some(end);
        self
    }

    /// Filter by minimum total amount (in cents)
    pub fn min_total(mut self, amount: i64) -> Self {
        self.builder.filters_mut().min_total = Some(amount);
        self
    }

    /// Filter by maximum total amount (in cents)
    pub fn max_total(mut self, amount: i64) -> Self {
        self.builder.filters_mut().max_total = Some(amount);
        self
    }

    /// Set the page limit
    pub fn limit(mut self, limit: u32) -> Self {
        self.builder = self.builder.limit(limit);
        self
    }

    /// Set the page number
    pub fn page(mut self, page: u32) -> Self {
        self.builder = self.builder.page(page);
        self
    }

    /// Set the cursor for cursor-based pagination
    pub fn cursor(mut self, cursor: String) -> Self {
        self.builder = self.builder.cursor(cursor);
        self
    }

    /// Sort by field
    pub fn sort_by_created_at(mut self, order: SortOrder) -> Self {
        self.builder = self.builder.sort("created_at", order);
        self
    }

    /// Sort by total amount
    pub fn sort_by_total(mut self, order: SortOrder) -> Self {
        self.builder = self.builder.sort("total", order);
        self
    }

    /// Sort by updated date
    pub fn sort_by_updated_at(mut self, order: SortOrder) -> Self {
        self.builder = self.builder.sort("updated_at", order);
        self
    }

    /// Execute the request and return a single page
    pub async fn execute(&self) -> Result<ListResponse<Order>> {
        let (options, filters) = self.builder.clone().build()?;
        
        let mut query_params = options.to_query_params();
        
        // Add filter parameters
        if let Some(status) = &filters.status {
            query_params.insert("status".to_string(), status.to_string());
        }
        if let Some(customer_id) = &filters.customer_id {
            query_params.insert("customer_id".to_string(), customer_id.to_string());
        }
        if let Some(created_after) = &filters.created_after {
            query_params.insert("created_after".to_string(), created_after.to_string());
        }
        if let Some(created_before) = &filters.created_before {
            query_params.insert("created_before".to_string(), created_before.to_string());
        }
        if let Some(min_total) = filters.min_total {
            query_params.insert("min_total".to_string(), min_total.to_string());
        }
        if let Some(max_total) = filters.max_total {
            query_params.insert("max_total".to_string(), max_total.to_string());
        }

        self.client
            .get_with_query("/api/v1/orders", &query_params)
            .await
    }

    /// Stream all pages of results with enhanced error handling
    pub fn stream(self) -> Pin<Box<dyn Stream<Item = Result<Order>> + Send>> {
        let (options, filters) = match self.builder.clone().build() {
            Ok(result) => result,
            Err(e) => {
                return Box::pin(futures::stream::once(async move { Err(e) }));
            }
        };

        let mut query_params = options.to_query_params();
        
        // Add filter parameters
        if let Some(status) = &filters.status {
            query_params.insert("status".to_string(), status.to_string());
        }
        if let Some(customer_id) = &filters.customer_id {
            query_params.insert("customer_id".to_string(), customer_id.to_string());
        }
        if let Some(created_after) = &filters.created_after {
            query_params.insert("created_after".to_string(), created_after.to_string());
        }
        if let Some(created_before) = &filters.created_before {
            query_params.insert("created_before".to_string(), created_before.to_string());
        }
        if let Some(min_total) = filters.min_total {
            query_params.insert("min_total".to_string(), min_total.to_string());
        }
        if let Some(max_total) = filters.max_total {
            query_params.insert("max_total".to_string(), max_total.to_string());
        }

        Box::pin(self.client.stream_with_query("/api/v1/orders", &query_params))
    }

    /// Collect all results into a vector (use with caution for large datasets)
    pub async fn collect_all(self) -> Result<Vec<Order>> {
        let mut results = Vec::new();
        let mut stream = self.stream();
        
        while let Some(item) = stream.next().await {
            results.push(item?);
        }
        
        Ok(results)
    }

    /// Count total results without fetching all data
    pub async fn count(&self) -> Result<u64> {
        let (options, filters) = self.builder.clone().build()?;
        
        let mut query_params = options.to_query_params();
        query_params.insert("count_only".to_string(), "true".to_string());
        
        // Add filter parameters
        if let Some(status) = &filters.status {
            query_params.insert("status".to_string(), status.to_string());
        }
        if let Some(customer_id) = &filters.customer_id {
            query_params.insert("customer_id".to_string(), customer_id.to_string());
        }
        
        let response: serde_json::Value = self.client
            .get_with_query("/api/v1/orders", &query_params)
            .await?;
            
        response.get("count")
            .and_then(|c| c.as_u64())
            .ok_or_else(|| Error::network("Invalid count response"))
    }
}

impl Clone for OrderListBuilder {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            builder: self.builder.clone(),
        }
    }
}

/// Builder for updating orders with enhanced validation
pub struct OrderUpdateBuilder {
    client: Client,
    id: ResourceId,
    request: UpdateOrderRequest,
}

impl OrderUpdateBuilder {
    /// Create a new update builder
    pub fn new(client: Client, id: impl Into<ResourceId>) -> Self {
        Self {
            client,
            id: id.into(),
            request: UpdateOrderRequest::default(),
        }
    }

    /// Update the order status
    pub fn status(mut self, status: OrderStatus) -> Self {
        self.request.status = Some(status);
        self
    }

    /// Update the tracking number
    pub fn tracking_number(mut self, tracking: impl Into<String>) -> Self {
        self.request.tracking_number = Some(tracking.into());
        self
    }

    /// Update notes
    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.request.notes = Some(notes.into());
        self
    }

    /// Update shipping address
    pub fn shipping_address(mut self, address: stateset_core::types::Address) -> Self {
        self.request.shipping_address = Some(address);
        self
    }

    /// Validate the update request
    pub fn validate(&self) -> Result<()> {
        // Add custom validation logic here
        if let Some(tracking) = &self.request.tracking_number {
            if tracking.is_empty() {
                return Err(Error::validation_field(
                    "Tracking number cannot be empty",
                    "tracking_number"
                ));
            }
        }
        Ok(())
    }

    /// Execute the update with validation
    pub async fn execute(self) -> Result<Order> {
        self.validate()?;
        let path = format!("/api/v1/orders/{}", self.id);
        self.client.patch(&path, &self.request).await
    }
}

// Extension trait for fluent updates
impl OrdersClient {
    /// Start building an update request
    pub fn update_builder(&self, id: impl Into<ResourceId>) -> OrderUpdateBuilder {
        OrderUpdateBuilder::new(self.client.clone(), id)
    }
} 