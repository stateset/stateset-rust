//! Request building utilities

use serde::Serialize;
use stateset_core::types::{QueryParams, SortDirection};
use std::collections::HashMap;

/// Options for building requests with query parameters
#[derive(Debug, Clone, Default)]
pub struct RequestOptions {
    pub query_params: QueryParams,
    pub custom_params: HashMap<String, String>,
}

impl RequestOptions {
    /// Create new request options
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the limit
    pub fn limit(mut self, limit: u32) -> Self {
        self.query_params.limit = Some(limit);
        self
    }

    /// Set the page
    pub fn page(mut self, page: u32) -> Self {
        self.query_params.page = Some(page);
        self
    }

    /// Set the cursor
    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.query_params.cursor = Some(cursor.into());
        self
    }

    /// Set sort field
    pub fn sort_by(mut self, field: impl Into<String>) -> Self {
        self.query_params.sort_by = Some(field.into());
        self
    }

    /// Set sort direction
    pub fn sort_direction(mut self, direction: SortDirection) -> Self {
        self.query_params.sort_direction = Some(direction);
        self
    }

    /// Add fields to expand
    pub fn expand(mut self, fields: Vec<String>) -> Self {
        self.query_params.expand = Some(fields);
        self
    }

    /// Add a custom parameter
    pub fn param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom_params.insert(key.into(), value.into());
        self
    }

    /// Build into a serializable query structure
    pub fn build(self) -> impl Serialize {
        QueryBuilder {
            base: self.query_params,
            custom: self.custom_params,
        }
    }
}

#[derive(Serialize)]
struct QueryBuilder {
    #[serde(flatten)]
    base: QueryParams,
    #[serde(flatten)]
    custom: HashMap<String, String>,
}

/// Builder for list requests
#[derive(Clone)]
pub struct ListRequestBuilder<T> {
    options: RequestOptions,
    filters: T,
}

impl<T: Default> ListRequestBuilder<T> {
    /// Create a new list request builder
    pub fn new() -> Self {
        Self {
            options: RequestOptions::new(),
            filters: T::default(),
        }
    }
}

impl<T> ListRequestBuilder<T> {
    /// Set the limit
    pub fn limit(mut self, limit: u32) -> Self {
        self.options = self.options.limit(limit);
        self
    }

    /// Set the page
    pub fn page(mut self, page: u32) -> Self {
        self.options = self.options.page(page);
        self
    }

    /// Set the cursor
    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.options = self.options.cursor(cursor);
        self
    }

    /// Set sort field
    pub fn sort_by(mut self, field: impl Into<String>) -> Self {
        self.options = self.options.sort_by(field);
        self
    }

    /// Set sort direction
    pub fn sort_direction(mut self, direction: SortDirection) -> Self {
        self.options = self.options.sort_direction(direction);
        self
    }

    /// Add fields to expand
    pub fn expand(mut self, fields: Vec<String>) -> Self {
        self.options = self.options.expand(fields);
        self
    }

    /// Apply filters
    pub fn with_filters(mut self, filters: T) -> Self {
        self.filters = filters;
        self
    }

    /// Build the request
    pub fn build(self) -> (RequestOptions, T) {
        (self.options, self.filters)
    }
} 