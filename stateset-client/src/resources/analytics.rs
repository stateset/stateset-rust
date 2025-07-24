//! Analytics API client implementation

use crate::{Client, request::{ListRequestBuilder, SortOrder}};
use stateset_core::{Error, Result, traits::ListResponse, types::ResourceId};
use stateset_models::analytics::{
    CreateAnalyticsReportRequest, AnalyticsReport, AnalyticsListFilters, ReportType, ChartType,
    AnalyticsQueryRequest, AnalyticsQueryResponse, SalesAnalytics, CustomerAnalytics,
    ProductAnalytics, InventoryAnalytics, RealtimeDashboard,
};

/// Analytics API client
pub struct AnalyticsClient {
    client: Client,
}

impl AnalyticsClient {
    /// Create a new analytics client
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a new analytics report
    pub async fn create_report(&self, request: CreateAnalyticsReportRequest) -> Result<AnalyticsReport> {
        self.client.post("/api/v1/analytics/reports", &request).await
    }

    /// Get an analytics report by ID
    pub async fn get_report(&self, id: impl Into<ResourceId>) -> Result<AnalyticsReport> {
        let path = format!("/api/v1/analytics/reports/{}", id.into());
        self.client.get(&path).await
    }

    /// Update an analytics report
    pub async fn update_report(&self, id: impl Into<ResourceId>, request: CreateAnalyticsReportRequest) -> Result<AnalyticsReport> {
        let path = format!("/api/v1/analytics/reports/{}", id.into());
        self.client.patch(&path, &request).await
    }

    /// Delete an analytics report
    pub async fn delete_report(&self, id: impl Into<ResourceId>) -> Result<()> {
        let path = format!("/api/v1/analytics/reports/{}", id.into());
        self.client.delete_no_content(&path).await
    }

    /// Execute an analytics query
    pub async fn query(&self, request: AnalyticsQueryRequest) -> Result<AnalyticsQueryResponse> {
        self.client.post("/api/v1/analytics/query", &request).await
    }

    /// Get sales analytics
    pub async fn sales_analytics(&self, date_range: Option<(String, String)>) -> Result<SalesAnalytics> {
        let mut path = "/api/v1/analytics/sales".to_string();
        if let Some((start, end)) = date_range {
            path.push_str(&format!("?start_date={}&end_date={}", start, end));
        }
        self.client.get(&path).await
    }

    /// Get customer analytics
    pub async fn customer_analytics(&self, date_range: Option<(String, String)>) -> Result<CustomerAnalytics> {
        let mut path = "/api/v1/analytics/customers".to_string();
        if let Some((start, end)) = date_range {
            path.push_str(&format!("?start_date={}&end_date={}", start, end));
        }
        self.client.get(&path).await
    }

    /// Get product analytics
    pub async fn product_analytics(&self, date_range: Option<(String, String)>) -> Result<ProductAnalytics> {
        let mut path = "/api/v1/analytics/products".to_string();
        if let Some((start, end)) = date_range {
            path.push_str(&format!("?start_date={}&end_date={}", start, end));
        }
        self.client.get(&path).await
    }

    /// Get inventory analytics
    pub async fn inventory_analytics(&self, date_range: Option<(String, String)>) -> Result<InventoryAnalytics> {
        let mut path = "/api/v1/analytics/inventory".to_string();
        if let Some((start, end)) = date_range {
            path.push_str(&format!("?start_date={}&end_date={}", start, end));
        }
        self.client.get(&path).await
    }

    /// Get real-time dashboard
    pub async fn realtime_dashboard(&self) -> Result<RealtimeDashboard> {
        self.client.get("/api/v1/analytics/dashboard/realtime").await
    }

    /// List analytics reports
    pub fn list_reports(&self) -> AnalyticsListBuilder {
        AnalyticsListBuilder::new(self.client.clone())
    }

    /// Export analytics data
    pub async fn export(&self, report_id: impl Into<ResourceId>, format: &str) -> Result<Vec<u8>> {
        let path = format!("/api/v1/analytics/reports/{}/export?format={}", report_id.into(), format);
        // Note: This would need special handling for binary data
        let response = self.client.get::<serde_json::Value>(&path).await?;
        // Convert to bytes - this is a simplified implementation
        Ok(response.to_string().into_bytes())
    }

    /// Get revenue trends
    pub async fn revenue_trends(&self, period: &str) -> Result<serde_json::Value> {
        let path = format!("/api/v1/analytics/trends/revenue?period={}", period);
        self.client.get(&path).await
    }

    /// Get conversion funnel
    pub async fn conversion_funnel(&self, date_range: Option<(String, String)>) -> Result<serde_json::Value> {
        let mut path = "/api/v1/analytics/funnel/conversion".to_string();
        if let Some((start, end)) = date_range {
            path.push_str(&format!("?start_date={}&end_date={}", start, end));
        }
        self.client.get(&path).await
    }

    /// Get cohort analysis
    pub async fn cohort_analysis(&self, cohort_type: &str) -> Result<serde_json::Value> {
        let path = format!("/api/v1/analytics/cohort?type={}", cohort_type);
        self.client.get(&path).await
    }
}

pub struct AnalyticsListBuilder {
    client: Client,
    builder: ListRequestBuilder<AnalyticsListFilters>,
}

impl AnalyticsListBuilder {
    fn new(client: Client) -> Self {
        Self {
            client,
            builder: ListRequestBuilder::new(),
        }
    }

    pub fn report_type(mut self, report_type: ReportType) -> Self {
        self.builder.filters.report_type = Some(report_type);
        self
    }

    pub fn chart_type(mut self, chart_type: ChartType) -> Self {
        self.builder.filters.chart_type = Some(chart_type);
        self
    }

    pub fn realtime(mut self, realtime: bool) -> Self {
        self.builder.filters.is_realtime = Some(realtime);
        self
    }

    pub async fn execute(self) -> Result<ListResponse<AnalyticsReport>> {
        self.client
            .get_with_query("/api/v1/analytics/reports", &self.builder.build())
            .await
    }
}