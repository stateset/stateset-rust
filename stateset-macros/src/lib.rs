//! Macros for StateSet SDK
//! 
//! This crate provides procedural macros to reduce boilerplate and improve
//! the developer experience when working with the StateSet SDK.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Field, Type, PathArguments, GenericArgument};

/// Derive macro for creating builder patterns
/// 
/// This macro generates a builder struct and implementation for any struct.
/// It supports optional fields and provides a fluent API.
///
/// # Example
///
/// ```rust
/// use stateset_macros::Builder;
///
/// #[derive(Builder)]
/// struct CreateOrderRequest {
///     customer_id: String,
///     #[builder(optional)]
///     notes: Option<String>,
///     items: Vec<OrderItem>,
/// }
/// ```
///
/// This generates:
/// - `CreateOrderRequestBuilder` struct
/// - `builder()` method on the original struct
/// - Fluent setter methods for each field
/// - `build()` method that validates and constructs the struct
#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    let name = &input.ident;
    let builder_name = format!("{}Builder", name);
    let builder_ident = syn::Ident::new(&builder_name, name.span());
    
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Builder can only be derived for structs with named fields"),
        },
        _ => panic!("Builder can only be derived for structs"),
    };

    let builder_fields = fields.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        
        // Check if field is marked as optional or is already Option<T>
        let is_optional = is_optional_field(field);
        
        if is_optional {
            quote! { #name: #ty }
        } else {
            quote! { #name: Option<#ty> }
        }
    });

    let builder_methods = fields.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        
        // Generate setter method
        if is_optional_field(field) {
            quote! {
                pub fn #name(mut self, #name: impl Into<#ty>) -> Self {
                    self.#name = #name.into();
                    self
                }
            }
        } else {
            quote! {
                pub fn #name(mut self, #name: impl Into<#ty>) -> Self {
                    self.#name = Some(#name.into());
                    self
                }
            }
        }
    });

    let build_assignments = fields.iter().map(|field| {
        let name = &field.ident;
        
        if is_optional_field(field) {
            quote! { #name: self.#name }
        } else {
            let name_str = name.as_ref().unwrap().to_string();
            quote! {
                #name: self.#name.ok_or_else(|| {
                    stateset_core::Error::validation(format!("Field '{}' is required", #name_str))
                })?
            }
        }
    });

    let default_values = fields.iter().map(|field| {
        let name = &field.ident;
        
        if is_optional_field(field) {
            quote! { #name: Default::default() }
        } else {
            quote! { #name: None }
        }
    });

    let expanded = quote! {
        #[derive(Debug, Default)]
        pub struct #builder_ident {
            #(#builder_fields,)*
        }

        impl #builder_ident {
            #(#builder_methods)*

            pub fn build(self) -> stateset_core::Result<#name> {
                Ok(#name {
                    #(#build_assignments,)*
                })
            }
        }

        impl #name {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    #(#default_values,)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

/// Check if a field is optional (either marked with #[builder(optional)] or is Option<T>)
fn is_optional_field(field: &Field) -> bool {
    // Check for #[builder(optional)] attribute
    for attr in &field.attrs {
        if attr.path().is_ident("builder") {
            if let Ok(meta) = attr.parse_args::<syn::Ident>() {
                if meta == "optional" {
                    return true;
                }
            }
        }
    }

    // Check if type is Option<T>
    is_option_type(&field.ty)
}

/// Check if a type is Option<T>
fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                return true;
            }
        }
    }
    false
}

/// Derive macro for creating API request structs
///
/// This macro adds common functionality to API request structs including
/// validation and serialization.
#[proc_macro_derive(ApiRequest)]
pub fn derive_api_request(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl #name {
            /// Validate the request before sending
            pub fn validate(&self) -> stateset_core::Result<()> {
                // Basic validation - can be extended per request type
                Ok(())
            }

            /// Convert to JSON for API transmission
            pub fn to_json(&self) -> stateset_core::Result<String> {
                serde_json::to_string(self)
                    .map_err(|e| stateset_core::Error::network(format!("Serialization failed: {}", e)))
            }
        }
    };

    TokenStream::from(expanded)
}

/// Derive macro for creating API response structs
///
/// This macro adds common functionality to API response structs.
#[proc_macro_derive(ApiResponse)]
pub fn derive_api_response(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl #name {
            /// Parse from JSON response
            pub fn from_json(json: &str) -> stateset_core::Result<Self> {
                serde_json::from_str(json)
                    .map_err(|e| stateset_core::Error::network(format!("Deserialization failed: {}", e)))
            }
        }
    };

    TokenStream::from(expanded)
}

/// Macro for creating fluent validation chains
///
/// # Example
///
/// ```rust
/// validate!(request)
///     .field("email", &request.email)?
///     .email()?
///     .required()?
///     .field("age", &request.age)?
///     .min(18)?
///     .max(120)?;
/// ```
#[proc_macro]
pub fn validate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::Expr);
    
    let expanded = quote! {
        stateset_core::validation::Validator::new(stringify!(#input))
    };

    TokenStream::from(expanded)
}

/// Macro for creating endpoint definitions
///
/// This macro generates type-safe endpoint definitions with proper HTTP methods,
/// paths, and parameter handling.
///
/// # Example
///
/// ```rust
/// endpoint! {
///     name: GetOrder,
///     method: GET,
///     path: "/orders/{id}",
///     params: { id: ResourceId },
///     response: Order,
/// }
/// ```
#[proc_macro]
pub fn endpoint(input: TokenStream) -> TokenStream {
    // This is a simplified version - a real implementation would parse
    // the endpoint definition and generate appropriate structs and implementations
    
    let expanded = quote! {
        // Placeholder for endpoint macro implementation
        compile_error!("Endpoint macro not yet fully implemented");
    };

    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Testing proc macros requires special setup
    // These are placeholder tests
    
    #[test]
    fn test_macro_compilation() {
        // Proc macro tests would typically use the `trybuild` crate
        // to compile test cases and verify the generated code
        assert!(true);
    }
} 