//! Checkout models and types

use serde::{Deserialize, Serialize};
use stateset_core::{
    traits::{ApiResource, Identifiable},
    types::{Address, Contact, Expandable, Metadata, Money, ResourceId, Timestamp},
};

/// Checkout status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutStatus {
    Started,
    InProgress,
    PaymentPending,
    PaymentProcessing,
    PaymentFailed,
    Completed,
    Abandoned,
    Expired,
    Cancelled,
}

/// Checkout step enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutStep {
    Cart,
    Information,
    Shipping,
    Payment,
    Review,
    Confirmation,
}

/// Payment status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    Pending,
    Processing,
    Authorized,
    Captured,
    Failed,
    Cancelled,
    Refunded,
    PartiallyRefunded,
}

/// Checkout model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkout {
    pub id: ResourceId,
    pub checkout_token: String,
    pub cart_id: ResourceId,
    pub cart: Option<Expandable<Cart>>,
    pub customer_id: Option<ResourceId>,
    pub customer: Option<Expandable<Customer>>,
    pub status: CheckoutStatus,
    pub current_step: CheckoutStep,
    pub completed_steps: Vec<CheckoutStep>,
    pub currency: String,
    pub line_items: Vec<CheckoutLineItem>,
    pub subtotal: Money,
    pub tax_total: Money,
    pub shipping_total: Money,
    pub discount_total: Money,
    pub total: Money,
    pub shipping_address: Option<Address>,
    pub billing_address: Option<Address>,
    pub contact: Contact,
    pub shipping_method_id: Option<ResourceId>,
    pub shipping_method: Option<Expandable<ShippingMethod>>,
    pub shipping_rates: Vec<ShippingRate>,
    pub payment_method_id: Option<ResourceId>,
    pub payment_method: Option<Expandable<PaymentMethod>>,
    pub payment_intent_id: Option<String>,
    pub payment_status: PaymentStatus,
    pub applied_coupons: Vec<AppliedCoupon>,
    pub tax_lines: Vec<TaxLine>,
    pub gift_cards: Vec<AppliedGiftCard>,
    pub notes: Option<String>,
    pub special_instructions: Option<String>,
    pub marketing_consent: bool,
    pub terms_accepted: bool,
    pub privacy_policy_accepted: bool,
    pub newsletter_signup: bool,
    pub validation_errors: Vec<ValidationError>,
    pub completed_at: Option<Timestamp>,
    pub abandoned_at: Option<Timestamp>,
    pub expires_at: Option<Timestamp>,
    pub created_order_id: Option<ResourceId>,
    pub metadata: Option<Metadata>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl Identifiable for Checkout {
    type Id = ResourceId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl ApiResource for Checkout {
    const ENDPOINT: &'static str = "/api/v1/checkouts";
    const TYPE_NAME: &'static str = "checkout";
}

/// Checkout line item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutLineItem {
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
    pub requires_shipping: bool,
    pub is_gift_card: bool,
    pub gift_card_recipient: Option<GiftCardRecipient>,
    pub custom_attributes: Vec<LineItemAttribute>,
    pub metadata: Option<Metadata>,
}

/// Gift card recipient information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCardRecipient {
    pub name: String,
    pub email: String,
    pub message: Option<String>,
    pub delivery_date: Option<Timestamp>,
}

/// Line item attribute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineItemAttribute {
    pub name: String,
    pub value: String,
    pub display_name: Option<String>,
}

/// Shipping rate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingRate {
    pub id: ResourceId,
    pub carrier: String,
    pub service_name: String,
    pub service_code: String,
    pub price: Money,
    pub estimated_delivery_min: Option<u32>,
    pub estimated_delivery_max: Option<u32>,
    pub delivery_date: Option<Timestamp>,
    pub description: Option<String>,
    pub is_available: bool,
}

/// Applied coupon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedCoupon {
    pub id: ResourceId,
    pub code: String,
    pub name: String,
    pub discount_type: String,
    pub discount_value: f64,
    pub discount_amount: Money,
    pub applied_at: Timestamp,
}

/// Tax line
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxLine {
    pub id: ResourceId,
    pub name: String,
    pub rate: f64,
    pub amount: Money,
    pub included_in_price: bool,
    pub jurisdiction: Option<String>,
}

/// Applied gift card
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedGiftCard {
    pub id: ResourceId,
    pub code: String,
    pub balance: Money,
    pub amount_used: Money,
    pub last_four: String,
}

/// Validation error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub code: String,
    pub message: String,
    pub severity: String, // "error", "warning", "info"
}

/// Cart model (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cart {
    pub id: ResourceId,
    pub total: Money,
    pub item_count: u32,
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
}

/// Product variant model (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductVariant {
    pub id: ResourceId,
    pub name: String,
    pub sku: String,
}

/// Shipping method model (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingMethod {
    pub id: ResourceId,
    pub name: String,
    pub description: Option<String>,
}

/// Payment method model (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    pub id: ResourceId,
    pub name: String,
    pub type_name: String,
}

/// Create checkout request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCheckoutRequest {
    pub cart_id: ResourceId,
    pub customer_id: Option<ResourceId>,
    pub contact: Contact,
    pub shipping_address: Option<Address>,
    pub billing_address: Option<Address>,
    pub currency: Option<String>,
    pub notes: Option<String>,
    pub marketing_consent: bool,
    pub newsletter_signup: bool,
    pub metadata: Option<Metadata>,
}

impl CreateCheckoutRequest {
    /// Create a builder for the request
    pub fn builder() -> CreateCheckoutRequestBuilder {
        CreateCheckoutRequestBuilder::default()
    }
}

/// Builder for CreateCheckoutRequest
#[derive(Default)]
pub struct CreateCheckoutRequestBuilder {
    cart_id: Option<ResourceId>,
    customer_id: Option<ResourceId>,
    contact: Option<Contact>,
    shipping_address: Option<Address>,
    billing_address: Option<Address>,
    currency: Option<String>,
    notes: Option<String>,
    marketing_consent: bool,
    newsletter_signup: bool,
    metadata: Option<Metadata>,
}

impl CreateCheckoutRequestBuilder {
    pub fn cart_id(mut self, cart_id: impl Into<ResourceId>) -> Self {
        self.cart_id = Some(cart_id.into());
        self
    }

    pub fn customer_id(mut self, customer_id: impl Into<ResourceId>) -> Self {
        self.customer_id = Some(customer_id.into());
        self
    }

    pub fn contact(mut self, contact: Contact) -> Self {
        self.contact = Some(contact);
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

    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }

    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    pub fn marketing_consent(mut self, consent: bool) -> Self {
        self.marketing_consent = consent;
        self
    }

    pub fn newsletter_signup(mut self, signup: bool) -> Self {
        self.newsletter_signup = signup;
        self
    }

    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn build(self) -> Result<CreateCheckoutRequest, String> {
        Ok(CreateCheckoutRequest {
            cart_id: self.cart_id.ok_or("cart_id is required")?,
            customer_id: self.customer_id,
            contact: self.contact.ok_or("contact is required")?,
            shipping_address: self.shipping_address,
            billing_address: self.billing_address,
            currency: self.currency,
            notes: self.notes,
            marketing_consent: self.marketing_consent,
            newsletter_signup: self.newsletter_signup,
            metadata: self.metadata,
        })
    }
}

/// Update checkout request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateCheckoutRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_step: Option<CheckoutStep>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_method_id: Option<ResourceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<ResourceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketing_consent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_accepted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy_accepted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newsletter_signup: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// Apply coupon to checkout request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyCheckoutCouponRequest {
    pub coupon_code: String,
}

/// Apply gift card to checkout request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyGiftCardRequest {
    pub gift_card_code: String,
}

/// Complete checkout request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteCheckoutRequest {
    pub payment_method_id: ResourceId,
    pub payment_details: PaymentDetails,
    pub save_payment_method: bool,
    pub terms_accepted: bool,
    pub privacy_policy_accepted: bool,
}

/// Payment details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentDetails {
    pub payment_method_type: String,
    pub card_details: Option<CardDetails>,
    pub digital_wallet_details: Option<DigitalWalletDetails>,
    pub bank_transfer_details: Option<BankTransferDetails>,
    pub billing_address: Address,
}

/// Card payment details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardDetails {
    pub card_number: String,
    pub expiry_month: u32,
    pub expiry_year: u32,
    pub cvv: String,
    pub cardholder_name: String,
}

/// Digital wallet payment details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalWalletDetails {
    pub wallet_type: String, // "apple_pay", "google_pay", "paypal"
    pub wallet_token: String,
}

/// Bank transfer payment details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferDetails {
    pub account_number: String,
    pub routing_number: String,
    pub bank_name: String,
    pub account_holder_name: String,
}

/// Checkout completion result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutCompletionResult {
    pub checkout_id: ResourceId,
    pub order_id: ResourceId,
    pub payment_status: PaymentStatus,
    pub payment_intent_id: Option<String>,
    pub requires_action: bool,
    pub client_secret: Option<String>,
    pub redirect_url: Option<String>,
    pub confirmation_number: String,
}

/// Checkout abandonment analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutAbandonmentAnalysis {
    pub total_checkouts: u32,
    pub completed_checkouts: u32,
    pub abandoned_checkouts: u32,
    pub completion_rate: f64,
    pub abandonment_rate: f64,
    pub abandonment_by_step: Vec<StepAbandonmentData>,
    pub average_completion_time: f64, // seconds
    pub period_start: Timestamp,
    pub period_end: Timestamp,
}

/// Step abandonment data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepAbandonmentData {
    pub step: CheckoutStep,
    pub started: u32,
    pub completed: u32,
    pub abandoned: u32,
    pub completion_rate: f64,
    pub average_time_spent: f64, // seconds
}

/// Checkout list filters
#[derive(Debug, Clone, Default, Serialize)]
pub struct CheckoutListFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CheckoutStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_step: Option<CheckoutStep>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_status: Option<PaymentStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<ResourceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cart_id: Option<ResourceId>,
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
    pub completed_after: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_before: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abandoned_after: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abandoned_before: Option<Timestamp>,
}