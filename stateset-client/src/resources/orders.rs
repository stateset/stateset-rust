//! Orders API client implementation

use crate::{Client, request::ListRequestBuilder};
use stateset_core::{Error, Result, traits::ListResponse, types::ResourceId};
use stateset_models::order::{
    CreateOrderRequest, Order, OrderListFilters, OrderStatus, UpdateOrderRequest,
};

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

    /// List orders
    pub fn list(&self) -> OrderListBuilder {
        OrderListBuilder::new(self.client.clone())
    }

    /// Create multiple orders in batch
    pub async fn create_batch(&self, orders: Vec<CreateOrderRequest>) -> Result<Vec<Order>> {
        self.client.post("/api/v1/orders/batch", &orders).await
    }
}

/// Builder for listing orders
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
        self.builder = self.builder.with_filters(OrderListFilters {
            status: Some(status),
            ..Default::default()
        });
        self
    }

    /// Filter by customer ID
    pub fn customer(mut self, customer_id: impl Into<ResourceId>) -> Self {
        self.builder = self.builder.with_filters(OrderListFilters {
            customer_id: Some(customer_id.into()),
            ..Default::default()
        });
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

    /// Execute the request
    pub async fn execute(&self) -> Result<ListResponse<Order>> {
        let (options, filters) = self.builder.clone().build();
        
        let mut query_params = serde_json::to_value(options.build())
            .map_err(|e| Error::Serialization(e))?;
        
        let filter_value = serde_json::to_value(&filters)
            .map_err(|e| Error::Serialization(e))?;
        
        if let Some(obj) = query_params.as_object_mut() {
            if let Some(filter_obj) = filter_value.as_object() {
                for (k, v) in filter_obj {
                    obj.insert(k.clone(), v.clone());
                }
            }
        }

        self.client
            .get_with_query("/api/v1/orders", &query_params)
            .await
    }

    /// Stream all pages of results
    pub fn stream(self) -> OrderStream {
        OrderStream::new(self)
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

/// Stream for paginating through orders
#[allow(dead_code)]
pub struct OrderStream {
    builder: OrderListBuilder,
    current_page: Option<ListResponse<Order>>,
    current_index: usize,
    cursor: Option<String>,
}

impl OrderStream {
    fn new(builder: OrderListBuilder) -> Self {
        Self {
            builder,
            current_page: None,
            current_index: 0,
            cursor: None,
        }
    }
}

use futures::stream::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

impl Stream for OrderStream {
    type Item = Result<Order>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // Implementation would go here
        // This is a simplified version - in production you'd implement proper async streaming
        Poll::Ready(None)
    }
}

/// Builder for updating orders
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

    /// Execute the update
    pub async fn execute(self) -> Result<Order> {
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