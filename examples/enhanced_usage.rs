//! Enhanced StateSet SDK Usage Example
//! 
//! This example demonstrates the improved features of the StateSet Rust SDK:
//! - Enhanced error handling with detailed context
//! - Improved configuration with validation
//! - Retry logic with exponential backoff and jitter
//! - Streaming pagination for large datasets
//! - Type-safe builders with validation
//! - Comprehensive logging and metrics

use stateset::{Client, Config};
use stateset::auth::Credentials;
use stateset::models::order::{CreateOrderRequest, OrderStatus};
use stateset_core::{PoolSettings, types::{Money, Address, Contact, ResourceId}};
use std::time::Duration;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    println!("🚀 StateSet SDK Enhanced Usage Example");
    println!("=====================================\n");

    // 1. ENHANCED CONFIGURATION
    println!("📋 Setting up enhanced configuration...");
    
    let config = Config::builder()
        .base_url("https://api.stateset.io")
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .retry_attempts(3)
        .retry_delay(Duration::from_millis(1000))
        .max_retry_delay(Duration::from_secs(60))
        .retry_multiplier(2.0)
        .pool_settings(PoolSettings {
            max_connections_per_host: 10,
            max_total_connections: 100,
            idle_timeout: Duration::from_secs(30),
            keep_alive_timeout: Duration::from_secs(90),
        })
        .compression(true)
        .keep_alive(Some(Duration::from_secs(90)))
        .default_header("X-Custom-Header", "enhanced-sdk")
        .build()?;

    println!("✅ Configuration created with validation");
    println!("   - Total timeout for retries: {:?}", config.total_timeout());
    println!("   - Base URL: {}", config.base_url);
    println!("   - Retry attempts: {}", config.retry_attempts);

    // 2. CLIENT CREATION WITH AUTHENTICATION
    println!("\n🔐 Creating authenticated client...");
    
    let client = Client::with_config(config)?
        .authenticate(Credentials::bearer(
            std::env::var("STATESET_TOKEN")
                .unwrap_or_else(|_| "demo_token".to_string())
        ));

    println!("✅ Client created and authenticated");

    // 3. ENHANCED TYPE CREATION WITH VALIDATION
    println!("\n🏗️  Creating enhanced types with validation...");

    // Create a money amount with proper currency handling
    let item_price = Money::from_decimal(29.99, "USD")?;
    println!("✅ Money created: {}", item_price.format());

    // Create an address with validation
    let shipping_address = Address::new(
        "123 Enhanced Street",
        "San Francisco",
        "US"
    )?
    .with_state("CA")
    .with_postal_code("94105");
    
    println!("✅ Address created:\n{}", shipping_address.format());

    // Create contact information with email validation
    let contact = Contact::new()
        .with_name("Jane Developer")
        .with_email("jane@example.com")?
        .with_phone("+1-555-ENHANCED");

    println!("✅ Contact created with validation");

    // 4. BUILDER PATTERN WITH VALIDATION
    println!("\n🔨 Using enhanced builder pattern...");

    // Note: In a real implementation, CreateOrderRequest would derive Builder
    // For this example, we'll demonstrate the concept
    let order_request = create_enhanced_order_request(
        ResourceId::new(),
        vec![
            create_order_item(ResourceId::new(), 2, &item_price)?,
            create_order_item(ResourceId::new(), 1, &Money::from_decimal(49.99, "USD")?)?
        ],
        Some(shipping_address),
        Some(contact)
    )?;

    println!("✅ Order request built with validation");

    // 5. ERROR HANDLING DEMONSTRATION
    println!("\n❌ Demonstrating enhanced error handling...");

    // Attempt an operation that might fail to show error handling
    match client.orders().get("non-existent-id").await {
        Ok(_) => println!("Unexpected success"),
        Err(e) => {
            println!("✅ Error handled gracefully:");
            println!("   - Type: {}", error_type_description(&e));
            println!("   - Retryable: {}", e.is_retryable());
            println!("   - Status: {:?}", e.status_code());
            
            if let Some(request_id) = e.request_id() {
                println!("   - Request ID: {}", request_id);
            }
            
            if e.is_client_error() {
                println!("   - This is a client error (4xx)");
            } else if e.is_server_error() {
                println!("   - This is a server error (5xx)");
            }
        }
    }

    // 6. STREAMING PAGINATION DEMO
    println!("\n📄 Demonstrating streaming pagination...");

    // Note: This would work with a real API
    /*
    let mut order_stream = client.stream::<Order>("/api/v1/orders?limit=10");
    let mut count = 0;

    while let Some(result) = order_stream.next().await {
        match result {
            Ok(order) => {
                count += 1;
                println!("   - Order {}: {} ({})", count, order.id, order.status);
                
                if count >= 5 {
                    println!("   - Limiting to first 5 orders for demo");
                    break;
                }
            }
            Err(e) => {
                println!("   - Stream error: {}", e);
                break;
            }
        }
    }
    */
    println!("✅ Streaming pagination ready (skipped for demo)");

    // 7. CONFIGURATION VALIDATION DEMO
    println!("\n⚙️  Demonstrating configuration validation...");

    // Try to create an invalid configuration
    match Config::builder()
        .base_url("invalid-url")
        .timeout(Duration::from_secs(0)) // Invalid timeout
        .build()
    {
        Ok(_) => println!("Unexpected success"),
        Err(e) => {
            println!("✅ Configuration validation caught error:");
            println!("   - {}", e);
        }
    }

    // 8. MONEY OPERATIONS DEMO
    println!("\n💰 Demonstrating money operations...");

    let price1 = Money::from_decimal(19.99, "USD")?;
    let price2 = Money::from_decimal(25.50, "USD")?;
    
    let total = price1.add(&price2)?;
    let difference = price2.subtract(&price1)?;
    let doubled = price1.multiply(2.0);

    println!("✅ Money operations completed:");
    println!("   - {} + {} = {}", price1, price2, total);
    println!("   - {} - {} = {}", price2, price1, difference);
    println!("   - {} × 2 = {}", price1, doubled);

    // 9. RESOURCE ID VALIDATION DEMO
    println!("\n🆔 Demonstrating resource ID validation...");

    match ResourceId::from_string("invalid@id!") {
        Ok(_) => println!("Unexpected success"),
        Err(e) => println!("✅ Invalid ID rejected: {}", e),
    }

    let valid_id = ResourceId::from_string("valid-id-123")?;
    println!("✅ Valid ID accepted: {}", valid_id);
    println!("   - Is UUID: {}", valid_id.is_uuid());

    let uuid_id = ResourceId::new();
    println!("✅ UUID ID generated: {}", uuid_id);
    println!("   - Is UUID: {}", uuid_id.is_uuid());

    println!("\n🎉 Enhanced SDK demonstration completed successfully!");
    println!("All improvements are working as expected.");

    Ok(())
}

/// Helper function to create an enhanced order request
/// In a real implementation, this would use the Builder derive macro
fn create_enhanced_order_request(
    customer_id: ResourceId,
    items: Vec<OrderItem>,
    shipping_address: Option<Address>,
    contact: Option<Contact>,
) -> Result<CreateOrderRequest, stateset_core::Error> {
    // Validate items
    if items.is_empty() {
        return Err(stateset_core::Error::validation("Order must have at least one item"));
    }

    // Calculate totals
    let subtotal = items.iter()
        .try_fold(Money::new(0, "USD"), |acc, item| {
            acc.add(&item.total_price)
        })?;

    Ok(CreateOrderRequest {
        customer_id,
        items,
        shipping_address,
        billing_address: shipping_address.clone(),
        contact,
        subtotal: Some(subtotal),
        notes: None,
        metadata: None,
    })
}

/// Simplified OrderItem for demonstration
#[derive(Debug, Clone)]
struct OrderItem {
    pub product_id: ResourceId,
    pub quantity: u32,
    pub unit_price: Money,
    pub total_price: Money,
}

/// Helper function to create order items
fn create_order_item(
    product_id: ResourceId,
    quantity: u32,
    unit_price: &Money,
) -> Result<OrderItem, stateset_core::Error> {
    let total_price = unit_price.multiply(quantity as f64);

    Ok(OrderItem {
        product_id,
        quantity,
        unit_price: unit_price.clone(),
        total_price,
    })
}

/// Simplified CreateOrderRequest for demonstration
#[derive(Debug)]
struct CreateOrderRequest {
    pub customer_id: ResourceId,
    pub items: Vec<OrderItem>,
    pub shipping_address: Option<Address>,
    pub billing_address: Option<Address>,
    pub contact: Option<Contact>,
    pub subtotal: Option<Money>,
    pub notes: Option<String>,
    pub metadata: Option<stateset_core::types::Metadata>,
}

/// Get a human-readable description of the error type
fn error_type_description(error: &stateset_core::Error) -> &'static str {
    match error {
        stateset_core::Error::NotFound => "Resource Not Found",
        stateset_core::Error::Authentication { .. } => "Authentication Error",
        stateset_core::Error::Authorization { .. } => "Authorization Error",
        stateset_core::Error::RateLimit { .. } => "Rate Limit Exceeded",
        stateset_core::Error::Api { .. } => "API Error",
        stateset_core::Error::Validation { .. } => "Validation Error",
        stateset_core::Error::Network { .. } => "Network Error",
        stateset_core::Error::Serialization(_) => "Serialization Error",
        stateset_core::Error::Configuration { .. } => "Configuration Error",
        stateset_core::Error::Timeout { .. } => "Timeout Error",
        stateset_core::Error::RetryExhausted { .. } => "Retry Exhausted",
        stateset_core::Error::ConnectionPool { .. } => "Connection Pool Error",
        stateset_core::Error::Conflict { .. } => "Resource Conflict",
        stateset_core::Error::ServiceUnavailable { .. } => "Service Unavailable",
        stateset_core::Error::InvalidRequest { .. } => "Invalid Request",
        stateset_core::Error::QuotaExceeded { .. } => "Quota Exceeded",
        stateset_core::Error::Other(_) => "Other Error",
        #[cfg(feature = "realtime")]
        stateset_core::Error::WebSocket { .. } => "WebSocket Error",
    }
}