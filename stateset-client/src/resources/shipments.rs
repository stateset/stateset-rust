//! Shipments API client implementation

use crate::Client;
use stateset_core::{Result, types::ResourceId};
use stateset_models::shipment::{
    CreateShipmentRequest, Shipment, UpdateShipmentRequest,
};

/// Shipments API client
pub struct ShipmentsClient {
    client: Client,
}

impl ShipmentsClient {
    /// Create a new shipments client
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a new shipment
    pub async fn create(&self, request: CreateShipmentRequest) -> Result<Shipment> {
        self.client.post("/api/v1/shipments", &request).await
    }

    /// Get a shipment by ID
    pub async fn get(&self, id: impl Into<ResourceId>) -> Result<Shipment> {
        let path = format!("/api/v1/shipments/{}", id.into());
        self.client.get(&path).await
    }

    /// Get a shipment by tracking number
    pub async fn get_by_tracking(&self, tracking_number: &str) -> Result<Shipment> {
        self.client
            .get_with_query(
                "/api/v1/shipments/tracking",
                &serde_json::json!({ "tracking_number": tracking_number }),
            )
            .await
    }

    /// Update a shipment
    pub async fn update(
        &self,
        id: impl Into<ResourceId>,
        request: UpdateShipmentRequest,
    ) -> Result<Shipment> {
        let path = format!("/api/v1/shipments/{}", id.into());
        self.client.patch(&path, &request).await
    }

    /// Cancel a shipment
    pub async fn cancel(&self, id: impl Into<ResourceId>) -> Result<Shipment> {
        let path = format!("/api/v1/shipments/{}/cancel", id.into());
        self.client
            .post::<Shipment, _>(&path, &serde_json::json!({}))
            .await
    }

    /// Mark a shipment as shipped
    pub async fn ship(&self, id: impl Into<ResourceId>) -> Result<Shipment> {
        let path = format!("/api/v1/shipments/{}/ship", id.into());
        self.client
            .post::<Shipment, _>(&path, &serde_json::json!({}))
            .await
    }

    /// Mark a shipment as delivered
    pub async fn deliver(&self, id: impl Into<ResourceId>) -> Result<Shipment> {
        let path = format!("/api/v1/shipments/{}/deliver", id.into());
        self.client
            .post::<Shipment, _>(&path, &serde_json::json!({}))
            .await
    }

    /// Get tracking events for a shipment
    pub async fn get_tracking_events(
        &self,
        id: impl Into<ResourceId>,
    ) -> Result<Vec<TrackingEvent>> {
        let path = format!("/api/v1/shipments/{}/tracking-events", id.into());
        self.client.get(&path).await
    }

    /// List shipments
    pub async fn list(&self) -> Result<Vec<Shipment>> {
        self.client.get("/api/v1/shipments").await
    }

    /// List shipments for a specific order
    pub async fn list_by_order(&self, order_id: impl Into<ResourceId>) -> Result<Vec<Shipment>> {
        let path = format!("/api/v1/orders/{}/shipments", order_id.into());
        self.client.get(&path).await
    }

    /// Create shipping labels
    pub async fn create_label(&self, id: impl Into<ResourceId>) -> Result<ShippingLabel> {
        let path = format!("/api/v1/shipments/{}/label", id.into());
        self.client.post::<ShippingLabel, _>(&path, &serde_json::json!({})).await
    }

    /// Get shipping rates
    pub async fn get_rates(&self, request: RateRequest) -> Result<Vec<ShippingRate>> {
        self.client.post("/api/v1/shipments/rates", &request).await
    }
}

/// Tracking event
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TrackingEvent {
    pub timestamp: stateset_core::types::Timestamp,
    pub status: String,
    pub location: Option<String>,
    pub description: String,
}

/// Shipping label
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShippingLabel {
    pub url: String,
    pub format: String,
    pub created_at: stateset_core::types::Timestamp,
}

/// Rate request
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RateRequest {
    pub from_address: stateset_core::types::Address,
    pub to_address: stateset_core::types::Address,
    pub weight: stateset_models::shipment::Weight,
    pub dimensions: Option<stateset_models::shipment::Dimensions>,
    pub carrier: Option<String>,
}

/// Shipping rate
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShippingRate {
    pub carrier: String,
    pub service: String,
    pub rate: stateset_core::types::Money,
    pub estimated_days: Option<u32>,
    pub description: String,
} 