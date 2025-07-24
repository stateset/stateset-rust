//! Common types used throughout StateSet SDK

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Common resource ID type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ResourceId(pub Uuid);

impl ResourceId {
    /// Create a new resource ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Parse a resource ID from a string
    pub fn parse(s: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(s)?))
    }

    /// Get the inner UUID
    pub fn inner(&self) -> &Uuid {
        &self.0
    }
}

impl Default for ResourceId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ResourceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Uuid> for ResourceId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl AsRef<Uuid> for ResourceId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

/// Timestamp type alias
pub type Timestamp = DateTime<Utc>;

/// Reference type for linking resources
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "id")]
pub enum ReferenceType {
    #[serde(rename = "sales_order")]
    SalesOrder(ResourceId),
    #[serde(rename = "purchase_order")]
    PurchaseOrder(ResourceId),
    #[serde(rename = "work_order")]
    WorkOrder(ResourceId),
    #[serde(rename = "shipment")]
    Shipment(ResourceId),
    #[serde(rename = "return")]
    Return(ResourceId),
    #[serde(rename = "transfer")]
    Transfer(ResourceId),
    #[serde(rename = "adjustment")]
    Adjustment(ResourceId),
}

/// Expandable field that can be either an ID or a full object
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Expandable<T> {
    /// Just the ID reference
    Id(String),
    /// Expanded object
    Expanded(Box<T>),
}

impl<T> Expandable<T> {
    /// Check if the field is expanded
    pub fn is_expanded(&self) -> bool {
        matches!(self, Self::Expanded(_))
    }

    /// Get the expanded value if available
    pub fn expanded(&self) -> Option<&T> {
        match self {
            Self::Expanded(obj) => Some(obj),
            _ => None,
        }
    }

    /// Get the ID reference
    pub fn id_ref(&self) -> Option<&str> {
        match self {
            Self::Id(id) => Some(id),
            _ => None,
        }
    }

    /// Convert to owned expanded value
    pub fn into_expanded(self) -> Option<T> {
        match self {
            Self::Expanded(obj) => Some(*obj),
            _ => None,
        }
    }
}

/// Metadata key-value pairs
pub type Metadata = std::collections::HashMap<String, serde_json::Value>;

/// Address structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub line1: String,
    pub line2: Option<String>,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
}

/// Contact information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

/// Money/currency amount
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Money {
    /// Amount in smallest currency unit (e.g., cents)
    pub amount: i64,
    /// ISO 4217 currency code
    pub currency: String,
}

impl Money {
    /// Create a new money instance
    pub fn new(amount: i64, currency: impl Into<String>) -> Self {
        Self { 
            amount, 
            currency: currency.into(),
        }
    }

    /// Create from a decimal amount (e.g., dollars to cents)
    pub fn from_decimal(amount: f64, currency: impl Into<String>) -> Self {
        Self {
            amount: (amount * 100.0).round() as i64,
            currency: currency.into(),
        }
    }

    /// Get the decimal representation
    pub fn to_decimal(&self) -> f64 {
        self.amount as f64 / 100.0
    }
}

/// Sort direction for queries
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    Asc,
    Desc,
}

/// Common query parameters
#[derive(Debug, Clone, Default, Serialize)]
pub struct QueryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<SortDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

/// Date range filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Timestamp>,
} 