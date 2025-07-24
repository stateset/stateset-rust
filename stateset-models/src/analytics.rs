//! Analytics models and types

use serde::{Deserialize, Serialize};
use stateset_core::{
    traits::{ApiResource, Identifiable},
    types::{Metadata, Money, ResourceId, Timestamp},
};
use std::collections::HashMap;

/// Analytics report type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportType {
    Sales,
    Revenue,
    Orders,
    Products,
    Customers,
    Inventory,
    Returns,
    Shipments,
    Conversion,
    Funnel,
    Cohort,
    Geographic,
    TimeSeriesRevenue,
    TimeSeriesOrders,
    TopProducts,
    TopCustomers,
    CustomerLifetimeValue,
    CartAbandonment,
    Custom,
}

/// Analytics period enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnalyticsPeriod {
    Hour,
    Day,
    Week,
    Month,
    Quarter,
    Year,
    Custom,
}

/// Chart type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChartType {
    Line,
    Bar,
    Pie,
    Donut,
    Area,
    Scatter,
    Heatmap,
    Funnel,
    Gauge,
    Table,
}

/// Analytics report model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsReport {
    pub id: ResourceId,
    pub name: String,
    pub description: Option<String>,
    pub report_type: ReportType,
    pub chart_type: ChartType,
    pub period: AnalyticsPeriod,
    pub start_date: Timestamp,
    pub end_date: Timestamp,
    pub filters: HashMap<String, serde_json::Value>,
    pub data: Vec<DataPoint>,
    pub summary: ReportSummary,
    pub metrics: Vec<Metric>,
    pub dimensions: Vec<Dimension>,
    pub visualization_config: VisualizationConfig,
    pub is_realtime: bool,
    pub refresh_interval: Option<u32>, // seconds
    pub generated_at: Timestamp,
    pub metadata: Option<Metadata>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl Identifiable for AnalyticsReport {
    type Id = ResourceId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl ApiResource for AnalyticsReport {
    const ENDPOINT: &'static str = "/api/v1/analytics/reports";
    const TYPE_NAME: &'static str = "analytics_report";
}

/// Data point for analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    pub timestamp: Timestamp,
    pub value: f64,
    pub label: Option<String>,
    pub dimensions: HashMap<String, String>,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Report summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSummary {
    pub total_value: f64,
    pub previous_period_value: Option<f64>,
    pub change_amount: Option<f64>,
    pub change_percentage: Option<f64>,
    pub trend: Option<String>, // "up", "down", "stable"
    pub period_comparison: Option<PeriodComparison>,
}

/// Period comparison data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodComparison {
    pub current_period: f64,
    pub previous_period: f64,
    pub year_over_year: Option<f64>,
    pub month_over_month: Option<f64>,
    pub week_over_week: Option<f64>,
}

/// Metric definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub value: f64,
    pub unit: Option<String>,
    pub format: Option<String>, // "currency", "percentage", "number"
    pub goal: Option<f64>,
    pub target: Option<f64>,
    pub benchmark: Option<f64>,
}

/// Dimension for data segmentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimension {
    pub name: String,
    pub display_name: String,
    pub values: Vec<String>,
}

/// Visualization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationConfig {
    pub chart_type: ChartType,
    pub colors: Vec<String>,
    pub show_legend: bool,
    pub show_grid: bool,
    pub show_tooltips: bool,
    pub x_axis_label: Option<String>,
    pub y_axis_label: Option<String>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub annotations: Vec<ChartAnnotation>,
}

/// Chart annotation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartAnnotation {
    pub x: f64,
    pub y: Option<f64>,
    pub text: String,
    pub color: Option<String>,
    pub style: Option<String>, // "line", "box", "arrow"
}

/// Sales analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesAnalytics {
    pub period_start: Timestamp,
    pub period_end: Timestamp,
    pub total_sales: Money,
    pub total_orders: u32,
    pub average_order_value: Money,
    pub gross_profit: Money,
    pub gross_margin: f64,
    pub conversion_rate: f64,
    pub sales_by_day: Vec<DailySales>,
    pub sales_by_product: Vec<ProductSales>,
    pub sales_by_region: Vec<RegionalSales>,
    pub top_customers: Vec<CustomerSales>,
    pub payment_methods: Vec<PaymentMethodSales>,
}

/// Daily sales data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailySales {
    pub date: Timestamp,
    pub sales: Money,
    pub orders: u32,
    pub average_order_value: Money,
    pub units_sold: u32,
}

/// Product sales data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSales {
    pub product_id: ResourceId,
    pub product_name: String,
    pub sku: String,
    pub units_sold: u32,
    pub revenue: Money,
    pub profit: Money,
    pub margin: f64,
    pub conversion_rate: f64,
}

/// Regional sales data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalSales {
    pub region: String,
    pub country: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
    pub sales: Money,
    pub orders: u32,
    pub customers: u32,
}

/// Customer sales data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerSales {
    pub customer_id: ResourceId,
    pub customer_name: Option<String>,
    pub customer_email: String,
    pub total_spent: Money,
    pub order_count: u32,
    pub average_order_value: Money,
    pub first_order_date: Timestamp,
    pub last_order_date: Timestamp,
}

/// Payment method sales data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethodSales {
    pub payment_method: String,
    pub sales: Money,
    pub orders: u32,
    pub percentage_of_total: f64,
}

/// Customer analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAnalytics {
    pub period_start: Timestamp,
    pub period_end: Timestamp,
    pub total_customers: u32,
    pub new_customers: u32,
    pub returning_customers: u32,
    pub customer_retention_rate: f64,
    pub customer_acquisition_cost: Money,
    pub customer_lifetime_value: Money,
    pub churn_rate: f64,
    pub cohort_analysis: Vec<CohortData>,
    pub customer_segments: Vec<CustomerSegment>,
}

/// Cohort data for customer analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortData {
    pub cohort_month: String,
    pub customers_count: u32,
    pub retention_rates: Vec<f64>, // by month
    pub revenue_per_customer: Vec<Money>,
}

/// Customer segment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerSegment {
    pub segment_name: String,
    pub customer_count: u32,
    pub percentage_of_total: f64,
    pub average_order_value: Money,
    pub total_revenue: Money,
    pub order_frequency: f64,
}

/// Product analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductAnalytics {
    pub period_start: Timestamp,
    pub period_end: Timestamp,
    pub total_products: u32,
    pub products_sold: u32,
    pub top_selling_products: Vec<ProductPerformance>,
    pub low_performing_products: Vec<ProductPerformance>,
    pub inventory_turnover: f64,
    pub average_selling_price: Money,
    pub product_categories: Vec<CategoryPerformance>,
}

/// Product performance data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductPerformance {
    pub product_id: ResourceId,
    pub product_name: String,
    pub sku: String,
    pub units_sold: u32,
    pub revenue: Money,
    pub profit_margin: f64,
    pub inventory_level: u32,
    pub days_in_stock: u32,
    pub return_rate: f64,
    pub review_rating: Option<f64>,
}

/// Category performance data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryPerformance {
    pub category_id: ResourceId,
    pub category_name: String,
    pub products_count: u32,
    pub units_sold: u32,
    pub revenue: Money,
    pub profit_margin: f64,
    pub conversion_rate: f64,
}

/// Inventory analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryAnalytics {
    pub period_start: Timestamp,
    pub period_end: Timestamp,
    pub total_inventory_value: Money,
    pub low_stock_items: u32,
    pub out_of_stock_items: u32,
    pub overstock_items: u32,
    pub inventory_turnover_ratio: f64,
    pub average_days_in_inventory: f64,
    pub carrying_cost: Money,
    pub inventory_by_location: Vec<LocationInventory>,
}

/// Location inventory data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationInventory {
    pub location_id: ResourceId,
    pub location_name: String,
    pub total_items: u32,
    pub total_value: Money,
    pub utilization_rate: f64,
}

/// Real-time dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeDashboard {
    pub current_visitors: u32,
    pub active_carts: u32,
    pub orders_today: u32,
    pub revenue_today: Money,
    pub conversion_rate_today: f64,
    pub top_products_today: Vec<ProductPerformance>,
    pub recent_orders: Vec<RecentOrder>,
    pub alert_notifications: Vec<AlertNotification>,
    pub last_updated: Timestamp,
}

/// Recent order for dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentOrder {
    pub order_id: ResourceId,
    pub order_number: String,
    pub customer_name: Option<String>,
    pub total: Money,
    pub status: String,
    pub created_at: Timestamp,
}

/// Alert notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertNotification {
    pub id: ResourceId,
    pub type_name: String, // "low_stock", "high_cart_abandonment", "revenue_target"
    pub severity: String,  // "info", "warning", "critical"
    pub title: String,
    pub message: String,
    pub action_url: Option<String>,
    pub created_at: Timestamp,
}

/// Create analytics report request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAnalyticsReportRequest {
    pub name: String,
    pub description: Option<String>,
    pub report_type: ReportType,
    pub chart_type: ChartType,
    pub period: AnalyticsPeriod,
    pub start_date: Timestamp,
    pub end_date: Timestamp,
    pub filters: HashMap<String, serde_json::Value>,
    pub dimensions: Vec<String>,
    pub metrics: Vec<String>,
    pub visualization_config: Option<VisualizationConfig>,
    pub is_realtime: bool,
    pub refresh_interval: Option<u32>,
    pub metadata: Option<Metadata>,
}

impl CreateAnalyticsReportRequest {
    /// Create a builder for the request
    pub fn builder() -> CreateAnalyticsReportRequestBuilder {
        CreateAnalyticsReportRequestBuilder::default()
    }
}

/// Builder for CreateAnalyticsReportRequest
#[derive(Default)]
pub struct CreateAnalyticsReportRequestBuilder {
    name: Option<String>,
    description: Option<String>,
    report_type: Option<ReportType>,
    chart_type: Option<ChartType>,
    period: Option<AnalyticsPeriod>,
    start_date: Option<Timestamp>,
    end_date: Option<Timestamp>,
    filters: HashMap<String, serde_json::Value>,
    dimensions: Vec<String>,
    metrics: Vec<String>,
    visualization_config: Option<VisualizationConfig>,
    is_realtime: bool,
    refresh_interval: Option<u32>,
    metadata: Option<Metadata>,
}

impl CreateAnalyticsReportRequestBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn report_type(mut self, report_type: ReportType) -> Self {
        self.report_type = Some(report_type);
        self
    }

    pub fn chart_type(mut self, chart_type: ChartType) -> Self {
        self.chart_type = Some(chart_type);
        self
    }

    pub fn period(mut self, period: AnalyticsPeriod) -> Self {
        self.period = Some(period);
        self
    }

    pub fn date_range(mut self, start_date: Timestamp, end_date: Timestamp) -> Self {
        self.start_date = Some(start_date);
        self.end_date = Some(end_date);
        self
    }

    pub fn add_filter(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.filters.insert(key.into(), value);
        self
    }

    pub fn add_dimension(mut self, dimension: impl Into<String>) -> Self {
        self.dimensions.push(dimension.into());
        self
    }

    pub fn add_metric(mut self, metric: impl Into<String>) -> Self {
        self.metrics.push(metric.into());
        self
    }

    pub fn visualization_config(mut self, config: VisualizationConfig) -> Self {
        self.visualization_config = Some(config);
        self
    }

    pub fn realtime(mut self, realtime: bool) -> Self {
        self.is_realtime = realtime;
        self
    }

    pub fn refresh_interval(mut self, interval: u32) -> Self {
        self.refresh_interval = Some(interval);
        self
    }

    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn build(self) -> Result<CreateAnalyticsReportRequest, String> {
        Ok(CreateAnalyticsReportRequest {
            name: self.name.ok_or("name is required")?,
            description: self.description,
            report_type: self.report_type.ok_or("report_type is required")?,
            chart_type: self.chart_type.unwrap_or(ChartType::Line),
            period: self.period.unwrap_or(AnalyticsPeriod::Day),
            start_date: self.start_date.ok_or("start_date is required")?,
            end_date: self.end_date.ok_or("end_date is required")?,
            filters: self.filters,
            dimensions: self.dimensions,
            metrics: self.metrics,
            visualization_config: self.visualization_config,
            is_realtime: self.is_realtime,
            refresh_interval: self.refresh_interval,
            metadata: self.metadata,
        })
    }
}

/// Analytics query request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsQueryRequest {
    pub report_type: ReportType,
    pub period: AnalyticsPeriod,
    pub start_date: Timestamp,
    pub end_date: Timestamp,
    pub filters: HashMap<String, serde_json::Value>,
    pub dimensions: Vec<String>,
    pub metrics: Vec<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// Analytics query response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsQueryResponse {
    pub query: AnalyticsQueryRequest,
    pub data: Vec<DataPoint>,
    pub summary: ReportSummary,
    pub total_records: u32,
    pub execution_time_ms: u64,
    pub cached: bool,
    pub generated_at: Timestamp,
}

/// Analytics list filters
#[derive(Debug, Clone, Default, Serialize)]
pub struct AnalyticsListFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_type: Option<ReportType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_type: Option<ChartType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<AnalyticsPeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_realtime: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<Timestamp>,
}