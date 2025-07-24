//! Procedural macros for StateSet SDK
//!
//! This crate provides derive macros and attribute macros to reduce boilerplate.

use proc_macro::TokenStream;

/// Derive macro for API resources
///
/// Example:
/// ```ignore
/// #[derive(ApiResource)]
/// #[api(endpoint = "/api/v1/widgets", type_name = "widget")]
/// struct Widget {
///     id: ResourceId,
///     name: String,
/// }
/// ```
#[proc_macro_derive(ApiResource, attributes(api))]
pub fn derive_api_resource(_input: TokenStream) -> TokenStream {
    // In a real implementation, this would generate the ApiResource trait impl
    TokenStream::new()
}

/// Derive macro for request builders
///
/// Example:
/// ```ignore
/// #[derive(RequestBuilder)]
/// struct CreateWidgetRequest {
///     name: String,
///     #[builder(optional)]
///     description: Option<String>,
/// }
/// ```
#[proc_macro_derive(RequestBuilder, attributes(builder))]
pub fn derive_request_builder(_input: TokenStream) -> TokenStream {
    // In a real implementation, this would generate a builder struct
    TokenStream::new()
} 