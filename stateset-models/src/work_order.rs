//! Work order models and types

use serde::{Deserialize, Serialize};
use stateset_core::{
    traits::{ApiResource, Identifiable},
    types::{Address, Contact, Expandable, Metadata, Money, ResourceId, Timestamp},
};

/// Work order status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkOrderStatus {
    Draft,
    Open,
    InProgress,
    OnHold,
    Completed,
    Cancelled,
    Closed,
}

/// Work order priority enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkOrderPriority {
    Low,
    Normal,
    High,
    Urgent,
    Critical,
}

/// Work order type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkOrderType {
    Preventive,
    Corrective,
    Emergency,
    Inspection,
    Calibration,
    Installation,
    Repair,
    Maintenance,
}

/// Work order model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkOrder {
    pub id: ResourceId,
    pub work_order_number: String,
    pub title: String,
    pub description: Option<String>,
    pub status: WorkOrderStatus,
    pub priority: WorkOrderPriority,
    pub work_order_type: WorkOrderType,
    pub assigned_to: Option<ResourceId>,
    pub assignee: Option<Expandable<User>>,
    pub created_by: ResourceId,
    pub creator: Option<Expandable<User>>,
    pub customer_id: Option<ResourceId>,
    pub customer: Option<Expandable<Customer>>,
    pub asset_id: Option<ResourceId>,
    pub asset: Option<Expandable<Asset>>,
    pub location: Option<Address>,
    pub contact: Option<Contact>,
    pub estimated_hours: Option<f64>,
    pub actual_hours: Option<f64>,
    pub estimated_cost: Option<Money>,
    pub actual_cost: Option<Money>,
    pub parts: Vec<WorkOrderPart>,
    pub attachments: Vec<WorkOrderAttachment>,
    pub scheduled_start: Option<Timestamp>,
    pub scheduled_end: Option<Timestamp>,
    pub actual_start: Option<Timestamp>,
    pub actual_end: Option<Timestamp>,
    pub completion_notes: Option<String>,
    pub metadata: Option<Metadata>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl Identifiable for WorkOrder {
    type Id = ResourceId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl ApiResource for WorkOrder {
    const ENDPOINT: &'static str = "/api/v1/work-orders";
    const TYPE_NAME: &'static str = "work_order";
}

/// Work order part/material
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkOrderPart {
    pub id: ResourceId,
    pub part_id: ResourceId,
    pub part: Option<Expandable<Part>>,
    pub quantity_required: u32,
    pub quantity_used: Option<u32>,
    pub unit_cost: Option<Money>,
    pub total_cost: Option<Money>,
    pub metadata: Option<Metadata>,
}

/// Work order attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkOrderAttachment {
    pub id: ResourceId,
    pub filename: String,
    pub url: String,
    pub content_type: String,
    pub size: u64,
    pub uploaded_by: ResourceId,
    pub uploaded_at: Timestamp,
}

/// User model (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: ResourceId,
    pub email: String,
    pub name: String,
    pub role: String,
}

/// Asset model (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: ResourceId,
    pub name: String,
    pub asset_type: String,
    pub serial_number: Option<String>,
}

/// Part model (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Part {
    pub id: ResourceId,
    pub name: String,
    pub sku: String,
    pub unit_cost: Money,
}

/// Customer model (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: ResourceId,
    pub name: String,
    pub email: Option<String>,
}

/// Create work order request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWorkOrderRequest {
    pub title: String,
    pub description: Option<String>,
    pub priority: WorkOrderPriority,
    pub work_order_type: WorkOrderType,
    pub assigned_to: Option<ResourceId>,
    pub customer_id: Option<ResourceId>,
    pub asset_id: Option<ResourceId>,
    pub location: Option<Address>,
    pub contact: Option<Contact>,
    pub estimated_hours: Option<f64>,
    pub estimated_cost: Option<Money>,
    pub parts: Vec<CreateWorkOrderPart>,
    pub scheduled_start: Option<Timestamp>,
    pub scheduled_end: Option<Timestamp>,
    pub metadata: Option<Metadata>,
}

/// Create work order part
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWorkOrderPart {
    pub part_id: ResourceId,
    pub quantity_required: u32,
    pub unit_cost: Option<Money>,
    pub metadata: Option<Metadata>,
}

impl CreateWorkOrderRequest {
    /// Create a builder for the request
    pub fn builder() -> CreateWorkOrderRequestBuilder {
        CreateWorkOrderRequestBuilder::default()
    }
}

/// Builder for CreateWorkOrderRequest
#[derive(Default)]
pub struct CreateWorkOrderRequestBuilder {
    title: Option<String>,
    description: Option<String>,
    priority: Option<WorkOrderPriority>,
    work_order_type: Option<WorkOrderType>,
    assigned_to: Option<ResourceId>,
    customer_id: Option<ResourceId>,
    asset_id: Option<ResourceId>,
    location: Option<Address>,
    contact: Option<Contact>,
    estimated_hours: Option<f64>,
    estimated_cost: Option<Money>,
    parts: Vec<CreateWorkOrderPart>,
    scheduled_start: Option<Timestamp>,
    scheduled_end: Option<Timestamp>,
    metadata: Option<Metadata>,
}

impl CreateWorkOrderRequestBuilder {
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn priority(mut self, priority: WorkOrderPriority) -> Self {
        self.priority = Some(priority);
        self
    }

    pub fn work_order_type(mut self, work_order_type: WorkOrderType) -> Self {
        self.work_order_type = Some(work_order_type);
        self
    }

    pub fn assigned_to(mut self, user_id: impl Into<ResourceId>) -> Self {
        self.assigned_to = Some(user_id.into());
        self
    }

    pub fn customer_id(mut self, customer_id: impl Into<ResourceId>) -> Self {
        self.customer_id = Some(customer_id.into());
        self
    }

    pub fn asset_id(mut self, asset_id: impl Into<ResourceId>) -> Self {
        self.asset_id = Some(asset_id.into());
        self
    }

    pub fn location(mut self, location: Address) -> Self {
        self.location = Some(location);
        self
    }

    pub fn contact(mut self, contact: Contact) -> Self {
        self.contact = Some(contact);
        self
    }

    pub fn estimated_hours(mut self, hours: f64) -> Self {
        self.estimated_hours = Some(hours);
        self
    }

    pub fn estimated_cost(mut self, cost: Money) -> Self {
        self.estimated_cost = Some(cost);
        self
    }

    pub fn add_part(mut self, part_id: impl Into<ResourceId>, quantity: u32) -> Self {
        self.parts.push(CreateWorkOrderPart {
            part_id: part_id.into(),
            quantity_required: quantity,
            unit_cost: None,
            metadata: None,
        });
        self
    }

    pub fn scheduled_start(mut self, start: Timestamp) -> Self {
        self.scheduled_start = Some(start);
        self
    }

    pub fn scheduled_end(mut self, end: Timestamp) -> Self {
        self.scheduled_end = Some(end);
        self
    }

    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn build(self) -> Result<CreateWorkOrderRequest, String> {
        Ok(CreateWorkOrderRequest {
            title: self.title.ok_or("title is required")?,
            description: self.description,
            priority: self.priority.unwrap_or(WorkOrderPriority::Normal),
            work_order_type: self.work_order_type.ok_or("work_order_type is required")?,
            assigned_to: self.assigned_to,
            customer_id: self.customer_id,
            asset_id: self.asset_id,
            location: self.location,
            contact: self.contact,
            estimated_hours: self.estimated_hours,
            estimated_cost: self.estimated_cost,
            parts: self.parts,
            scheduled_start: self.scheduled_start,
            scheduled_end: self.scheduled_end,
            metadata: self.metadata,
        })
    }
}

/// Update work order request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateWorkOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<WorkOrderStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<WorkOrderPriority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<ResourceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_hours: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_hours: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_cost: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_cost: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_start: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_end: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_start: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_end: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// Work order list filters
#[derive(Debug, Clone, Default, Serialize)]
pub struct WorkOrderListFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<WorkOrderStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<WorkOrderPriority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_order_type: Option<WorkOrderType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<ResourceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<ResourceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<ResourceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_after: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_before: Option<Timestamp>,
}