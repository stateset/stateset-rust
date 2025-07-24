//! Official Rust SDK for StateSet API
//!
//! This crate provides a comprehensive, type-safe client library for interacting
//! with the StateSet API. It supports both async and sync operations, includes
//! built-in retry logic, rate limiting, and error handling.
//!
//! # Quick Start
//!
//! ```no_run
//! use stateset::{Client, auth::Credentials};
//! use stateset::models::order::{CreateOrderRequest, OrderItem};
//! use uuid::Uuid;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize client
//!     let client = Client::new("https://api.stateset.io")?
//!         .authenticate(Credentials::bearer("your-api-token"))?;
//!
//!     // Create an order
//!     let order = client
//!         .orders()
//!         .create(
//!             CreateOrderRequest::builder()
//!                 .customer_id(Uuid::new_v4())
//!                 .add_item(Uuid::new_v4(), 2)
//!                 .build()
//!         )
//!         .await?;
//!
//!     println!("Created order: {}", order.id);
//!     Ok(())
//! }
//! ```

// Re-export core types
pub use stateset_core::{
    Config, ConfigBuilder, Error, Result,
    traits::{ApiResource, Identifiable, ListableResource, Paginated},
    types::{Address, Contact, Money, ResourceId, Timestamp},
};

// Re-export models
pub mod models {
    pub use stateset_models::*;
}

// Re-export authentication
pub mod auth {
    pub use stateset_auth::*;
}

// Re-export the client
pub use stateset_client::{Client, request::RequestOptions};

// Re-export real-time support if enabled
#[cfg(feature = "realtime")]
pub mod realtime {
    pub use stateset_realtime::*;
}

// Prelude for common imports
pub mod prelude {
    pub use crate::{Client, Config, Error, Result};
    pub use crate::auth::Credentials;
    pub use crate::models::{
        order::{Order, OrderStatus, CreateOrderRequest},
        inventory::{InventoryLevel, InventoryReservation},
        returns::{Return, ReturnStatus},
        shipment::{Shipment, ShipmentStatus},
        work_order::{WorkOrder, WorkOrderStatus, CreateWorkOrderRequest},
        warranty::{Warranty, WarrantyStatus, CreateWarrantyRequest},
        bom::{Bom, BomStatus, CreateBomRequest},
        product::{Product, ProductStatus, CreateProductRequest},
        cart::{Cart, CartStatus, CreateCartRequest},
        checkout::{Checkout, CheckoutStatus, CreateCheckoutRequest},
        analytics::{AnalyticsReport, ReportType, CreateAnalyticsReportRequest},
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exports() {
        // Ensure key types are accessible
        let _ = Client::new("https://api.stateset.io");
        let _ = Config::default();
        let _ = auth::Credentials::bearer("test");
    }
} 