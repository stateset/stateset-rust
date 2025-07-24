//! Core traits for StateSet API resources

use serde::{Deserialize, Serialize};

/// Trait for resources that have a unique identifier
pub trait Identifiable {
    /// The type of the resource ID
    type Id: Clone + Send + Sync;

    /// Get the resource ID
    fn id(&self) -> &Self::Id;
}

/// Trait for API resources
pub trait ApiResource: Serialize + for<'de> Deserialize<'de> + Send + Sync {
    /// The API endpoint path for this resource
    const ENDPOINT: &'static str;

    /// The resource type name
    const TYPE_NAME: &'static str;
}

/// Trait for resources that support listing
pub trait ListableResource: ApiResource {
    /// The type used for list requests
    type ListRequest: Default + Send;

    /// The type returned when listing
    type ListResponse: Send;
}

/// Trait for paginated responses
pub trait Paginated {
    /// The type of items in the page
    type Item;

    /// Get the items in this page
    fn items(&self) -> &[Self::Item];

    /// Check if there are more pages
    fn has_more(&self) -> bool;

    /// Get the next page token/cursor if available
    fn next_page_token(&self) -> Option<&str>;

    /// Get the total count if available
    fn total_count(&self) -> Option<usize>;
}

/// Trait for resources that support create operations
pub trait CreatableResource: ApiResource {
    /// The type used for create requests
    type CreateRequest: Serialize + Send;
}

/// Trait for resources that support update operations
pub trait UpdatableResource: ApiResource + Identifiable {
    /// The type used for update requests
    type UpdateRequest: Serialize + Send;
}

/// Trait for resources that support delete operations
pub trait DeletableResource: ApiResource + Identifiable {}

/// Trait for resources that support bulk operations
pub trait BulkOperations: ApiResource {
    /// The type used for bulk create requests
    type BulkCreateRequest: Serialize + Send;

    /// The type returned from bulk create operations
    type BulkCreateResponse: for<'de> Deserialize<'de> + Send;

    /// The type used for bulk update requests
    type BulkUpdateRequest: Serialize + Send;

    /// The type returned from bulk update operations
    type BulkUpdateResponse: for<'de> Deserialize<'de> + Send;
}

/// Trait for resources that support search operations
pub trait SearchableResource: ApiResource {
    /// The type used for search requests
    type SearchRequest: Serialize + Send;

    /// The type returned from search operations
    type SearchResponse: for<'de> Deserialize<'de> + Send;
}

/// Trait for expandable fields in API responses
pub trait Expandable<T> {
    /// Check if the field is expanded
    fn is_expanded(&self) -> bool;

    /// Get the expanded value if available
    fn expanded(&self) -> Option<&T>;

    /// Get the ID reference
    fn id_ref(&self) -> Option<&str>;
}

/// Standard list response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListResponse<T> {
    /// The list of items
    pub data: Vec<T>,
    /// Whether there are more items
    pub has_more: bool,
    /// The total count of items (if available)
    pub total_count: Option<usize>,
    /// The next page cursor/token
    pub next_page: Option<String>,
}

impl<T> Paginated for ListResponse<T> {
    type Item = T;

    fn items(&self) -> &[Self::Item] {
        &self.data
    }

    fn has_more(&self) -> bool {
        self.has_more
    }

    fn next_page_token(&self) -> Option<&str> {
        self.next_page.as_deref()
    }

    fn total_count(&self) -> Option<usize> {
        self.total_count
    }
} 