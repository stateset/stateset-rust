//! Order-related models

use serde::{Deserialize, Serialize};
use stateset_core::{
    traits::{ApiResource, Identifiable},
    types::{Address, Contact, Expandable, Metadata, Money, ResourceId, Timestamp},
};

/// Order status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Draft,
    Pending,
    Confirmed,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
    Refunded,
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStatus::Draft => write!(f, "draft"),
            OrderStatus::Pending => write!(f, "pending"),
            OrderStatus::Confirmed => write!(f, "confirmed"),
            OrderStatus::Processing => write!(f, "processing"),
            OrderStatus::Shipped => write!(f, "shipped"),
            OrderStatus::Delivered => write!(f, "delivered"),
            OrderStatus::Cancelled => write!(f, "cancelled"),
            OrderStatus::Refunded => write!(f, "refunded"),
        }
    }
}

/// Order model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: ResourceId,
    pub order_number: String,
    pub status: OrderStatus,
    pub customer_id: ResourceId,
    pub customer: Option<Expandable<Customer>>,
    pub items: Vec<OrderItem>,
    pub subtotal: Money,
    pub tax: Money,
    pub shipping: Money,
    pub total: Money,
    pub currency: String,
    pub shipping_address: Option<Address>,
    pub billing_address: Option<Address>,
    pub contact: Option<Contact>,
    pub notes: Option<String>,
    pub metadata: Option<Metadata>,
    pub tracking_number: Option<String>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl Identifiable for Order {
    type Id = ResourceId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl ApiResource for Order {
    const ENDPOINT: &'static str = "/api/v1/orders";
    const TYPE_NAME: &'static str = "order";
}

/// Order item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub id: ResourceId,
    pub product_id: ResourceId,
    pub product: Option<Expandable<Product>>,
    pub sku: String,
    pub name: String,
    pub quantity: u32,
    pub unit_price: Money,
    pub total_price: Money,
    pub metadata: Option<Metadata>,
}

impl OrderItem {
    /// Create a new order item
    pub fn new(product_id: impl Into<ResourceId>, quantity: u32) -> Self {
        Self {
            id: ResourceId::new(),
            product_id: product_id.into(),
            product: None,
            sku: String::new(),
            name: String::new(),
            quantity,
            unit_price: Money::new(0, "USD"),
            total_price: Money::new(0, "USD"),
            metadata: None,
        }
    }
}

/// Customer model (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: ResourceId,
    pub email: String,
    pub name: Option<String>,
}

/// Product model (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: ResourceId,
    pub sku: String,
    pub name: String,
    pub price: Money,
}

/// Create order request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub customer_id: ResourceId,
    pub items: Vec<CreateOrderItem>,
    pub shipping_address: Option<Address>,
    pub billing_address: Option<Address>,
    pub contact: Option<Contact>,
    pub notes: Option<String>,
    pub metadata: Option<Metadata>,
}

/// Create order item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderItem {
    pub product_id: ResourceId,
    pub quantity: u32,
    pub unit_price: Option<Money>,
    pub metadata: Option<Metadata>,
}

impl CreateOrderRequest {
    /// Create a builder for the request
    pub fn builder() -> CreateOrderRequestBuilder {
        CreateOrderRequestBuilder::default()
    }
}

/// Builder for CreateOrderRequest
#[derive(Default)]
pub struct CreateOrderRequestBuilder {
    customer_id: Option<ResourceId>,
    items: Vec<CreateOrderItem>,
    shipping_address: Option<Address>,
    billing_address: Option<Address>,
    contact: Option<Contact>,
    notes: Option<String>,
    metadata: Option<Metadata>,
}

impl CreateOrderRequestBuilder {
    pub fn customer_id(mut self, id: impl Into<ResourceId>) -> Self {
        self.customer_id = Some(id.into());
        self
    }

    pub fn items(mut self, items: Vec<OrderItem>) -> Self {
        self.items = items
            .into_iter()
            .map(|item| CreateOrderItem {
                product_id: item.product_id,
                quantity: item.quantity,
                unit_price: Some(item.unit_price),
                metadata: item.metadata,
            })
            .collect();
        self
    }

    pub fn add_item(mut self, product_id: impl Into<ResourceId>, quantity: u32) -> Self {
        self.items.push(CreateOrderItem {
            product_id: product_id.into(),
            quantity,
            unit_price: None,
            metadata: None,
        });
        self
    }

    pub fn shipping_address(mut self, address: Address) -> Self {
        self.shipping_address = Some(address);
        self
    }

    pub fn billing_address(mut self, address: Address) -> Self {
        self.billing_address = Some(address);
        self
    }

    pub fn contact(mut self, contact: Contact) -> Self {
        self.contact = Some(contact);
        self
    }

    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn build(self) -> CreateOrderRequest {
        CreateOrderRequest {
            customer_id: self.customer_id.expect("customer_id is required"),
            items: self.items,
            shipping_address: self.shipping_address,
            billing_address: self.billing_address,
            contact: self.contact,
            notes: self.notes,
            metadata: self.metadata,
        }
    }
}

/// Update order request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// Order list filters
#[derive(Debug, Clone, Default, Serialize)]
pub struct OrderListFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<ResourceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_total: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_total: Option<i64>,
} 