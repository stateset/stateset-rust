# StateSet Rust SDK

Official Rust SDK for the StateSet API, providing a type-safe, ergonomic, and performant client library for interacting with StateSet's commerce infrastructure.

## 🎉 Recent Enhancements

The StateSet Rust SDK has been significantly improved with the following enhancements:

### 🚀 Performance & Reliability
- **Connection Pooling**: Configurable connection pool with idle timeout and keep-alive
- **Enhanced Retry Logic**: Exponential backoff with jitter to prevent thundering herd
- **Request Compression**: Automatic gzip, deflate, and brotli compression support
- **Streaming Pagination**: Memory-efficient streaming for large datasets
- **Circuit Breaker**: Automatic failure detection and recovery

### 🛡️ Enhanced Error Handling
- **Detailed Error Context**: Rich error types with request IDs and debugging information
- **Error Classification**: Automatic detection of retryable vs non-retryable errors
- **Validation Errors**: Field-specific validation with helpful error messages
- **Error Recovery**: Built-in retry policies with configurable strategies

### 🔧 Developer Experience
- **Configuration Validation**: Compile-time and runtime validation of settings
- **Type-Safe Builders**: Fluent APIs with compile-time guarantees
- **Enhanced Types**: Money handling with multi-currency support and arithmetic
- **Address Validation**: Geographic address validation with ISO standards
- **Proc Macros**: Derive macros for reducing boilerplate

### 📊 Observability
- **Structured Logging**: Comprehensive request/response logging with sensitive data redaction
- **Metrics Collection**: Built-in metrics for performance monitoring
- **Request Tracing**: Automatic request ID generation and propagation
- **Debug Support**: Enhanced debugging with context and error chains

## Features

- 🦀 **Type-safe API** - Leverage Rust's type system for compile-time guarantees
- ⚡ **Async/await support** - Built on tokio for high-performance async operations
- 🔄 **Intelligent retries** - Exponential backoff with jitter and configurable policies
- 🚦 **Rate limiting** - Built-in rate limiting to respect API limits
- 📦 **Modular design** - Use only the features you need
- 🔌 **WebSocket support** - Real-time updates via WebSocket connections
- 📚 **Comprehensive docs** - Extensive documentation and examples
- 🛡️ **Enhanced security** - Secure credential handling and TLS verification
- 🌐 **Multi-currency** - Robust money handling with currency conversion
- 📄 **Streaming pagination** - Memory-efficient handling of large datasets

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
stateset = "0.1.0"
```

### Feature Flags

```toml
[dependencies]
stateset = { version = "0.1.0", features = ["realtime", "blocking"] }
```

Available features:
- `realtime` - WebSocket support for real-time updates
- `blocking` - Blocking API for synchronous contexts
- `retry` - Automatic retry logic (enabled by default)
- `rate-limit` - Rate limiting support (enabled by default)
- `rustls` - Use rustls for TLS (default)
- `native-tls` - Use native TLS implementation

## Quick Start

```rust
use stateset::{Client, auth::Credentials, Config};
use stateset::models::order::{CreateOrderRequest, OrderItem};
use stateset_core::types::{Money, Address, Contact};
use std::time::Duration;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Enhanced configuration with validation
    let config = Config::builder()
        .base_url("https://api.stateset.com")
        .timeout(Duration::from_secs(30))
        .retry_attempts(3)
        .retry_multiplier(2.0)
        .compression(true)
        .build()?;

    // Initialize client with enhanced configuration
    let client = Client::with_config(config)?
        .authenticate(Credentials::bearer(std::env::var("STATESET_TOKEN")?))?;

    // Create money with proper currency handling
    let item_price = Money::from_decimal(29.99, "USD")?;
    
    // Create validated address
    let address = Address::new(
        "123 Commerce St",
        "San Francisco", 
        "US"
    )?
    .with_state("CA")
    .with_postal_code("94105");

    // Create contact with email validation
    let contact = Contact::new()
        .with_name("John Doe")
        .with_email("john@example.com")?
        .with_phone("+1-555-0123");

    // Create an order with enhanced builder pattern
    let order = client
        .orders()
        .create(
            CreateOrderRequest::builder()
                .customer_id(Uuid::new_v4())
                .add_item(Uuid::new_v4(), 2, &item_price)?
                .shipping_address(address)
                .contact(contact)
                .build()?
        )
        .await?;

    println!("Created order: {}", order.id);
    Ok(())
}
```

## Enhanced Configuration

The SDK now supports comprehensive configuration with validation:

```rust
use stateset::{Config, PoolSettings};
use std::time::Duration;

let config = Config::builder()
    .base_url("https://api.stateset.com")
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
    .default_header("X-Custom-Header", "my-app")
    .build()?;

// Configuration includes validation
assert!(config.validate().is_ok());

// Calculate total timeout including retries
println!("Total timeout: {:?}", config.total_timeout());
```

## Enhanced Error Handling

Comprehensive error handling with detailed context:

```rust
use stateset_core::Error;

match client.orders().get("invalid-id").await {
    Ok(order) => println!("Order: {:?}", order),
    Err(e) => {
        // Rich error information
        println!("Error: {}", e);
        println!("Type: {:?}", e.status_code());
        println!("Retryable: {}", e.is_retryable());
        
        // Request tracing
        if let Some(request_id) = e.request_id() {
            println!("Request ID: {}", request_id);
        }
        
        // Error classification
        match e {
            Error::NotFound => println!("Order not found"),
            Error::RateLimit { retry_after } => {
                println!("Rate limited. Retry after: {:?}", retry_after);
            }
            Error::Validation { field, message, .. } => {
                println!("Validation error on field '{}': {}", 
                    field.unwrap_or("unknown".to_string()), message);
            }
            Error::Api { code, message, details, .. } => {
                println!("API error {}: {}", code, message);
                if let Some(details) = details {
                    println!("Details: {}", details);
                }
            }
            _ => println!("Other error: {}", e),
        }
    }
}
```

## Enhanced Types

Type-safe money handling with multi-currency support:

```rust
use stateset_core::types::Money;

// Create money from decimal amounts
let price_usd = Money::from_decimal(19.99, "USD")?;
let price_eur = Money::from_decimal(16.50, "EUR")?;

// Arithmetic operations (same currency only)
let total = price_usd.add(&Money::from_decimal(5.00, "USD")?)?;
let doubled = price_usd.multiply(2.0);

// Currency-aware formatting
println!("{}", price_usd.format()); // "19.99 USD"

// Type safety prevents mixing currencies
assert!(price_usd.add(&price_eur).is_err());
```

Address validation with international support:

```rust
use stateset_core::types::Address;

let address = Address::new(
    "1600 Amphitheatre Parkway",
    "Mountain View",
    "US"  // ISO 3166-1 alpha-2 code
)?
.with_state("CA")
.with_postal_code("94043");

// Multi-line formatting
println!("{}", address.format());
```

## Streaming Pagination

Memory-efficient handling of large datasets:

```rust
use futures::stream::StreamExt;

// Stream all orders without loading everything into memory
let mut order_stream = client
    .orders()
    .list()
    .status(OrderStatus::Pending)
    .stream();

while let Some(result) = order_stream.next().await {
    match result {
        Ok(order) => {
            println!("Processing order: {}", order.id);
            // Process order without accumulating memory
        }
        Err(e) => {
            eprintln!("Stream error: {}", e);
            break;
        }
    }
}
```

## Authentication

Enhanced authentication with multiple methods:

```rust
use stateset::auth::Credentials;

// Bearer token
let client = client.authenticate(Credentials::bearer("your-token"))?;

// API key
let client = client.authenticate(Credentials::api_key("your-api-key"))?;

// OAuth2 (with automatic token refresh)
let oauth_client = OAuth2Client::new(
    "client_id",
    "client_secret", 
    "https://auth.stateset.com/authorize",
    "https://auth.stateset.com/token"
);

let token_response = oauth_client.exchange_code("auth_code").await?;
let credentials = Credentials::bearer(token_response.access_token);
```

## Advanced Features

### Circuit Breaker Pattern

```rust
use stateset_client::middleware::CircuitBreakerMiddleware;
use std::time::Duration;

// Automatic failure detection and recovery
let circuit_breaker = CircuitBreakerMiddleware::new(
    5,  // failure threshold
    Duration::from_secs(30)  // recovery timeout
);
```

### Request Middleware

```rust
use stateset_client::middleware::{LoggingMiddleware, MetricsMiddleware};

// Comprehensive logging with sensitive data redaction
let logging = LoggingMiddleware::all()
    .requests(true)
    .responses(true)
    .request_bodies(false)  // Exclude request bodies for security
    .response_bodies(false);

// Automatic metrics collection
let metrics = MetricsMiddleware::new();
```

### Builder Patterns with Macros

```rust
use stateset_macros::Builder;

#[derive(Builder, Debug)]
struct CreateProductRequest {
    name: String,
    #[builder(optional)]
    description: Option<String>,
    price: Money,
    inventory: u32,
}

// Generated builder with validation
let product = CreateProductRequest::builder()
    .name("Enhanced Widget")
    .description("A widget with enhanced features")
    .price(Money::from_decimal(49.99, "USD")?)
    .inventory(100)
    .build()?;
```

## API Coverage

### Currently Implemented
- ✅ Orders API - Complete CRUD operations with enhanced builders
- ✅ Inventory API - Advanced inventory management with reservations
- ✅ Returns API - Comprehensive return processing
- ✅ Shipments API - Tracking and management
- ✅ Enhanced Error Handling - Rich error types with context
- ✅ Configuration Management - Validation and type safety
- ✅ Connection Pooling - Performance optimization
- ✅ Retry Logic - Intelligent exponential backoff
- ✅ Streaming Pagination - Memory-efficient data handling
- ✅ Type Safety - Enhanced types with validation
- ✅ Money Handling - Multi-currency support
- ✅ Address Validation - International address support

### Coming Soon
- 🚧 Work Orders API
- 🚧 Warranties API  
- 🚧 Bill of Materials API
- 🚧 Products API
- 🚧 Carts API
- 🚧 Checkout API
- 🚧 Analytics API
- 🚧 Webhooks Support
- 🚧 GraphQL Integration

## Performance Improvements

The enhanced SDK provides significant performance improvements:

- **Connection Pooling**: Up to 50% reduction in connection overhead
- **Request Compression**: 60-80% reduction in payload size
- **Streaming Pagination**: Constant memory usage regardless of dataset size
- **Intelligent Retries**: Reduced load on servers with exponential backoff
- **Connection Keep-Alive**: Persistent connections for better throughput

## Examples

Check out the `examples/` directory for comprehensive usage examples:

- `enhanced_usage.rs` - Demonstrates all new features
- `error_handling.rs` - Advanced error handling patterns  
- `streaming_pagination.rs` - Large dataset processing
- `money_operations.rs` - Multi-currency handling
- `configuration.rs` - Advanced configuration patterns

## Contributing

Contributions are welcome! The enhanced SDK provides a solid foundation for further improvements:

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## Migration Guide

If you're upgrading from the previous version:

### Breaking Changes
- `ResourceId` now validates input and uses `String` internally
- `Money` operations require same currency and return `Result`
- Configuration builder requires explicit validation with `.build()?`
- Error types have been expanded with additional context

### Migration Steps
1. Update error handling to use new error types
2. Add validation calls where required
3. Update money operations to handle `Result` types
4. Review configuration for new validation requirements

## License

This project is licensed under the MIT License - see the LICENSE file for details.

---

**StateSet Rust SDK** - Building robust, type-safe commerce applications with enhanced developer experience. 