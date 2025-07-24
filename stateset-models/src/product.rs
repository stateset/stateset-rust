//! Product models and types

use serde::{Deserialize, Serialize};
use stateset_core::{
    traits::{ApiResource, Identifiable},
    types::{Address, Expandable, Metadata, Money, ResourceId, Timestamp},
};

/// Product status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProductStatus {
    Draft,
    Active,
    Inactive,
    Discontinued,
    OutOfStock,
    Backordered,
    PreOrder,
    Archived,
}

/// Product type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProductType {
    Physical,
    Digital,
    Service,
    Subscription,
    Bundle,
    GiftCard,
    Variable,
    Grouped,
}

/// Product visibility enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProductVisibility {
    Public,
    Private,
    Hidden,
    Catalog,
    Search,
}

/// Product condition enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProductCondition {
    New,
    Used,
    Refurbished,
    OpenBox,
    Damaged,
    Prototype,
}

/// Product model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: ResourceId,
    pub sku: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub short_description: Option<String>,
    pub status: ProductStatus,
    pub product_type: ProductType,
    pub visibility: ProductVisibility,
    pub condition: ProductCondition,
    pub brand_id: Option<ResourceId>,
    pub brand: Option<Expandable<Brand>>,
    pub category_id: Option<ResourceId>,
    pub category: Option<Expandable<Category>>,
    pub tags: Vec<String>,
    pub images: Vec<ProductImage>,
    pub videos: Vec<ProductVideo>,
    pub documents: Vec<ProductDocument>,
    pub variants: Vec<ProductVariant>,
    pub attributes: Vec<ProductAttribute>,
    pub pricing: ProductPricing,
    pub inventory: ProductInventory,
    pub shipping: ProductShipping,
    pub seo: ProductSeo,
    pub related_products: Vec<ResourceId>,
    pub cross_sell_products: Vec<ResourceId>,
    pub up_sell_products: Vec<ResourceId>,
    pub weight: Option<f64>,
    pub dimensions: Option<ProductDimensions>,
    pub color: Option<String>,
    pub size: Option<String>,
    pub material: Option<String>,
    pub model_number: Option<String>,
    pub manufacturer_part_number: Option<String>,
    pub upc: Option<String>,
    pub ean: Option<String>,
    pub isbn: Option<String>,
    pub gtin: Option<String>,
    pub tax_class_id: Option<ResourceId>,
    pub tax_class: Option<Expandable<TaxClass>>,
    pub requires_shipping: bool,
    pub is_virtual: bool,
    pub is_downloadable: bool,
    pub download_limit: Option<u32>,
    pub download_expiry: Option<u32>,
    pub external_url: Option<String>,
    pub button_text: Option<String>,
    pub reviews_allowed: bool,
    pub average_rating: Option<f64>,
    pub review_count: u32,
    pub featured: bool,
    pub metadata: Option<Metadata>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl Identifiable for Product {
    type Id = ResourceId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl ApiResource for Product {
    const ENDPOINT: &'static str = "/api/v1/products";
    const TYPE_NAME: &'static str = "product";
}

/// Product image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductImage {
    pub id: ResourceId,
    pub url: String,
    pub alt_text: Option<String>,
    pub caption: Option<String>,
    pub position: u32,
    pub is_primary: bool,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub file_size: Option<u64>,
    pub format: Option<String>,
}

/// Product video
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductVideo {
    pub id: ResourceId,
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub duration: Option<u32>,
    pub position: u32,
    pub provider: Option<String>,
}

/// Product document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductDocument {
    pub id: ResourceId,
    pub filename: String,
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub content_type: String,
    pub file_size: u64,
    pub position: u32,
    pub is_public: bool,
}

/// Product variant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductVariant {
    pub id: ResourceId,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub status: ProductStatus,
    pub attributes: Vec<ProductVariantAttribute>,
    pub pricing: ProductPricing,
    pub inventory: ProductInventory,
    pub weight: Option<f64>,
    pub dimensions: Option<ProductDimensions>,
    pub images: Vec<ProductImage>,
    pub position: u32,
    pub is_default: bool,
    pub metadata: Option<Metadata>,
}

/// Product variant attribute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductVariantAttribute {
    pub name: String,
    pub value: String,
    pub display_name: Option<String>,
}

/// Product attribute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductAttribute {
    pub id: ResourceId,
    pub name: String,
    pub value: String,
    pub display_name: Option<String>,
    pub attribute_type: String,
    pub is_visible: bool,
    pub is_variation: bool,
    pub position: u32,
}

/// Product pricing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductPricing {
    pub regular_price: Money,
    pub sale_price: Option<Money>,
    pub cost_price: Option<Money>,
    pub wholesale_price: Option<Money>,
    pub msrp: Option<Money>,
    pub tax_included: bool,
    pub sale_start_date: Option<Timestamp>,
    pub sale_end_date: Option<Timestamp>,
    pub price_tiers: Vec<PriceTier>,
}

/// Price tier for bulk pricing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceTier {
    pub min_quantity: u32,
    pub max_quantity: Option<u32>,
    pub price: Money,
    pub discount_percentage: Option<f64>,
}

/// Product inventory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductInventory {
    pub track_quantity: bool,
    pub quantity: Option<u32>,
    pub reserved_quantity: Option<u32>,
    pub available_quantity: Option<u32>,
    pub low_stock_threshold: Option<u32>,
    pub stock_status: Option<String>,
    pub backorders_allowed: bool,
    pub manage_stock: bool,
    pub sold_individually: bool,
    pub inventory_locations: Vec<InventoryLocation>,
}

/// Inventory location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryLocation {
    pub location_id: ResourceId,
    pub location_name: String,
    pub quantity: u32,
    pub reserved_quantity: u32,
    pub available_quantity: u32,
}

/// Product shipping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductShipping {
    pub requires_shipping: bool,
    pub weight: Option<f64>,
    pub dimensions: Option<ProductDimensions>,
    pub shipping_class_id: Option<ResourceId>,
    pub shipping_class: Option<Expandable<ShippingClass>>,
    pub free_shipping: bool,
    pub separate_shipping: bool,
    pub shipping_cost: Option<Money>,
}

/// Product dimensions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductDimensions {
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub unit: String, // "in", "cm", etc.
}

/// Product SEO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSeo {
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,
    pub focus_keyword: Option<String>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub og_image: Option<String>,
    pub twitter_title: Option<String>,
    pub twitter_description: Option<String>,
    pub twitter_image: Option<String>,
}

/// Brand model (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brand {
    pub id: ResourceId,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub logo_url: Option<String>,
}

/// Category model (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: ResourceId,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub parent_id: Option<ResourceId>,
    pub image_url: Option<String>,
}

/// Tax class model (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxClass {
    pub id: ResourceId,
    pub name: String,
    pub rate: f64,
}

/// Shipping class model (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingClass {
    pub id: ResourceId,
    pub name: String,
    pub description: Option<String>,
    pub cost: Money,
}

/// Create product request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProductRequest {
    pub sku: String,
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub short_description: Option<String>,
    pub product_type: ProductType,
    pub status: Option<ProductStatus>,
    pub visibility: Option<ProductVisibility>,
    pub condition: Option<ProductCondition>,
    pub brand_id: Option<ResourceId>,
    pub category_id: Option<ResourceId>,
    pub tags: Vec<String>,
    pub attributes: Vec<CreateProductAttribute>,
    pub pricing: CreateProductPricing,
    pub inventory: Option<CreateProductInventory>,
    pub shipping: Option<CreateProductShipping>,
    pub seo: Option<CreateProductSeo>,
    pub weight: Option<f64>,
    pub dimensions: Option<ProductDimensions>,
    pub tax_class_id: Option<ResourceId>,
    pub requires_shipping: bool,
    pub is_virtual: bool,
    pub is_downloadable: bool,
    pub reviews_allowed: bool,
    pub featured: bool,
    pub metadata: Option<Metadata>,
}

/// Create product attribute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProductAttribute {
    pub name: String,
    pub value: String,
    pub display_name: Option<String>,
    pub attribute_type: String,
    pub is_visible: bool,
    pub is_variation: bool,
    pub position: u32,
}

/// Create product pricing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProductPricing {
    pub regular_price: Money,
    pub sale_price: Option<Money>,
    pub cost_price: Option<Money>,
    pub wholesale_price: Option<Money>,
    pub msrp: Option<Money>,
    pub tax_included: bool,
    pub sale_start_date: Option<Timestamp>,
    pub sale_end_date: Option<Timestamp>,
    pub price_tiers: Vec<PriceTier>,
}

/// Create product inventory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProductInventory {
    pub track_quantity: bool,
    pub quantity: Option<u32>,
    pub low_stock_threshold: Option<u32>,
    pub backorders_allowed: bool,
    pub manage_stock: bool,
    pub sold_individually: bool,
}

/// Create product shipping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProductShipping {
    pub requires_shipping: bool,
    pub weight: Option<f64>,
    pub dimensions: Option<ProductDimensions>,
    pub shipping_class_id: Option<ResourceId>,
    pub free_shipping: bool,
    pub separate_shipping: bool,
    pub shipping_cost: Option<Money>,
}

/// Create product SEO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProductSeo {
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,
    pub focus_keyword: Option<String>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub og_image: Option<String>,
    pub twitter_title: Option<String>,
    pub twitter_description: Option<String>,
    pub twitter_image: Option<String>,
}

impl CreateProductRequest {
    /// Create a builder for the request
    pub fn builder() -> CreateProductRequestBuilder {
        CreateProductRequestBuilder::default()
    }
}

/// Builder for CreateProductRequest
#[derive(Default)]
pub struct CreateProductRequestBuilder {
    sku: Option<String>,
    name: Option<String>,
    slug: Option<String>,
    description: Option<String>,
    short_description: Option<String>,
    product_type: Option<ProductType>,
    status: Option<ProductStatus>,
    visibility: Option<ProductVisibility>,
    condition: Option<ProductCondition>,
    brand_id: Option<ResourceId>,
    category_id: Option<ResourceId>,
    tags: Vec<String>,
    attributes: Vec<CreateProductAttribute>,
    pricing: Option<CreateProductPricing>,
    inventory: Option<CreateProductInventory>,
    shipping: Option<CreateProductShipping>,
    seo: Option<CreateProductSeo>,
    weight: Option<f64>,
    dimensions: Option<ProductDimensions>,
    tax_class_id: Option<ResourceId>,
    requires_shipping: bool,
    is_virtual: bool,
    is_downloadable: bool,
    reviews_allowed: bool,
    featured: bool,
    metadata: Option<Metadata>,
}

impl CreateProductRequestBuilder {
    pub fn sku(mut self, sku: impl Into<String>) -> Self {
        self.sku = Some(sku.into());
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn slug(mut self, slug: impl Into<String>) -> Self {
        self.slug = Some(slug.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn short_description(mut self, description: impl Into<String>) -> Self {
        self.short_description = Some(description.into());
        self
    }

    pub fn product_type(mut self, product_type: ProductType) -> Self {
        self.product_type = Some(product_type);
        self
    }

    pub fn status(mut self, status: ProductStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn visibility(mut self, visibility: ProductVisibility) -> Self {
        self.visibility = Some(visibility);
        self
    }

    pub fn condition(mut self, condition: ProductCondition) -> Self {
        self.condition = Some(condition);
        self
    }

    pub fn brand_id(mut self, brand_id: impl Into<ResourceId>) -> Self {
        self.brand_id = Some(brand_id.into());
        self
    }

    pub fn category_id(mut self, category_id: impl Into<ResourceId>) -> Self {
        self.category_id = Some(category_id.into());
        self
    }

    pub fn add_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    pub fn add_attribute(mut self, attribute: CreateProductAttribute) -> Self {
        self.attributes.push(attribute);
        self
    }

    pub fn pricing(mut self, pricing: CreateProductPricing) -> Self {
        self.pricing = Some(pricing);
        self
    }

    pub fn inventory(mut self, inventory: CreateProductInventory) -> Self {
        self.inventory = Some(inventory);
        self
    }

    pub fn shipping(mut self, shipping: CreateProductShipping) -> Self {
        self.shipping = Some(shipping);
        self
    }

    pub fn seo(mut self, seo: CreateProductSeo) -> Self {
        self.seo = Some(seo);
        self
    }

    pub fn weight(mut self, weight: f64) -> Self {
        self.weight = Some(weight);
        self
    }

    pub fn dimensions(mut self, dimensions: ProductDimensions) -> Self {
        self.dimensions = Some(dimensions);
        self
    }

    pub fn tax_class_id(mut self, tax_class_id: impl Into<ResourceId>) -> Self {
        self.tax_class_id = Some(tax_class_id.into());
        self
    }

    pub fn requires_shipping(mut self, requires_shipping: bool) -> Self {
        self.requires_shipping = requires_shipping;
        self
    }

    pub fn is_virtual(mut self, is_virtual: bool) -> Self {
        self.is_virtual = is_virtual;
        self
    }

    pub fn is_downloadable(mut self, is_downloadable: bool) -> Self {
        self.is_downloadable = is_downloadable;
        self
    }

    pub fn reviews_allowed(mut self, reviews_allowed: bool) -> Self {
        self.reviews_allowed = reviews_allowed;
        self
    }

    pub fn featured(mut self, featured: bool) -> Self {
        self.featured = featured;
        self
    }

    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn build(self) -> Result<CreateProductRequest, String> {
        Ok(CreateProductRequest {
            sku: self.sku.ok_or("sku is required")?,
            name: self.name.ok_or("name is required")?,
            slug: self.slug,
            description: self.description,
            short_description: self.short_description,
            product_type: self.product_type.unwrap_or(ProductType::Physical),
            status: self.status,
            visibility: self.visibility,
            condition: self.condition,
            brand_id: self.brand_id,
            category_id: self.category_id,
            tags: self.tags,
            attributes: self.attributes,
            pricing: self.pricing.ok_or("pricing is required")?,
            inventory: self.inventory,
            shipping: self.shipping,
            seo: self.seo,
            weight: self.weight,
            dimensions: self.dimensions,
            tax_class_id: self.tax_class_id,
            requires_shipping: self.requires_shipping,
            is_virtual: self.is_virtual,
            is_downloadable: self.is_downloadable,
            reviews_allowed: self.reviews_allowed,
            featured: self.featured,
            metadata: self.metadata,
        })
    }
}

/// Update product request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateProductRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ProductStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<ProductVisibility>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<ProductCondition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_id: Option<ResourceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<ResourceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing: Option<CreateProductPricing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory: Option<CreateProductInventory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CreateProductShipping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seo: Option<CreateProductSeo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<ProductDimensions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featured: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// Product list filters
#[derive(Debug, Clone, Default, Serialize)]
pub struct ProductListFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ProductStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<ProductType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<ProductVisibility>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<ProductCondition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_id: Option<ResourceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<ResourceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featured: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_price: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_price: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_stock: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<Timestamp>,
}