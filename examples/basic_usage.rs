//! Basic usage example for StateSet SDK

use stateset::{Client, auth::Credentials, Result};
use stateset::models::order::{CreateOrderRequest, OrderStatus};
use stateset::models::inventory::ReferenceType;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize client
    let token = std::env::var("STATESET_TOKEN")
        .unwrap_or_else(|_| "demo-token".to_string());
    
    let client = Client::new("https://api.stateset.com")?
        .authenticate(Credentials::bearer(token))?;

    println!("StateSet SDK Example");
    println!("====================\n");

    // Example: Create an order
    println!("Creating a new order...");
    let customer_id = Uuid::new_v4();
    let product1 = Uuid::new_v4();
    let product2 = Uuid::new_v4();

    let order_request = CreateOrderRequest::builder()
        .customer_id(customer_id)
        .add_item(product1, 2)
        .add_item(product2, 1)
        .build();

    // In a real scenario, this would make an API call
    println!("Order request created: {:?}", order_request);

    // Example: List orders with filters
    println!("\nPreparing to list orders...");
    let _list_builder = client
        .orders()
        .list()
        .status(OrderStatus::Pending)
        .limit(20);

    println!("Would fetch orders with status: Pending, limit: 20");

    // Example: Inventory reservation
    println!("\nPreparing inventory reservation...");
    let order_id = Uuid::new_v4();
    let _reservation_builder = client
        .inventory()
        .reserve()
        .warehouse("WH001")
        .reference(order_id, ReferenceType::SalesOrder(order_id.into()))
        .item(product1, 2)
        .item(product2, 1);

    println!("Would reserve inventory for order: {}", order_id);

    println!("\nSDK is ready to use!");
    Ok(())
} 