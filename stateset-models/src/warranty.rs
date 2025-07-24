//! Warranty models and types

use serde::{Deserialize, Serialize};
use stateset_core::{
    traits::{ApiResource, Identifiable},
    types::{Address, Contact, Expandable, Metadata, Money, ResourceId, Timestamp},
};

/// Warranty status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WarrantyStatus {
    Active,
    Expired,
    Claimed,
    Voided,
    Transferred,
    Suspended,
}

/// Warranty type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WarrantyType {
    Manufacturer,
    Extended,
    ServiceContract,
    Replacement,
    Repair,
    PartsCoverage,
    LaborCoverage,
    Comprehensive,
}

/// Warranty coverage level enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WarrantyCoverageLevel {
    Basic,
    Standard,
    Premium,
    Enterprise,
    Custom,
}

/// Warranty claim status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WarrantyClaimStatus {
    Submitted,
    UnderReview,
    Approved,
    Denied,
    InProgress,
    Completed,
    Cancelled,
}

/// Warranty model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warranty {
    pub id: ResourceId,
    pub warranty_number: String,
    pub product_id: ResourceId,
    pub product: Option<Expandable<Product>>,
    pub customer_id: ResourceId,
    pub customer: Option<Expandable<Customer>>,
    pub order_id: Option<ResourceId>,
    pub order: Option<Expandable<Order>>,
    pub status: WarrantyStatus,
    pub warranty_type: WarrantyType,
    pub coverage_level: WarrantyCoverageLevel,
    pub purchase_date: Timestamp,
    pub start_date: Timestamp,
    pub end_date: Timestamp,
    pub duration_months: u32,
    pub coverage_terms: Vec<WarrantyCoverageTerm>,
    pub exclusions: Vec<String>,
    pub purchase_price: Money,
    pub warranty_cost: Option<Money>,
    pub serial_number: Option<String>,
    pub registration_date: Option<Timestamp>,
    pub provider: WarrantyProvider,
    pub claims: Vec<WarrantyClaim>,
    pub transferable: bool,
    pub renewable: bool,
    pub prorated_refund: bool,
    pub contact: Option<Contact>,
    pub location: Option<Address>,
    pub metadata: Option<Metadata>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl Identifiable for Warranty {
    type Id = ResourceId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl ApiResource for Warranty {
    const ENDPOINT: &'static str = "/api/v1/warranties";
    const TYPE_NAME: &'static str = "warranty";
}

/// Warranty coverage term
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarrantyCoverageTerm {
    pub id: ResourceId,
    pub category: String,
    pub description: String,
    pub coverage_percentage: f64,
    pub deductible: Option<Money>,
    pub max_coverage: Option<Money>,
    pub conditions: Vec<String>,
}

/// Warranty provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarrantyProvider {
    pub id: ResourceId,
    pub name: String,
    pub contact_email: String,
    pub contact_phone: Option<String>,
    pub website: Option<String>,
    pub address: Option<Address>,
}

/// Warranty claim
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarrantyClaim {
    pub id: ResourceId,
    pub claim_number: String,
    pub warranty_id: ResourceId,
    pub status: WarrantyClaimStatus,
    pub claim_type: String,
    pub issue_description: String,
    pub submitted_date: Timestamp,
    pub reviewed_date: Option<Timestamp>,
    pub approved_date: Option<Timestamp>,
    pub completed_date: Option<Timestamp>,
    pub claim_amount: Money,
    pub approved_amount: Option<Money>,
    pub denial_reason: Option<String>,
    pub repair_details: Option<String>,
    pub replacement_details: Option<String>,
    pub attachments: Vec<WarrantyClaimAttachment>,
    pub notes: Vec<WarrantyClaimNote>,
    pub metadata: Option<Metadata>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

/// Warranty claim attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarrantyClaimAttachment {
    pub id: ResourceId,
    pub filename: String,
    pub url: String,
    pub content_type: String,
    pub size: u64,
    pub uploaded_by: ResourceId,
    pub uploaded_at: Timestamp,
}

/// Warranty claim note
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarrantyClaimNote {
    pub id: ResourceId,
    pub content: String,
    pub created_by: ResourceId,
    pub created_at: Timestamp,
    pub internal: bool,
}

/// Product model (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: ResourceId,
    pub name: String,
    pub sku: String,
    pub model: Option<String>,
    pub brand: Option<String>,
}

/// Customer model (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: ResourceId,
    pub name: String,
    pub email: String,
}

/// Order model (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: ResourceId,
    pub order_number: String,
}

/// Create warranty request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWarrantyRequest {
    pub product_id: ResourceId,
    pub customer_id: ResourceId,
    pub order_id: Option<ResourceId>,
    pub warranty_type: WarrantyType,
    pub coverage_level: WarrantyCoverageLevel,
    pub purchase_date: Timestamp,
    pub start_date: Timestamp,
    pub duration_months: u32,
    pub purchase_price: Money,
    pub warranty_cost: Option<Money>,
    pub serial_number: Option<String>,
    pub provider: CreateWarrantyProvider,
    pub coverage_terms: Vec<CreateWarrantyCoverageTerm>,
    pub exclusions: Vec<String>,
    pub transferable: bool,
    pub renewable: bool,
    pub prorated_refund: bool,
    pub contact: Option<Contact>,
    pub location: Option<Address>,
    pub metadata: Option<Metadata>,
}

/// Create warranty provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWarrantyProvider {
    pub name: String,
    pub contact_email: String,
    pub contact_phone: Option<String>,
    pub website: Option<String>,
    pub address: Option<Address>,
}

/// Create warranty coverage term
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWarrantyCoverageTerm {
    pub category: String,
    pub description: String,
    pub coverage_percentage: f64,
    pub deductible: Option<Money>,
    pub max_coverage: Option<Money>,
    pub conditions: Vec<String>,
}

impl CreateWarrantyRequest {
    /// Create a builder for the request
    pub fn builder() -> CreateWarrantyRequestBuilder {
        CreateWarrantyRequestBuilder::default()
    }
}

/// Builder for CreateWarrantyRequest
#[derive(Default)]
pub struct CreateWarrantyRequestBuilder {
    product_id: Option<ResourceId>,
    customer_id: Option<ResourceId>,
    order_id: Option<ResourceId>,
    warranty_type: Option<WarrantyType>,
    coverage_level: Option<WarrantyCoverageLevel>,
    purchase_date: Option<Timestamp>,
    start_date: Option<Timestamp>,
    duration_months: Option<u32>,
    purchase_price: Option<Money>,
    warranty_cost: Option<Money>,
    serial_number: Option<String>,
    provider: Option<CreateWarrantyProvider>,
    coverage_terms: Vec<CreateWarrantyCoverageTerm>,
    exclusions: Vec<String>,
    transferable: bool,
    renewable: bool,
    prorated_refund: bool,
    contact: Option<Contact>,
    location: Option<Address>,
    metadata: Option<Metadata>,
}

impl CreateWarrantyRequestBuilder {
    pub fn product_id(mut self, product_id: impl Into<ResourceId>) -> Self {
        self.product_id = Some(product_id.into());
        self
    }

    pub fn customer_id(mut self, customer_id: impl Into<ResourceId>) -> Self {
        self.customer_id = Some(customer_id.into());
        self
    }

    pub fn order_id(mut self, order_id: impl Into<ResourceId>) -> Self {
        self.order_id = Some(order_id.into());
        self
    }

    pub fn warranty_type(mut self, warranty_type: WarrantyType) -> Self {
        self.warranty_type = Some(warranty_type);
        self
    }

    pub fn coverage_level(mut self, coverage_level: WarrantyCoverageLevel) -> Self {
        self.coverage_level = Some(coverage_level);
        self
    }

    pub fn purchase_date(mut self, date: Timestamp) -> Self {
        self.purchase_date = Some(date);
        self
    }

    pub fn start_date(mut self, date: Timestamp) -> Self {
        self.start_date = Some(date);
        self
    }

    pub fn duration_months(mut self, months: u32) -> Self {
        self.duration_months = Some(months);
        self
    }

    pub fn purchase_price(mut self, price: Money) -> Self {
        self.purchase_price = Some(price);
        self
    }

    pub fn warranty_cost(mut self, cost: Money) -> Self {
        self.warranty_cost = Some(cost);
        self
    }

    pub fn serial_number(mut self, serial: impl Into<String>) -> Self {
        self.serial_number = Some(serial.into());
        self
    }

    pub fn provider(mut self, provider: CreateWarrantyProvider) -> Self {
        self.provider = Some(provider);
        self
    }

    pub fn add_coverage_term(mut self, term: CreateWarrantyCoverageTerm) -> Self {
        self.coverage_terms.push(term);
        self
    }

    pub fn add_exclusion(mut self, exclusion: impl Into<String>) -> Self {
        self.exclusions.push(exclusion.into());
        self
    }

    pub fn transferable(mut self, transferable: bool) -> Self {
        self.transferable = transferable;
        self
    }

    pub fn renewable(mut self, renewable: bool) -> Self {
        self.renewable = renewable;
        self
    }

    pub fn prorated_refund(mut self, prorated: bool) -> Self {
        self.prorated_refund = prorated;
        self
    }

    pub fn contact(mut self, contact: Contact) -> Self {
        self.contact = Some(contact);
        self
    }

    pub fn location(mut self, location: Address) -> Self {
        self.location = Some(location);
        self
    }

    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn build(self) -> Result<CreateWarrantyRequest, String> {
        Ok(CreateWarrantyRequest {
            product_id: self.product_id.ok_or("product_id is required")?,
            customer_id: self.customer_id.ok_or("customer_id is required")?,
            order_id: self.order_id,
            warranty_type: self.warranty_type.ok_or("warranty_type is required")?,
            coverage_level: self.coverage_level.unwrap_or(WarrantyCoverageLevel::Standard),
            purchase_date: self.purchase_date.ok_or("purchase_date is required")?,
            start_date: self.start_date.ok_or("start_date is required")?,
            duration_months: self.duration_months.ok_or("duration_months is required")?,
            purchase_price: self.purchase_price.ok_or("purchase_price is required")?,
            warranty_cost: self.warranty_cost,
            serial_number: self.serial_number,
            provider: self.provider.ok_or("provider is required")?,
            coverage_terms: self.coverage_terms,
            exclusions: self.exclusions,
            transferable: self.transferable,
            renewable: self.renewable,
            prorated_refund: self.prorated_refund,
            contact: self.contact,
            location: self.location,
            metadata: self.metadata,
        })
    }
}

/// Create warranty claim request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWarrantyClaimRequest {
    pub warranty_id: ResourceId,
    pub claim_type: String,
    pub issue_description: String,
    pub claim_amount: Money,
    pub repair_details: Option<String>,
    pub replacement_details: Option<String>,
    pub metadata: Option<Metadata>,
}

/// Update warranty request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateWarrantyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<WarrantyStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage_level: Option<WarrantyCoverageLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transferable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// Update warranty claim request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateWarrantyClaimRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<WarrantyClaimStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_amount: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub denial_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repair_details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// Warranty list filters
#[derive(Debug, Clone, Default, Serialize)]
pub struct WarrantyListFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<WarrantyStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warranty_type: Option<WarrantyType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage_level: Option<WarrantyCoverageLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<ResourceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<ResourceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_before: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<Timestamp>,
}

/// Warranty claim list filters
#[derive(Debug, Clone, Default, Serialize)]
pub struct WarrantyClaimListFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<WarrantyClaimStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warranty_id: Option<ResourceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_after: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_before: Option<Timestamp>,
}