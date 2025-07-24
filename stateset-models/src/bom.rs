//! Bill of Materials (BOM) models and types

use serde::{Deserialize, Serialize};
use stateset_core::{
    traits::{ApiResource, Identifiable},
    types::{Expandable, Metadata, Money, ResourceId, Timestamp},
};

/// BOM status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BomStatus {
    Draft,
    Active,
    Inactive,
    Obsolete,
    UnderReview,
    Approved,
    Rejected,
}

/// BOM type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BomType {
    Manufacturing,
    Engineering,
    Sales,
    Service,
    Planning,
    Costing,
}

/// Component type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentType {
    Raw,
    Purchased,
    Manufactured,
    Assembly,
    Subassembly,
    Phantom,
    Reference,
    Tool,
}

/// Bill of Materials model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bom {
    pub id: ResourceId,
    pub bom_number: String,
    pub name: String,
    pub description: Option<String>,
    pub status: BomStatus,
    pub bom_type: BomType,
    pub version: String,
    pub revision: u32,
    pub product_id: ResourceId,
    pub product: Option<Expandable<Product>>,
    pub components: Vec<BomComponent>,
    pub total_cost: Money,
    pub total_weight: Option<f64>,
    pub total_volume: Option<f64>,
    pub lead_time_days: Option<u32>,
    pub effective_date: Option<Timestamp>,
    pub expiry_date: Option<Timestamp>,
    pub created_by: ResourceId,
    pub creator: Option<Expandable<User>>,
    pub approved_by: Option<ResourceId>,
    pub approver: Option<Expandable<User>>,
    pub approved_at: Option<Timestamp>,
    pub parent_bom_id: Option<ResourceId>,
    pub parent_bom: Option<Expandable<Bom>>,
    pub assembly_instructions: Option<String>,
    pub quality_notes: Option<String>,
    pub metadata: Option<Metadata>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl Identifiable for Bom {
    type Id = ResourceId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl ApiResource for Bom {
    const ENDPOINT: &'static str = "/api/v1/boms";
    const TYPE_NAME: &'static str = "bom";
}

/// BOM component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BomComponent {
    pub id: ResourceId,
    pub component_id: ResourceId,
    pub component: Option<Expandable<Product>>,
    pub component_type: ComponentType,
    pub quantity: f64,
    pub unit_of_measure: String,
    pub unit_cost: Money,
    pub total_cost: Money,
    pub position: Option<String>,
    pub reference_designator: Option<String>,
    pub supplier_id: Option<ResourceId>,
    pub supplier: Option<Expandable<Supplier>>,
    pub lead_time_days: Option<u32>,
    pub minimum_quantity: Option<f64>,
    pub scrap_factor: Option<f64>,
    pub yield_factor: Option<f64>,
    pub substitute_components: Vec<BomSubstitute>,
    pub assembly_notes: Option<String>,
    pub is_critical: bool,
    pub is_optional: bool,
    pub effective_date: Option<Timestamp>,
    pub expiry_date: Option<Timestamp>,
    pub metadata: Option<Metadata>,
}

/// BOM substitute component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BomSubstitute {
    pub id: ResourceId,
    pub substitute_component_id: ResourceId,
    pub substitute_component: Option<Expandable<Product>>,
    pub priority: u32,
    pub ratio: f64,
    pub notes: Option<String>,
}

/// Product model (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: ResourceId,
    pub name: String,
    pub sku: String,
    pub unit_cost: Money,
    pub weight: Option<f64>,
    pub volume: Option<f64>,
}

/// Supplier model (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Supplier {
    pub id: ResourceId,
    pub name: String,
    pub supplier_code: String,
}

/// User model (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: ResourceId,
    pub name: String,
    pub email: String,
}

/// Create BOM request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBomRequest {
    pub name: String,
    pub description: Option<String>,
    pub bom_type: BomType,
    pub version: String,
    pub product_id: ResourceId,
    pub components: Vec<CreateBomComponent>,
    pub effective_date: Option<Timestamp>,
    pub expiry_date: Option<Timestamp>,
    pub parent_bom_id: Option<ResourceId>,
    pub assembly_instructions: Option<String>,
    pub quality_notes: Option<String>,
    pub metadata: Option<Metadata>,
}

/// Create BOM component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBomComponent {
    pub component_id: ResourceId,
    pub component_type: ComponentType,
    pub quantity: f64,
    pub unit_of_measure: String,
    pub unit_cost: Money,
    pub position: Option<String>,
    pub reference_designator: Option<String>,
    pub supplier_id: Option<ResourceId>,
    pub lead_time_days: Option<u32>,
    pub minimum_quantity: Option<f64>,
    pub scrap_factor: Option<f64>,
    pub yield_factor: Option<f64>,
    pub substitute_components: Vec<CreateBomSubstitute>,
    pub assembly_notes: Option<String>,
    pub is_critical: bool,
    pub is_optional: bool,
    pub effective_date: Option<Timestamp>,
    pub expiry_date: Option<Timestamp>,
    pub metadata: Option<Metadata>,
}

/// Create BOM substitute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBomSubstitute {
    pub substitute_component_id: ResourceId,
    pub priority: u32,
    pub ratio: f64,
    pub notes: Option<String>,
}

impl CreateBomRequest {
    /// Create a builder for the request
    pub fn builder() -> CreateBomRequestBuilder {
        CreateBomRequestBuilder::default()
    }
}

/// Builder for CreateBomRequest
#[derive(Default)]
pub struct CreateBomRequestBuilder {
    name: Option<String>,
    description: Option<String>,
    bom_type: Option<BomType>,
    version: Option<String>,
    product_id: Option<ResourceId>,
    components: Vec<CreateBomComponent>,
    effective_date: Option<Timestamp>,
    expiry_date: Option<Timestamp>,
    parent_bom_id: Option<ResourceId>,
    assembly_instructions: Option<String>,
    quality_notes: Option<String>,
    metadata: Option<Metadata>,
}

impl CreateBomRequestBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn bom_type(mut self, bom_type: BomType) -> Self {
        self.bom_type = Some(bom_type);
        self
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn product_id(mut self, product_id: impl Into<ResourceId>) -> Self {
        self.product_id = Some(product_id.into());
        self
    }

    pub fn add_component(mut self, component: CreateBomComponent) -> Self {
        self.components.push(component);
        self
    }

    pub fn effective_date(mut self, date: Timestamp) -> Self {
        self.effective_date = Some(date);
        self
    }

    pub fn expiry_date(mut self, date: Timestamp) -> Self {
        self.expiry_date = Some(date);
        self
    }

    pub fn parent_bom_id(mut self, bom_id: impl Into<ResourceId>) -> Self {
        self.parent_bom_id = Some(bom_id.into());
        self
    }

    pub fn assembly_instructions(mut self, instructions: impl Into<String>) -> Self {
        self.assembly_instructions = Some(instructions.into());
        self
    }

    pub fn quality_notes(mut self, notes: impl Into<String>) -> Self {
        self.quality_notes = Some(notes.into());
        self
    }

    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn build(self) -> Result<CreateBomRequest, String> {
        Ok(CreateBomRequest {
            name: self.name.ok_or("name is required")?,
            description: self.description,
            bom_type: self.bom_type.unwrap_or(BomType::Manufacturing),
            version: self.version.unwrap_or_else(|| "1.0".to_string()),
            product_id: self.product_id.ok_or("product_id is required")?,
            components: self.components,
            effective_date: self.effective_date,
            expiry_date: self.expiry_date,
            parent_bom_id: self.parent_bom_id,
            assembly_instructions: self.assembly_instructions,
            quality_notes: self.quality_notes,
            metadata: self.metadata,
        })
    }
}

/// Update BOM request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateBomRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<BomStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assembly_instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// BOM costing analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BomCostAnalysis {
    pub bom_id: ResourceId,
    pub total_material_cost: Money,
    pub total_labor_cost: Money,
    pub total_overhead_cost: Money,
    pub total_cost: Money,
    pub cost_breakdown: Vec<BomCostBreakdown>,
    pub cost_rollup: Vec<BomCostRollup>,
    pub analysis_date: Timestamp,
}

/// BOM cost breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BomCostBreakdown {
    pub component_id: ResourceId,
    pub component_name: String,
    pub quantity: f64,
    pub unit_cost: Money,
    pub total_cost: Money,
    pub percentage_of_total: f64,
    pub cost_category: String,
}

/// BOM cost rollup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BomCostRollup {
    pub level: u32,
    pub assembly_id: ResourceId,
    pub assembly_name: String,
    pub quantity: f64,
    pub unit_cost: Money,
    pub total_cost: Money,
}

/// BOM explosion (where-used analysis)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BomExplosion {
    pub component_id: ResourceId,
    pub used_in_boms: Vec<BomUsage>,
    pub total_quantity: f64,
    pub analysis_date: Timestamp,
}

/// BOM usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BomUsage {
    pub bom_id: ResourceId,
    pub bom_name: String,
    pub product_id: ResourceId,
    pub product_name: String,
    pub quantity_per_assembly: f64,
    pub level: u32,
}

/// BOM list filters
#[derive(Debug, Clone, Default, Serialize)]
pub struct BomListFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<BomStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bom_type: Option<BomType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<ResourceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_after: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_before: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<Timestamp>,
}