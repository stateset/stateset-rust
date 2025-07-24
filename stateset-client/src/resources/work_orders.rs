//! Work Orders API client implementation

use crate::{Client, request::{ListRequestBuilder, SortOrder}};
use stateset_core::{Error, Result, traits::ListResponse, types::ResourceId};
use stateset_models::work_order::{
    CreateWorkOrderRequest, WorkOrder, WorkOrderListFilters, WorkOrderStatus, WorkOrderPriority, 
    WorkOrderType, UpdateWorkOrderRequest,
};
use futures::stream::{Stream, StreamExt};
use std::pin::Pin;

/// Work Orders API client
pub struct WorkOrdersClient {
    client: Client,
}

impl WorkOrdersClient {
    /// Create a new work orders client
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a new work order
    pub async fn create(&self, request: CreateWorkOrderRequest) -> Result<WorkOrder> {
        self.client.post("/api/v1/work-orders", &request).await
    }

    /// Get a work order by ID
    pub async fn get(&self, id: impl Into<ResourceId>) -> Result<WorkOrder> {
        let path = format!("/api/v1/work-orders/{}", id.into());
        self.client.get(&path).await
    }

    /// Update a work order
    pub async fn update(
        &self,
        id: impl Into<ResourceId>,
        request: UpdateWorkOrderRequest,
    ) -> Result<WorkOrder> {
        let path = format!("/api/v1/work-orders/{}", id.into());
        self.client.patch(&path, &request).await
    }

    /// Delete a work order
    pub async fn delete(&self, id: impl Into<ResourceId>) -> Result<()> {
        let path = format!("/api/v1/work-orders/{}", id.into());
        self.client.delete_no_content(&path).await
    }

    /// Start a work order
    pub async fn start(&self, id: impl Into<ResourceId>) -> Result<WorkOrder> {
        let path = format!("/api/v1/work-orders/{}/start", id.into());
        self.client.post::<WorkOrder, _>(&path, &serde_json::json!({})).await
    }

    /// Complete a work order
    pub async fn complete(&self, id: impl Into<ResourceId>, completion_notes: Option<String>) -> Result<WorkOrder> {
        let path = format!("/api/v1/work-orders/{}/complete", id.into());
        let body = if let Some(notes) = completion_notes {
            serde_json::json!({"completion_notes": notes})
        } else {
            serde_json::json!({})
        };
        self.client.post::<WorkOrder, _>(&path, &body).await
    }

    /// Cancel a work order
    pub async fn cancel(&self, id: impl Into<ResourceId>) -> Result<WorkOrder> {
        let path = format!("/api/v1/work-orders/{}/cancel", id.into());
        self.client.post::<WorkOrder, _>(&path, &serde_json::json!({})).await
    }

    /// Put a work order on hold
    pub async fn hold(&self, id: impl Into<ResourceId>, reason: Option<String>) -> Result<WorkOrder> {
        let path = format!("/api/v1/work-orders/{}/hold", id.into());
        let body = if let Some(reason) = reason {
            serde_json::json!({"reason": reason})
        } else {
            serde_json::json!({})
        };
        self.client.post::<WorkOrder, _>(&path, &body).await
    }

    /// Resume a work order from hold
    pub async fn resume(&self, id: impl Into<ResourceId>) -> Result<WorkOrder> {
        let path = format!("/api/v1/work-orders/{}/resume", id.into());
        self.client.post::<WorkOrder, _>(&path, &serde_json::json!({})).await
    }

    /// Assign a work order to a user
    pub async fn assign(&self, id: impl Into<ResourceId>, user_id: impl Into<ResourceId>) -> Result<WorkOrder> {
        let path = format!("/api/v1/work-orders/{}/assign", id.into());
        let body = serde_json::json!({"assigned_to": user_id.into()});
        self.client.post::<WorkOrder, _>(&path, &body).await
    }

    /// List work orders with a builder pattern
    pub fn list(&self) -> WorkOrderListBuilder {
        WorkOrderListBuilder::new(self.client.clone())
    }

    /// Create multiple work orders in batch
    pub async fn create_batch(&self, work_orders: Vec<CreateWorkOrderRequest>) -> Result<Vec<WorkOrder>> {
        self.client.post("/api/v1/work-orders/batch", &work_orders).await
    }

    /// Get work order analytics
    pub async fn analytics(&self, date_range: Option<(String, String)>) -> Result<serde_json::Value> {
        let mut path = "/api/v1/work-orders/analytics".to_string();
        if let Some((start, end)) = date_range {
            path.push_str(&format!("?start_date={}&end_date={}", start, end));
        }
        self.client.get(&path).await
    }

    /// Get work orders by asset
    pub async fn by_asset(&self, asset_id: impl Into<ResourceId>) -> Result<Vec<WorkOrder>> {
        let path = format!("/api/v1/work-orders/by-asset/{}", asset_id.into());
        self.client.get(&path).await
    }

    /// Get work orders by customer
    pub async fn by_customer(&self, customer_id: impl Into<ResourceId>) -> Result<Vec<WorkOrder>> {
        let path = format!("/api/v1/work-orders/by-customer/{}", customer_id.into());
        self.client.get(&path).await
    }

    /// Get work orders assigned to a user
    pub async fn by_assignee(&self, user_id: impl Into<ResourceId>) -> Result<Vec<WorkOrder>> {
        let path = format!("/api/v1/work-orders/by-assignee/{}", user_id.into());
        self.client.get(&path).await
    }
}

/// Builder for listing work orders with enhanced filtering and pagination
pub struct WorkOrderListBuilder {
    client: Client,
    builder: ListRequestBuilder<WorkOrderListFilters>,
}

impl WorkOrderListBuilder {
    fn new(client: Client) -> Self {
        Self {
            client,
            builder: ListRequestBuilder::new(),
        }
    }

    /// Filter by work order status
    pub fn status(mut self, status: WorkOrderStatus) -> Self {
        self.builder.filters.status = Some(status);
        self
    }

    /// Filter by work order priority
    pub fn priority(mut self, priority: WorkOrderPriority) -> Self {
        self.builder.filters.priority = Some(priority);
        self
    }

    /// Filter by work order type
    pub fn work_order_type(mut self, work_order_type: WorkOrderType) -> Self {
        self.builder.filters.work_order_type = Some(work_order_type);
        self
    }

    /// Filter by assigned user
    pub fn assigned_to(mut self, user_id: impl Into<ResourceId>) -> Self {
        self.builder.filters.assigned_to = Some(user_id.into());
        self
    }

    /// Filter by customer
    pub fn customer_id(mut self, customer_id: impl Into<ResourceId>) -> Self {
        self.builder.filters.customer_id = Some(customer_id.into());
        self
    }

    /// Filter by asset
    pub fn asset_id(mut self, asset_id: impl Into<ResourceId>) -> Self {
        self.builder.filters.asset_id = Some(asset_id.into());
        self
    }

    /// Filter by creation date range
    pub fn created_between(mut self, start: impl Into<String>, end: impl Into<String>) -> Self {
        use chrono::{DateTime, Utc};
        if let Ok(start_date) = start.into().parse::<DateTime<Utc>>() {
            self.builder.filters.created_after = Some(start_date.into());
        }
        if let Ok(end_date) = end.into().parse::<DateTime<Utc>>() {
            self.builder.filters.created_before = Some(end_date.into());
        }
        self
    }

    /// Filter by scheduled date range
    pub fn scheduled_between(mut self, start: impl Into<String>, end: impl Into<String>) -> Self {
        use chrono::{DateTime, Utc};
        if let Ok(start_date) = start.into().parse::<DateTime<Utc>>() {
            self.builder.filters.scheduled_after = Some(start_date.into());
        }
        if let Ok(end_date) = end.into().parse::<DateTime<Utc>>() {
            self.builder.filters.scheduled_before = Some(end_date.into());
        }
        self
    }

    /// Set the number of items per page
    pub fn limit(mut self, limit: u32) -> Self {
        self.builder = self.builder.limit(limit);
        self
    }

    /// Set the page offset
    pub fn offset(mut self, offset: u32) -> Self {
        self.builder = self.builder.offset(offset);
        self
    }

    /// Set the sort field and order
    pub fn sort_by(mut self, field: &str, order: SortOrder) -> Self {
        self.builder = self.builder.sort_by(field, order);
        self
    }

    /// Execute the query and return a paginated response
    pub async fn execute(self) -> Result<ListResponse<WorkOrder>> {
        self.client
            .get_with_query("/api/v1/work-orders", &self.builder.build())
            .await
    }

    /// Execute the query and return all results as a vector
    pub async fn all(self) -> Result<Vec<WorkOrder>> {
        let mut results = Vec::new();
        let mut builder = self;
        let mut offset = 0;
        const PAGE_SIZE: u32 = 100;

        loop {
            let response: ListResponse<WorkOrder> = builder
                .builder
                .clone()
                .limit(PAGE_SIZE)
                .offset(offset)
                .build()
                .execute(&builder.client, "/api/v1/work-orders")
                .await?;

            let page_size = response.data.len();
            results.extend(response.data);

            if page_size < PAGE_SIZE as usize || response.pagination.total <= offset + PAGE_SIZE {
                break;
            }

            offset += PAGE_SIZE;
        }

        Ok(results)
    }

    /// Return a stream of work orders
    pub fn stream(self) -> Pin<Box<dyn Stream<Item = Result<WorkOrder>> + Send>> {
        let query = self.builder.build();
        Box::pin(self.client.stream_with_query("/api/v1/work-orders", &query))
    }
}