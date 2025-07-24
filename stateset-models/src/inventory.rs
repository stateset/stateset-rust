//! Inventory-related models

use serde::{Deserialize, Serialize};
use stateset_core::{
    traits::{ApiResource, Identifiable},
    types::{Metadata, ResourceId, ReferenceType, Timestamp},
};
use std::time::Duration;

/// Inventory item model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub id: ResourceId,
    pub product_id: ResourceId,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub unit_of_measure: String,
    pub metadata: Option<Metadata>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl Identifiable for InventoryItem {
    type Id = ResourceId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl ApiResource for InventoryItem {
    const ENDPOINT: &'static str = "/api/v1/inventory/items";
    const TYPE_NAME: &'static str = "inventory_item";
}

/// Inventory level at a specific location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryLevel {
    pub id: ResourceId,
    pub item_id: ResourceId,
    pub location_id: ResourceId,
    pub warehouse_id: ResourceId,
    pub quantity_on_hand: i32,
    pub quantity_available: i32,
    pub quantity_reserved: i32,
    pub quantity_in_transit: i32,
    pub reorder_point: Option<i32>,
    pub reorder_quantity: Option<i32>,
    pub metadata: Option<Metadata>,
    pub last_counted_at: Option<Timestamp>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl ApiResource for InventoryLevel {
    const ENDPOINT: &'static str = "/api/v1/inventory/levels";
    const TYPE_NAME: &'static str = "inventory_level";
}

/// Inventory reservation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryReservation {
    pub id: ResourceId,
    pub reference: ReferenceType,
    pub items: Vec<ReservationItem>,
    pub status: ReservationStatus,
    pub warehouse_id: ResourceId,
    pub expires_at: Option<Timestamp>,
    pub metadata: Option<Metadata>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl Identifiable for InventoryReservation {
    type Id = ResourceId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl ApiResource for InventoryReservation {
    const ENDPOINT: &'static str = "/api/v1/inventory/reservations";
    const TYPE_NAME: &'static str = "inventory_reservation";
}

/// Reservation item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservationItem {
    pub item_id: ResourceId,
    pub quantity: u32,
    pub location_id: Option<ResourceId>,
}

/// Reservation status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReservationStatus {
    Pending,
    Confirmed,
    Partial,
    Fulfilled,
    Cancelled,
    Expired,
}

/// Reservation strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReservationStrategy {
    /// Reserve all items or fail
    All,
    /// Reserve as many items as possible
    Partial,
    /// Reserve from specific locations only
    LocationSpecific,
}

/// Inventory update/adjustment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryUpdate {
    pub item_id: ResourceId,
    pub location_id: ResourceId,
    pub adjustment: i32,
    pub reason: AdjustmentReason,
    pub reference: Option<ReferenceType>,
    pub notes: Option<String>,
    pub metadata: Option<Metadata>,
}

impl InventoryUpdate {
    /// Create an adjustment
    pub fn adjust(
        item_id: impl Into<ResourceId>,
        location_id: impl Into<ResourceId>,
        adjustment: i32,
    ) -> Self {
        Self {
            item_id: item_id.into(),
            location_id: location_id.into(),
            adjustment,
            reason: AdjustmentReason::Manual,
            reference: None,
            notes: None,
            metadata: None,
        }
    }
}

/// Adjustment reason
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdjustmentReason {
    Manual,
    CycleCount,
    Damage,
    Loss,
    Theft,
    Return,
    Transfer,
    Production,
    Other,
}

/// Create reservation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReservationRequest {
    pub reference: ReferenceType,
    pub items: Vec<ReservationItem>,
    pub warehouse_id: ResourceId,
    pub strategy: ReservationStrategy,
    pub duration: Option<Duration>,
    pub priority: Option<u8>,
    pub metadata: Option<Metadata>,
}

/// Reservation builder
pub struct ReservationBuilder {
    reference: Option<ReferenceType>,
    items: Vec<ReservationItem>,
    warehouse_id: Option<ResourceId>,
    strategy: ReservationStrategy,
    duration: Option<Duration>,
    priority: Option<u8>,
    metadata: Option<Metadata>,
}

impl ReservationBuilder {
    pub fn new() -> Self {
        Self {
            reference: None,
            items: Vec::new(),
            warehouse_id: None,
            strategy: ReservationStrategy::All,
            duration: None,
            priority: None,
            metadata: None,
        }
    }

    pub fn reference(mut self, _id: ResourceId, ref_type: ReferenceType) -> Self {
        self.reference = Some(ref_type);
        self
    }

    pub fn warehouse(mut self, id: impl Into<String>) -> Self {
        if let Ok(uuid) = id.into().parse::<uuid::Uuid>() {
            self.warehouse_id = Some(ResourceId::from(uuid));
        }
        self
    }

    pub fn item(mut self, item_id: impl Into<ResourceId>, quantity: u32) -> Self {
        self.items.push(ReservationItem {
            item_id: item_id.into(),
            quantity,
            location_id: None,
        });
        self
    }

    pub fn items(mut self, items: &[crate::order::OrderItem]) -> Self {
        for item in items {
            self.items.push(ReservationItem {
                item_id: item.product_id.clone(),
                quantity: item.quantity,
                location_id: None,
            });
        }
        self
    }

    pub fn strategy(mut self, strategy: ReservationStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn priority(mut self, priority: u8) -> Self {
        self.priority = Some(priority);
        self
    }

    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn build(self) -> CreateReservationRequest {
        CreateReservationRequest {
            reference: self.reference.expect("reference is required"),
            items: self.items,
            warehouse_id: self.warehouse_id.expect("warehouse_id is required"),
            strategy: self.strategy,
            duration: self.duration,
            priority: self.priority,
            metadata: self.metadata,
        }
    }
}

/// Inventory list filters
#[derive(Debug, Clone, Default, Serialize)]
pub struct InventoryListFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<ResourceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<ResourceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<ResourceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub below_reorder_point: Option<bool>,
} 