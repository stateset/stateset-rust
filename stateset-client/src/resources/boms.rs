//! BOMs API client implementation

use crate::{Client, request::{ListRequestBuilder, SortOrder}};
use stateset_core::{Error, Result, traits::ListResponse, types::ResourceId};
use stateset_models::bom::{
    CreateBomRequest, Bom, BomListFilters, BomStatus, BomType, UpdateBomRequest,
    BomCostAnalysis, BomExplosion,
};

/// BOMs API client
pub struct BomsClient {
    client: Client,
}

impl BomsClient {
    /// Create a new BOMs client
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a new BOM
    pub async fn create(&self, request: CreateBomRequest) -> Result<Bom> {
        self.client.post("/api/v1/boms", &request).await
    }

    /// Get a BOM by ID
    pub async fn get(&self, id: impl Into<ResourceId>) -> Result<Bom> {
        let path = format!("/api/v1/boms/{}", id.into());
        self.client.get(&path).await
    }

    /// Update a BOM
    pub async fn update(&self, id: impl Into<ResourceId>, request: UpdateBomRequest) -> Result<Bom> {
        let path = format!("/api/v1/boms/{}", id.into());
        self.client.patch(&path, &request).await
    }

    /// Delete a BOM
    pub async fn delete(&self, id: impl Into<ResourceId>) -> Result<()> {
        let path = format!("/api/v1/boms/{}", id.into());
        self.client.delete_no_content(&path).await
    }

    /// Approve a BOM
    pub async fn approve(&self, id: impl Into<ResourceId>) -> Result<Bom> {
        let path = format!("/api/v1/boms/{}/approve", id.into());
        self.client.post::<Bom, _>(&path, &serde_json::json!({})).await
    }

    /// Get BOM cost analysis
    pub async fn cost_analysis(&self, id: impl Into<ResourceId>) -> Result<BomCostAnalysis> {
        let path = format!("/api/v1/boms/{}/cost-analysis", id.into());
        self.client.get(&path).await
    }

    /// Get BOM explosion (where-used analysis)
    pub async fn explosion(&self, component_id: impl Into<ResourceId>) -> Result<BomExplosion> {
        let path = format!("/api/v1/boms/explosion/{}", component_id.into());
        self.client.get(&path).await
    }

    /// List BOMs
    pub fn list(&self) -> BomListBuilder {
        BomListBuilder::new(self.client.clone())
    }

    /// Get BOMs by product
    pub async fn by_product(&self, product_id: impl Into<ResourceId>) -> Result<Vec<Bom>> {
        let path = format!("/api/v1/boms/by-product/{}", product_id.into());
        self.client.get(&path).await
    }
}

pub struct BomListBuilder {
    client: Client,
    builder: ListRequestBuilder<BomListFilters>,
}

impl BomListBuilder {
    fn new(client: Client) -> Self {
        Self {
            client,
            builder: ListRequestBuilder::new(),
        }
    }

    pub fn status(mut self, status: BomStatus) -> Self {
        self.builder.filters.status = Some(status);
        self
    }

    pub fn bom_type(mut self, bom_type: BomType) -> Self {
        self.builder.filters.bom_type = Some(bom_type);
        self
    }

    pub fn product_id(mut self, product_id: impl Into<ResourceId>) -> Self {
        self.builder.filters.product_id = Some(product_id.into());
        self
    }

    pub async fn execute(self) -> Result<ListResponse<Bom>> {
        self.client
            .get_with_query("/api/v1/boms", &self.builder.build())
            .await
    }
}