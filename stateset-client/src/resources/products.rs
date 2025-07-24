//! Products API client implementation

use crate::{Client, request::{ListRequestBuilder, SortOrder}};
use stateset_core::{Error, Result, traits::ListResponse, types::ResourceId};
use stateset_models::product::{
    CreateProductRequest, Product, ProductListFilters, ProductStatus, ProductType,
    UpdateProductRequest,
};

/// Products API client
pub struct ProductsClient {
    client: Client,
}

impl ProductsClient {
    /// Create a new products client
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a new product
    pub async fn create(&self, request: CreateProductRequest) -> Result<Product> {
        self.client.post("/api/v1/products", &request).await
    }

    /// Get a product by ID
    pub async fn get(&self, id: impl Into<ResourceId>) -> Result<Product> {
        let path = format!("/api/v1/products/{}", id.into());
        self.client.get(&path).await
    }

    /// Get a product by SKU
    pub async fn get_by_sku(&self, sku: &str) -> Result<Product> {
        let path = format!("/api/v1/products/sku/{}", sku);
        self.client.get(&path).await
    }

    /// Update a product
    pub async fn update(&self, id: impl Into<ResourceId>, request: UpdateProductRequest) -> Result<Product> {
        let path = format!("/api/v1/products/{}", id.into());
        self.client.patch(&path, &request).await
    }

    /// Delete a product
    pub async fn delete(&self, id: impl Into<ResourceId>) -> Result<()> {
        let path = format!("/api/v1/products/{}", id.into());
        self.client.delete_no_content(&path).await
    }

    /// Duplicate a product
    pub async fn duplicate(&self, id: impl Into<ResourceId>) -> Result<Product> {
        let path = format!("/api/v1/products/{}/duplicate", id.into());
        self.client.post::<Product, _>(&path, &serde_json::json!({})).await
    }

    /// List products
    pub fn list(&self) -> ProductListBuilder {
        ProductListBuilder::new(self.client.clone())
    }

    /// Search products
    pub async fn search(&self, query: &str) -> Result<Vec<Product>> {
        let path = format!("/api/v1/products/search?q={}", urlencoding::encode(query));
        self.client.get(&path).await
    }

    /// Get products by category
    pub async fn by_category(&self, category_id: impl Into<ResourceId>) -> Result<Vec<Product>> {
        let path = format!("/api/v1/products/by-category/{}", category_id.into());
        self.client.get(&path).await
    }

    /// Get products by brand
    pub async fn by_brand(&self, brand_id: impl Into<ResourceId>) -> Result<Vec<Product>> {
        let path = format!("/api/v1/products/by-brand/{}", brand_id.into());
        self.client.get(&path).await
    }
}

pub struct ProductListBuilder {
    client: Client,
    builder: ListRequestBuilder<ProductListFilters>,
}

impl ProductListBuilder {
    fn new(client: Client) -> Self {
        Self {
            client,
            builder: ListRequestBuilder::new(),
        }
    }

    pub fn status(mut self, status: ProductStatus) -> Self {
        self.builder.filters.status = Some(status);
        self
    }

    pub fn product_type(mut self, product_type: ProductType) -> Self {
        self.builder.filters.product_type = Some(product_type);
        self
    }

    pub fn category_id(mut self, category_id: impl Into<ResourceId>) -> Self {
        self.builder.filters.category_id = Some(category_id.into());
        self
    }

    pub fn brand_id(mut self, brand_id: impl Into<ResourceId>) -> Self {
        self.builder.filters.brand_id = Some(brand_id.into());
        self
    }

    pub fn featured(mut self, featured: bool) -> Self {
        self.builder.filters.featured = Some(featured);
        self
    }

    pub async fn execute(self) -> Result<ListResponse<Product>> {
        self.client
            .get_with_query("/api/v1/products", &self.builder.build())
            .await
    }
}