//! Shipment-related models

use serde::{Deserialize, Serialize};
use stateset_core::{
    traits::{ApiResource, Identifiable},
    types::{Address, Metadata, Money, ResourceId, Timestamp},
};

/// Shipment status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShipmentStatus {
    Created,
    Pending,
    Ready,
    InTransit,
    OutForDelivery,
    Delivered,
    Failed,
    Returned,
    Cancelled,
}

/// Shipment model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shipment {
    pub id: ResourceId,
    pub shipment_number: String,
    pub order_id: ResourceId,
    pub status: ShipmentStatus,
    pub carrier: String,
    pub service_type: String,
    pub tracking_number: String,
    pub tracking_url: Option<String>,
    pub estimated_delivery: Option<Timestamp>,
    pub actual_delivery: Option<Timestamp>,
    pub from_address: Address,
    pub to_address: Address,
    pub weight: Option<Weight>,
    pub dimensions: Option<Dimensions>,
    pub insurance_amount: Option<Money>,
    pub shipping_cost: Money,
    pub items: Vec<ShipmentItem>,
    pub metadata: Option<Metadata>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl Identifiable for Shipment {
    type Id = ResourceId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl ApiResource for Shipment {
    const ENDPOINT: &'static str = "/api/v1/shipments";
    const TYPE_NAME: &'static str = "shipment";
}

/// Shipment item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipmentItem {
    pub order_item_id: ResourceId,
    pub product_id: ResourceId,
    pub sku: String,
    pub name: String,
    pub quantity: u32,
}

/// Weight measurement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weight {
    pub value: f64,
    pub unit: WeightUnit,
}

/// Weight unit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WeightUnit {
    Lb,
    Kg,
    Oz,
    G,
}

/// Package dimensions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimensions {
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub unit: DimensionUnit,
}

/// Dimension unit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DimensionUnit {
    In,
    Cm,
    Ft,
    M,
}

/// Create shipment request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateShipmentRequest {
    pub order_id: ResourceId,
    pub carrier: String,
    pub service_type: String,
    pub from_address: Address,
    pub to_address: Address,
    pub items: Vec<CreateShipmentItem>,
    pub weight: Option<Weight>,
    pub dimensions: Option<Dimensions>,
    pub insurance_amount: Option<Money>,
    pub metadata: Option<Metadata>,
}

/// Create shipment item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateShipmentItem {
    pub order_item_id: ResourceId,
    pub quantity: u32,
}

impl CreateShipmentRequest {
    /// Create a builder for the request
    pub fn builder() -> CreateShipmentRequestBuilder {
        CreateShipmentRequestBuilder::default()
    }
}

/// Builder for CreateShipmentRequest
#[derive(Default)]
pub struct CreateShipmentRequestBuilder {
    order_id: Option<ResourceId>,
    carrier: Option<String>,
    service_type: Option<String>,
    from_address: Option<Address>,
    to_address: Option<Address>,
    items: Vec<CreateShipmentItem>,
    weight: Option<Weight>,
    dimensions: Option<Dimensions>,
    insurance_amount: Option<Money>,
    metadata: Option<Metadata>,
}

impl CreateShipmentRequestBuilder {
    pub fn order_id(mut self, id: impl Into<ResourceId>) -> Self {
        self.order_id = Some(id.into());
        self
    }

    pub fn carrier(mut self, carrier: impl Into<String>) -> Self {
        self.carrier = Some(carrier.into());
        self
    }

    pub fn service_type(mut self, service_type: impl Into<String>) -> Self {
        self.service_type = Some(service_type.into());
        self
    }

    pub fn from_address(mut self, address: Address) -> Self {
        self.from_address = Some(address);
        self
    }

    pub fn to_address(mut self, address: Address) -> Self {
        self.to_address = Some(address);
        self
    }

    pub fn add_item(mut self, order_item_id: impl Into<ResourceId>, quantity: u32) -> Self {
        self.items.push(CreateShipmentItem {
            order_item_id: order_item_id.into(),
            quantity,
        });
        self
    }

    pub fn weight(mut self, weight: Weight) -> Self {
        self.weight = Some(weight);
        self
    }

    pub fn dimensions(mut self, dimensions: Dimensions) -> Self {
        self.dimensions = Some(dimensions);
        self
    }

    pub fn insurance_amount(mut self, amount: Money) -> Self {
        self.insurance_amount = Some(amount);
        self
    }

    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn build(self) -> CreateShipmentRequest {
        CreateShipmentRequest {
            order_id: self.order_id.expect("order_id is required"),
            carrier: self.carrier.expect("carrier is required"),
            service_type: self.service_type.expect("service_type is required"),
            from_address: self.from_address.expect("from_address is required"),
            to_address: self.to_address.expect("to_address is required"),
            items: self.items,
            weight: self.weight,
            dimensions: self.dimensions,
            insurance_amount: self.insurance_amount,
            metadata: self.metadata,
        }
    }
}

/// Update shipment request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateShipmentRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ShipmentStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_delivery: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_delivery: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
} 