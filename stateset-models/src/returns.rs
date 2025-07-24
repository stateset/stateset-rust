//! Return-related models

use serde::{Deserialize, Serialize};
use stateset_core::{
    traits::{ApiResource, Identifiable},
    types::{Metadata, Money, ResourceId, Timestamp},
};

/// Return status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReturnStatus {
    Requested,
    Approved,
    Rejected,
    InTransit,
    Received,
    Processing,
    Completed,
    Cancelled,
}

/// Return reason
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReturnReason {
    Defective,
    NotAsDescribed,
    WrongItem,
    Damaged,
    UnwantedItem,
    TooLate,
    Other,
}

/// Return model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Return {
    pub id: ResourceId,
    pub return_number: String,
    pub order_id: ResourceId,
    pub customer_id: ResourceId,
    pub status: ReturnStatus,
    pub reason: ReturnReason,
    pub items: Vec<ReturnItem>,
    pub refund_amount: Money,
    pub shipping_method: Option<String>,
    pub tracking_number: Option<String>,
    pub notes: Option<String>,
    pub metadata: Option<Metadata>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl Identifiable for Return {
    type Id = ResourceId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl ApiResource for Return {
    const ENDPOINT: &'static str = "/api/v1/returns";
    const TYPE_NAME: &'static str = "return";
}

/// Return item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnItem {
    pub id: ResourceId,
    pub order_item_id: ResourceId,
    pub product_id: ResourceId,
    pub sku: String,
    pub name: String,
    pub quantity: u32,
    pub condition: ItemCondition,
    pub reason: ReturnReason,
    pub notes: Option<String>,
}

/// Item condition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemCondition {
    New,
    OpenedUnused,
    UsedLikeNew,
    UsedGood,
    UsedFair,
    Damaged,
    Defective,
}

/// Create return request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReturnRequest {
    pub order_id: ResourceId,
    pub reason: ReturnReason,
    pub items: Vec<CreateReturnItem>,
    pub notes: Option<String>,
    pub metadata: Option<Metadata>,
}

/// Create return item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReturnItem {
    pub order_item_id: ResourceId,
    pub quantity: u32,
    pub condition: ItemCondition,
    pub reason: ReturnReason,
    pub notes: Option<String>,
}

/// Update return request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateReturnRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ReturnStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
} 