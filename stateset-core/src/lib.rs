//! Core types and traits for StateSet SDK
//!
//! This crate provides the fundamental building blocks for the StateSet SDK,
//! including common traits, error types, and configuration structures.

pub mod config;
pub mod error;
pub mod traits;
pub mod types;

pub use config::{Config, ConfigBuilder};
pub use error::{Error, Result};
pub use traits::{ApiResource, Identifiable, ListableResource, Paginated};
pub use types::{ResourceId, Timestamp}; 