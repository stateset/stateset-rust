//! Request building utilities

use stateset_core::{Error, Result};
use serde::Serialize;
use std::collections::HashMap;

/// Options for list requests
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct ListOptions {
    pub limit: Option<u32>,
    pub page: Option<u32>,
    pub offset: Option<u32>,
    pub cursor: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<SortOrder>,
}

/// Sort order options
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

impl ListOptions {
    /// Create new list options
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the page limit
    pub fn limit(mut self, limit: u32) -> Result<Self> {
        if limit == 0 {
            return Err(Error::validation_field(
                "Limit must be greater than 0",
                "limit",
            ));
        }
        if limit > 1000 {
            return Err(Error::validation_field(
                "Limit cannot exceed 1000",
                "limit",
            ));
        }
        self.limit = Some(limit);
        Ok(self)
    }

    /// Set the page number
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// Set the offset for pagination
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Set the cursor for cursor-based pagination
    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    /// Set the sort field
    pub fn sort_by(mut self, field: impl Into<String>) -> Self {
        self.sort_by = Some(field.into());
        self
    }

    /// Set the sort order
    pub fn sort_order(mut self, order: SortOrder) -> Self {
        self.sort_order = Some(order);
        self
    }

    /// Convert to query parameters
    pub fn to_query_params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        
        if let Some(limit) = self.limit {
            params.insert("limit".to_string(), limit.to_string());
        }
        
        if let Some(page) = self.page {
            params.insert("page".to_string(), page.to_string());
        }

        if let Some(offset) = self.offset {
            params.insert("offset".to_string(), offset.to_string());
        }
        
        if let Some(cursor) = &self.cursor {
            params.insert("cursor".to_string(), cursor.clone());
        }
        
        if let Some(sort_by) = &self.sort_by {
            params.insert("sort_by".to_string(), sort_by.clone());
        }
        
        if let Some(sort_order) = &self.sort_order {
            let order_str = match sort_order {
                SortOrder::Asc => "asc",
                SortOrder::Desc => "desc",
            };
            params.insert("sort_order".to_string(), order_str.to_string());
        }
        
        params
    }

    /// Validate the options
    pub fn validate(&self) -> Result<()> {
        // Can't use both cursor and page pagination
        if self.cursor.is_some() && self.page.is_some() {
            return Err(Error::validation(
                "Cannot use both cursor and page pagination simultaneously"
            ));
        }

        // Sort order requires sort_by
        if self.sort_order.is_some() && self.sort_by.is_none() {
            return Err(Error::validation(
                "sort_order requires sort_by to be specified"
            ));
        }

        Ok(())
    }
}

/// Builder for list requests with filters
#[derive(Debug, Clone)]
pub struct ListRequestBuilder<F> {
    options: ListOptions,
    filters: Option<F>,
}

impl<F> ListRequestBuilder<F>
where
    F: Default + Clone,
{
    /// Create a new list request builder
    pub fn new() -> Self {
        Self {
            options: ListOptions::default(),
            filters: None,
        }
    }

    /// Set the page limit with validation
    pub fn limit(mut self, limit: u32) -> Self {
        if let Ok(new_options) = self.options.clone().limit(limit) {
            self.options = new_options;
        }
        self
    }

    /// Set the page number
    pub fn page(mut self, page: u32) -> Self {
        self.options = self.options.page(page);
        self
    }

    /// Set the cursor for pagination
    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.options = self.options.cursor(cursor);
        self
    }

    /// Set the sort field and order
    pub fn sort(mut self, field: impl Into<String>, order: SortOrder) -> Self {
        self.options = self.options.sort_by(field).sort_order(order);
        self
    }

    /// Set the offset for pagination
    pub fn offset(mut self, offset: u32) -> Self {
        self.options = self.options.offset(offset);
        self
    }

    /// Set sorting by field and order (alias for sort)
    pub fn sort_by(mut self, field: impl Into<String>, order: SortOrder) -> Self {
        self.sort(field, order)
    }

    /// Add filters
    pub fn with_filters(mut self, filters: F) -> Self {
        self.filters = Some(filters);
        self
    }

    /// Get mutable access to filters, creating default if needed
    pub fn filters_mut(&mut self) -> &mut F {
        if self.filters.is_none() {
            self.filters = Some(F::default());
        }
        self.filters.as_mut().unwrap()
    }

    /// Get reference to filters
    pub fn filters(&self) -> Option<&F> {
        self.filters.as_ref()
    }

    /// Build the request options and filters
    pub fn build(self) -> Result<(ListOptions, F)> {
        self.options.validate()?;
        
        let filters = self.filters.unwrap_or_default();
        Ok((self.options, filters))
    }
}

impl<F> Default for ListRequestBuilder<F>
where
    F: Default + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating resources with validation
#[derive(Debug, Clone)]
pub struct CreateRequestBuilder<T> {
    data: Option<T>,
    validate_before_send: bool,
}

impl<T> CreateRequestBuilder<T> {
    /// Create a new create request builder
    pub fn new() -> Self {
        Self {
            data: None,
            validate_before_send: true,
        }
    }

    /// Set the data to create
    pub fn data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    /// Enable or disable validation before sending
    pub fn validate(mut self, enabled: bool) -> Self {
        self.validate_before_send = enabled;
        self
    }

    /// Build the request
    pub fn build(self) -> Result<T> {
        let data = self.data.ok_or_else(|| {
            Error::validation("Request data is required")
        })?;

        // In a real implementation, we might validate the data here
        // if self.validate_before_send { ... }

        Ok(data)
    }
}

impl<T> Default for CreateRequestBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for updating resources
#[derive(Debug, Clone)]
pub struct UpdateRequestBuilder<T> {
    data: Option<T>,
    merge_strategy: MergeStrategy,
}

/// Strategy for merging updates
#[derive(Debug, Clone)]
pub enum MergeStrategy {
    /// Replace all fields (default)
    Replace,
    /// Merge only non-null fields
    Merge,
    /// Use PATCH semantics
    Patch,
}

impl Default for MergeStrategy {
    fn default() -> Self {
        MergeStrategy::Merge
    }
}

impl<T> UpdateRequestBuilder<T> {
    /// Create a new update request builder
    pub fn new() -> Self {
        Self {
            data: None,
            merge_strategy: MergeStrategy::default(),
        }
    }

    /// Set the data to update
    pub fn data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    /// Set the merge strategy
    pub fn merge_strategy(mut self, strategy: MergeStrategy) -> Self {
        self.merge_strategy = strategy;
        self
    }

    /// Build the request
    pub fn build(self) -> Result<(T, MergeStrategy)> {
        let data = self.data.ok_or_else(|| {
            Error::validation("Update data is required")
        })?;

        Ok((data, self.merge_strategy))
    }
}

impl<T> Default for UpdateRequestBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_options_validation() {
        let options = ListOptions::new();
        assert!(options.validate().is_ok());

        // Test limit validation
        let result = ListOptions::new().limit(0);
        assert!(result.is_err());

        let result = ListOptions::new().limit(1001);
        assert!(result.is_err());

        let result = ListOptions::new().limit(100);
        assert!(result.is_ok());
    }

    #[test]
    fn test_conflicting_pagination() {
        let options = ListOptions::new()
            .page(1)
            .cursor("test".to_string());
        
        assert!(options.validate().is_err());
    }

    #[test]
    fn test_sort_validation() {
        let options = ListOptions::new()
            .sort_order(SortOrder::Asc);
        
        assert!(options.validate().is_err());

        let options = ListOptions::new()
            .sort_by("created_at")
            .sort_order(SortOrder::Desc);
        
        assert!(options.validate().is_ok());
    }

    #[test]
    fn test_query_params_conversion() {
        let options = ListOptions::new()
            .limit(50).unwrap()
            .page(2)
            .sort_by("name")
            .sort_order(SortOrder::Asc);

        let params = options.to_query_params();
        
        assert_eq!(params.get("limit"), Some(&"50".to_string()));
        assert_eq!(params.get("page"), Some(&"2".to_string()));
        assert_eq!(params.get("sort_by"), Some(&"name".to_string()));
        assert_eq!(params.get("sort_order"), Some(&"asc".to_string()));
    }

    #[derive(Default, Clone)]
    struct TestFilters {
        status: Option<String>,
    }

    #[test]
    fn test_list_request_builder() {
        let builder = ListRequestBuilder::<TestFilters>::new()
            .limit(25)
            .page(1)
            .sort("created_at".to_string(), SortOrder::Desc);

        let result = builder.build();
        assert!(result.is_ok());

        let (options, _filters) = result.unwrap();
        assert_eq!(options.limit, Some(25));
        assert_eq!(options.page, Some(1));
    }

    #[test]
    fn test_create_request_builder() {
        let builder = CreateRequestBuilder::new()
            .data("test data".to_string());

        let result = builder.build();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test data");

        // Test missing data
        let builder = CreateRequestBuilder::<String>::new();
        let result = builder.build();
        assert!(result.is_err());
    }

    #[test]
    fn test_update_request_builder() {
        let builder = UpdateRequestBuilder::new()
            .data("updated data".to_string())
            .merge_strategy(MergeStrategy::Patch);

        let result = builder.build();
        assert!(result.is_ok());

        let (data, strategy) = result.unwrap();
        assert_eq!(data, "updated data");
        assert!(matches!(strategy, MergeStrategy::Patch));
    }
} 