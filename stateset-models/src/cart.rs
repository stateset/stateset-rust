//! Cart models and types

use serde::{Deserialize, Serialize};
use stateset_core::{
    traits::{ApiResource, Identifiable},
    types::{Address, Contact, Expandable, Metadata, Money, ResourceId, Timestamp},
};

/// Cart status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CartStatus {
    Active,
    Abandoned,
    Converted,
    Expired,
    Merged,
    Saved,
}

/// Cart type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CartType {
    Shopping,
    Wishlist,
    SavedForLater,
    QuickOrder,
    Subscription,
    Quote,
}

/// Cart model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cart {
    pub id: ResourceId,
    pub cart_token: String,
    pub session_id: Option<String>,
    pub customer_id: Option<ResourceId>,
    pub customer: Option<Expandable<Customer>>,
    pub status: CartStatus,
    pub cart_type: CartType,
    pub currency: String,
    pub items: Vec<CartItem>,
    pub item_count: u32,
    pub total_quantity: u32,
    pub subtotal: Money,
    pub tax_total: Money,
    pub shipping_total: Money,
    pub discount_total: Money,
    pub total: Money,
    pub applied_coupons: Vec<AppliedCoupon>,
    pub shipping_address: Option<Address>,
    pub billing_address: Option<Address>,
    pub contact: Option<Contact>,
    pub shipping_method_id: Option<ResourceId>,
    pub shipping_method: Option<Expandable<ShippingMethod>>,
    pub payment_method_id: Option<ResourceId>,
    pub payment_method: Option<Expandable<PaymentMethod>>,
    pub notes: Option<String>,
    pub abandoned_at: Option<Timestamp>,
    pub converted_at: Option<Timestamp>,
    pub converted_order_id: Option<ResourceId>,
    pub expires_at: Option<Timestamp>,
    pub metadata: Option<Metadata>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl Identifiable for Cart {
    type Id = ResourceId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl ApiResource for Cart {
    const ENDPOINT: &'static str = "/api/v1/carts";
    const TYPE_NAME: &'static str = "cart";
}

/// Cart item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItem {
    pub id: ResourceId,
    pub product_id: ResourceId,
    pub product: Option<Expandable<Product>>,
    pub variant_id: Option<ResourceId>,
    pub variant: Option<Expandable<ProductVariant>>,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub quantity: u32,
    pub unit_price: Money,
    pub line_total: Money,
    pub original_price: Money,
    pub discount_amount: Money,
    pub tax_amount: Money,
    pub weight: Option<f64>,
    pub dimensions: Option<ProductDimensions>,
    pub image_url: Option<String>,
    pub custom_attributes: Vec<CartItemAttribute>,
    pub personalization: Option<String>,
    pub gift_wrap: Option<GiftWrap>,
    pub recurring: Option<RecurringInfo>,
    pub added_at: Timestamp,
    pub updated_at: Timestamp,
    pub metadata: Option<Metadata>,
}

/// Cart item attribute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItemAttribute {
    pub name: String,
    pub value: String,
    pub display_name: Option<String>,
}

/// Gift wrap information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftWrap {
    pub id: ResourceId,
    pub name: String,
    pub price: Money,
    pub message: Option<String>,
}

/// Recurring information for subscriptions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringInfo {
    pub interval: String, // "day", "week", "month", "year"
    pub interval_count: u32,
    pub trial_period_days: Option<u32>,
}

/// Applied coupon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedCoupon {
    pub id: ResourceId,
    pub code: String,
    pub name: String,
    pub discount_type: String, // "percentage", "fixed_amount", "free_shipping"
    pub discount_value: f64,
    pub discount_amount: Money,
    pub minimum_amount: Option<Money>,
    pub maximum_discount: Option<Money>,
    pub applied_at: Timestamp,
}

/// Product dimensions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductDimensions {
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub unit: String,
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
    pub name: String,
    pub sku: String,
    pub price: Money,
    pub image_url: Option<String>,
}

/// Product variant model (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductVariant {
    pub id: ResourceId,
    pub name: String,
    pub sku: String,
    pub price: Money,
    pub attributes: Vec<CartItemAttribute>,
}

/// Shipping method model (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingMethod {
    pub id: ResourceId,
    pub name: String,
    pub description: Option<String>,
    pub cost: Money,
    pub estimated_delivery: Option<String>,
}

/// Payment method model (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    pub id: ResourceId,
    pub name: String,
    pub type_name: String,
    pub description: Option<String>,
}

/// Create cart request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCartRequest {
    pub customer_id: Option<ResourceId>,
    pub session_id: Option<String>,
    pub cart_type: Option<CartType>,
    pub currency: Option<String>,
    pub shipping_address: Option<Address>,
    pub billing_address: Option<Address>,
    pub contact: Option<Contact>,
    pub notes: Option<String>,
    pub expires_at: Option<Timestamp>,
    pub metadata: Option<Metadata>,
}

impl CreateCartRequest {
    /// Create a builder for the request
    pub fn builder() -> CreateCartRequestBuilder {
        CreateCartRequestBuilder::default()
    }
}

/// Builder for CreateCartRequest
#[derive(Default)]
pub struct CreateCartRequestBuilder {
    customer_id: Option<ResourceId>,
    session_id: Option<String>,
    cart_type: Option<CartType>,
    currency: Option<String>,
    shipping_address: Option<Address>,
    billing_address: Option<Address>,
    contact: Option<Contact>,
    notes: Option<String>,
    expires_at: Option<Timestamp>,
    metadata: Option<Metadata>,
}

impl CreateCartRequestBuilder {
    pub fn customer_id(mut self, customer_id: impl Into<ResourceId>) -> Self {
        self.customer_id = Some(customer_id.into());
        self
    }

    pub fn session_id(mut self, session_id: impl Into<String>) -> Self {
        self.session_id = Some(session_id.into());
        self
    }

    pub fn cart_type(mut self, cart_type: CartType) -> Self {
        self.cart_type = Some(cart_type);
        self
    }

    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
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

    pub fn expires_at(mut self, expires_at: Timestamp) -> Self {
        self.expires_at = Some(expires_at);
        self
    }

    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn build(self) -> CreateCartRequest {
        CreateCartRequest {
            customer_id: self.customer_id,
            session_id: self.session_id,
            cart_type: self.cart_type,
            currency: self.currency,
            shipping_address: self.shipping_address,
            billing_address: self.billing_address,
            contact: self.contact,
            notes: self.notes,
            expires_at: self.expires_at,
            metadata: self.metadata,
        }
    }
}

/// Add item to cart request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddCartItemRequest {
    pub product_id: ResourceId,
    pub variant_id: Option<ResourceId>,
    pub quantity: u32,
    pub unit_price: Option<Money>,
    pub custom_attributes: Vec<CartItemAttribute>,
    pub personalization: Option<String>,
    pub gift_wrap_id: Option<ResourceId>,
    pub recurring: Option<RecurringInfo>,
    pub metadata: Option<Metadata>,
}

/// Update cart item request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCartItemRequest {
    pub quantity: Option<u32>,
    pub custom_attributes: Option<Vec<CartItemAttribute>>,
    pub personalization: Option<String>,
    pub gift_wrap_id: Option<ResourceId>,
    pub metadata: Option<Metadata>,
}

/// Apply coupon request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyCouponRequest {
    pub coupon_code: String,
}

/// Update cart request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateCartRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<ResourceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CartStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_method_id: Option<ResourceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<ResourceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// Cart abandonment recovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartRecovery {
    pub cart_id: ResourceId,
    pub recovery_token: String,
    pub recovery_url: String,
    pub email_sent: bool,
    pub email_sent_at: Option<Timestamp>,
    pub recovered: bool,
    pub recovered_at: Option<Timestamp>,
    pub created_at: Timestamp,
}

/// Cart analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartAnalytics {
    pub total_carts: u32,
    pub active_carts: u32,
    pub abandoned_carts: u32,
    pub converted_carts: u32,
    pub abandonment_rate: f64,
    pub conversion_rate: f64,
    pub average_cart_value: Money,
    pub total_cart_value: Money,
    pub period_start: Timestamp,
    pub period_end: Timestamp,
}

/// Cart list filters
#[derive(Debug, Clone, Default, Serialize)]
pub struct CartListFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CartStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cart_type: Option<CartType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<ResourceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_total: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_total: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_customer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abandoned_after: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abandoned_before: Option<Timestamp>,
}