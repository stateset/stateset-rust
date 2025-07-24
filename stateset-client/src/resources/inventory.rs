//! Inventory API client implementation

use crate::Client;
use stateset_core::{Result, types::ResourceId};
use stateset_models::inventory::{
    CreateReservationRequest, InventoryLevel, InventoryReservation, InventoryUpdate,
    ReservationBuilder, ReservationStrategy,
};
use std::time::Duration;

/// Inventory API client
pub struct InventoryClient {
    client: Client,
}

impl InventoryClient {
    /// Create a new inventory client
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Get inventory levels for a specific item
    pub async fn get_levels(&self, item_id: impl Into<ResourceId>) -> Result<Vec<InventoryLevel>> {
        let path = format!("/api/v1/inventory/items/{}/levels", item_id.into());
        self.client.get(&path).await
    }

    /// Get inventory level at a specific location
    pub async fn get_level(
        &self,
        item_id: impl Into<ResourceId>,
        location_id: impl Into<ResourceId>,
    ) -> Result<InventoryLevel> {
        let path = format!(
            "/api/v1/inventory/items/{}/locations/{}/level",
            item_id.into(),
            location_id.into()
        );
        self.client.get(&path).await
    }

    /// Update inventory levels (adjustments)
    pub async fn adjust(&self, updates: Vec<InventoryUpdate>) -> Result<Vec<InventoryLevel>> {
        self.client.post("/api/v1/inventory/adjust", &updates).await
    }

    /// Update inventory in batch
    pub async fn update_batch(
        &self,
        updates: Vec<InventoryUpdate>,
    ) -> Result<Vec<InventoryLevel>> {
        self.adjust(updates).await
    }

    /// Create an inventory reservation
    pub async fn create_reservation(
        &self,
        request: CreateReservationRequest,
    ) -> Result<InventoryReservation> {
        self.client
            .post("/api/v1/inventory/reservations", &request)
            .await
    }

    /// Get a reservation by ID
    pub async fn get_reservation(
        &self,
        id: impl Into<ResourceId>,
    ) -> Result<InventoryReservation> {
        let path = format!("/api/v1/inventory/reservations/{}", id.into());
        self.client.get(&path).await
    }

    /// Cancel a reservation
    pub async fn cancel_reservation(
        &self,
        id: impl Into<ResourceId>,
    ) -> Result<InventoryReservation> {
        let path = format!("/api/v1/inventory/reservations/{}/cancel", id.into());
        self.client
            .post::<InventoryReservation, _>(&path, &serde_json::json!({}))
            .await
    }

    /// Start building a reservation
    pub fn reserve(&self) -> InventoryReservationBuilder {
        InventoryReservationBuilder::new(self.client.clone())
    }
}

/// Builder for creating inventory reservations
pub struct InventoryReservationBuilder {
    client: Client,
    builder: ReservationBuilder,
}

impl InventoryReservationBuilder {
    fn new(client: Client) -> Self {
        Self {
            client,
            builder: ReservationBuilder::new(),
        }
    }

    /// Set the warehouse
    pub fn warehouse(mut self, warehouse_id: impl Into<String>) -> Self {
        self.builder = self.builder.warehouse(warehouse_id);
        self
    }

    /// Set the reference (e.g., order ID)
    pub fn reference(
        mut self,
        id: impl Into<ResourceId>,
        ref_type: stateset_core::types::ReferenceType,
    ) -> Self {
        self.builder = self.builder.reference(id.into(), ref_type);
        self
    }

    /// Add an item to reserve
    pub fn item(mut self, item_id: impl Into<ResourceId>, quantity: u32) -> Self {
        self.builder = self.builder.item(item_id, quantity);
        self
    }

    /// Add items from an order
    pub fn items(mut self, items: &[stateset_models::order::OrderItem]) -> Self {
        self.builder = self.builder.items(items);
        self
    }

    /// Set the reservation strategy
    pub fn strategy(mut self, strategy: ReservationStrategy) -> Self {
        self.builder = self.builder.strategy(strategy);
        self
    }

    /// Set the reservation duration
    pub fn duration(mut self, duration: Duration) -> Self {
        self.builder = self.builder.duration(duration);
        self
    }

    /// Set the priority
    pub fn priority(mut self, priority: u8) -> Self {
        self.builder = self.builder.priority(priority);
        self
    }

    /// Execute the reservation
    pub async fn execute(self) -> Result<InventoryReservation> {
        let request = self.builder.build();
        self.client
            .post("/api/v1/inventory/reservations", &request)
            .await
    }
} 