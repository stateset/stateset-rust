# StateSet Rust SDK

Official Rust SDK for the StateSet API, providing a type-safe, ergonomic, and performant client library for interacting with StateSet's commerce infrastructure.

## Features

- 🦀 **Type-safe API** - Leverage Rust's type system for compile-time guarantees
- ⚡ **Async/await support** - Built on tokio for high-performance async operations
- 🔄 **Automatic retries** - Configurable retry logic with exponential backoff
- 🚦 **Rate limiting** - Built-in rate limiting to respect API limits
- 📦 **Modular design** - Use only the features you need
- 🔌 **WebSocket support** - Real-time updates via WebSocket connections
- 📚 **Comprehensive docs** - Extensive documentation and examples

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
use stateset::{Client, auth::Credentials};
use stateset::models::order::{CreateOrderRequest, OrderItem};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let client = Client::new("https://api.stateset.com")?
        .authenticate(Credentials::bearer(std::env::var("STATESET_TOKEN")?))?;

    // Create an order
    let order = client
        .orders()
        .create(
            CreateOrderRequest::builder()
                .customer_id(Uuid::new_v4())
                .add_item(Uuid::new_v4(), 2)
                .add_item(Uuid::new_v4(), 1)
                .build()
        )
        .await?;

    println!("Created order: {}", order.id);
    Ok(())
}
```

## Usage Examples

### Configuration

```rust
use stateset::{Client, Config};
use std::time::Duration;

let config = Config::builder()
    .base_url("https://api.stateset.com")
    .timeout(Duration::from_secs(30))
    .retry_attempts(3)
    .rate_limit(100, Duration::from_secs(60))
    .build()?;

let client = Client::with_config(config);
```

### Authentication

```rust
// Bearer token
let client = client.authenticate(Credentials::bearer("your-token"))?;

// API key
let client = client.authenticate(Credentials::api_key("your-api-key"))?;

// OAuth2
let credentials = Credentials::oauth2("client_id", "client_secret");
```

### Orders API

```rust
// List orders
let orders = client
    .orders()
    .list()
    .status(OrderStatus::Pending)
    .limit(20)
    .execute()
    .await?;

// Get specific order
let order = client.orders().get(order_id).await?;

// Update order with builder
let updated = client
    .orders()
    .update_builder(order_id)
    .status(OrderStatus::Shipped)
    .tracking_number("1234567890")
    .execute()
    .await?;

// Cancel order
client.orders().cancel(order_id).await?;
```

### Inventory Management

```rust
// Reserve inventory
let reservation = client
    .inventory()
    .reserve()
    .warehouse("WH001")
    .reference(order.id, ReferenceType::SalesOrder(order.id))
    .items(&order.items)
    .strategy(ReservationStrategy::Partial)
    .duration(Duration::from_days(7))
    .execute()
    .await?;

// Adjust inventory levels
let updates = vec![
    InventoryUpdate::adjust(product1, location1, 10),
    InventoryUpdate::adjust(product2, location1, -5),
];
client.inventory().update_batch(updates).await?;
```

### Returns Processing

```rust
// Create a return
let return_request = CreateReturnRequest {
    order_id: order.id,
    reason: ReturnReason::Defective,
    items: vec![
        CreateReturnItem {
            order_item_id: item_id,
            quantity: 1,
            condition: ItemCondition::Defective,
            reason: ReturnReason::Defective,
            notes: Some("Screen is cracked".to_string()),
        }
    ],
    notes: None,
    metadata: None,
};

let return_obj = client.returns().create(return_request).await?;

// Process the return
client.returns().approve(return_obj.id).await?;
client.returns().receive(return_obj.id).await?;
client.returns().process(return_obj.id).await?;
```

### Shipment Creation

```rust
// Create shipment
let shipment = client
    .shipments()
    .create(
        CreateShipmentRequest::builder()
            .order_id(order.id)
            .carrier("FedEx")
            .service_type("Ground")
            .from_address(warehouse_address)
            .to_address(customer_address)
            .build()
    )
    .await?;

// Track shipment
let tracking = client
    .shipments()
    .get_tracking_events(shipment.id)
    .await?;
```

### Error Handling

```rust
use stateset::Error;

match client.orders().get(order_id).await {
    Ok(order) => println!("Order: {:?}", order),
    Err(Error::NotFound) => println!("Order not found"),
    Err(Error::RateLimit { retry_after }) => {
        println!("Rate limited. Retry after {:?}", retry_after);
    }
    Err(Error::Api { code, message, .. }) => {
        println!("API error {}: {}", code, message);
    }
    Err(e) => println!("Unexpected error: {}", e),
}
```

### Streaming Results

```rust
use futures::stream::StreamExt;

// Stream all pending orders
let mut order_stream = client
    .orders()
    .list()
    .status(OrderStatus::Pending)
    .stream();

while let Some(result) = order_stream.next().await {
    match result {
        Ok(order) => process_order(order),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Real-time Updates (requires `realtime` feature)

```rust
#[cfg(feature = "realtime")]
{
    use stateset::realtime::{Channel, Event};

    // Connect to WebSocket
    let mut realtime = client.realtime().connect().await?;

    // Subscribe to channels
    realtime.subscribe(Channel::Orders).await?;
    realtime.subscribe(Channel::Inventory).await?;

    // Handle events
    while let Some(event) = realtime.next().await {
        match event? {
            Event::Order(order_event) => {
                println!("Order event: {:?}", order_event);
            }
            Event::Inventory(inv_event) => {
                println!("Inventory event: {:?}", inv_event);
            }
            _ => {}
        }
    }
}
```

## Complete Example

```rust
use stateset::{Client, auth::Credentials};
use stateset::models::{
    order::{CreateOrderRequest, OrderItem, OrderStatus},
    inventory::ReferenceType,
    shipment::CreateShipmentRequest,
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let client = Client::new("https://api.stateset.com")?
        .authenticate(Credentials::bearer(std::env::var("STATESET_TOKEN")?))?;

    // Create an order
    let order = client
        .orders()
        .create(
            CreateOrderRequest::builder()
                .customer_id(Uuid::new_v4())
                .add_item(Uuid::new_v4(), 2)
                .add_item(Uuid::new_v4(), 1)
                .build()
        )
        .await?;

    println!("Created order: {}", order.id);

    // Reserve inventory
    let reservation = client
        .inventory()
        .reserve()
        .warehouse("WH001")
        .reference(order.id, ReferenceType::SalesOrder(order.id))
        .items(&order.items)
        .execute()
        .await?;

    println!("Reserved inventory: {:?}", reservation);

    // Create shipment
    let shipment = client
        .shipments()
        .create(
            CreateShipmentRequest::builder()
                .order_id(order.id)
                .carrier("FedEx")
                .service_type("Ground")
                .build()
        )
        .await?;

    println!("Created shipment: {}", shipment.tracking_number);

    // Update order status
    client
        .orders()
        .update_builder(order.id)
        .status(OrderStatus::Shipped)
        .tracking_number(&shipment.tracking_number)
        .execute()
        .await?;

    println!("Order shipped!");

    Ok(())
}
```

## API Coverage

### Currently Implemented
- ✅ Orders API
- ✅ Inventory API  
- ✅ Returns API
- ✅ Shipments API

### Coming Soon
- 🚧 Work Orders API
- 🚧 Warranties API
- 🚧 Bill of Materials API
- 🚧 Products API
- 🚧 Carts API
- 🚧 Checkout API
- 🚧 Analytics API

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 