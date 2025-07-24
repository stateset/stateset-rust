//! Streaming Orders Example
//! 
//! This example demonstrates the enhanced streaming capabilities of the StateSet Rust SDK,
//! including efficient memory usage, error handling, and real-time processing of large datasets.

use stateset::{Client, Config};
use stateset::auth::Credentials;
use stateset::models::order::OrderStatus;
use stateset_client::request::SortOrder;
use stateset_core::PoolSettings;
use std::time::Duration;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    println!("🔄 StateSet SDK Streaming Orders Example");
    println!("=======================================\n");

    // Create enhanced configuration optimized for streaming
    let config = Config::builder()
        .base_url("https://api.stateset.io")
        .timeout(Duration::from_secs(60)) // Longer timeout for streaming
        .connect_timeout(Duration::from_secs(15))
        .retry_attempts(5) // More retries for reliability
        .retry_delay(Duration::from_millis(500))
        .max_retry_delay(Duration::from_secs(30))
        .retry_multiplier(1.5) // Gentler backoff for streaming
        .pool_settings(PoolSettings {
            max_connections_per_host: 5, // Fewer connections for streaming
            max_total_connections: 20,
            idle_timeout: Duration::from_secs(60),
            keep_alive_timeout: Duration::from_secs(300), // Longer keep-alive
        })
        .compression(true)
        .keep_alive(Some(Duration::from_secs(300)))
        .build()?;

    // Initialize client
    let client = Client::with_config(config)?
        .authenticate(Credentials::bearer(
            std::env::var("STATESET_TOKEN")
                .unwrap_or_else(|_| "demo-token".to_string())
        ))?;

    println!("✅ Client initialized with streaming-optimized configuration");

    // Example 1: Stream all pending orders
    println!("\n📋 Example 1: Streaming all pending orders...");
    stream_pending_orders(&client).await?;

    // Example 2: Stream orders with filters
    println!("\n🔍 Example 2: Streaming orders with complex filters...");
    stream_filtered_orders(&client).await?;

    // Example 3: Process orders in batches
    println!("\n📦 Example 3: Processing orders in batches...");
    process_orders_in_batches(&client).await?;

    // Example 4: Count orders efficiently
    println!("\n🔢 Example 4: Counting orders without fetching all data...");
    count_orders(&client).await?;

    // Example 5: Error handling in streams
    println!("\n⚠️ Example 5: Demonstrating error handling in streams...");
    handle_stream_errors(&client).await?;

    println!("\n🎉 All streaming examples completed successfully!");
    Ok(())
}

/// Example 1: Stream all pending orders with memory-efficient processing
async fn stream_pending_orders(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let mut count = 0;
    let mut total_value = 0.0;

    let mut stream = client
        .orders()
        .list()
        .status(OrderStatus::Pending)
        .sort_by_created_at(SortOrder::Desc)
        .limit(50) // Process in chunks of 50
        .stream();

    while let Some(result) = stream.next().await {
        match result {
            Ok(order) => {
                count += 1;
                if let Some(total) = order.total {
                    total_value += total.amount;
                }

                // Log progress every 100 orders
                if count % 100 == 0 {
                    println!("   Processed {} pending orders so far...", count);
                }

                // Simulate some processing
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
            Err(e) => {
                eprintln!("   Error processing order: {}", e);
                // In a real application, you might want to continue or break
                // depending on the error type
                if e.is_retryable() {
                    println!("   Error is retryable, continuing...");
                    continue;
                } else {
                    return Err(e.into());
                }
            }
        }
    }

    println!("   ✅ Processed {} pending orders", count);
    println!("   💰 Total value: ${:.2}", total_value);
    Ok(())
}

/// Example 2: Stream orders with complex filters
async fn stream_filtered_orders(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let start_date = "2024-01-01".to_string();
    let end_date = "2024-12-31".to_string();

    let mut high_value_count = 0;
    let mut stream = client
        .orders()
        .list()
        .date_range(start_date, end_date)
        .min_total(1000.0) // High-value orders only
        .sort_by_total(SortOrder::Desc)
        .limit(25)
        .stream();

    println!("   Streaming high-value orders from 2024...");

    while let Some(result) = stream.next().await {
        match result {
            Ok(order) => {
                high_value_count += 1;
                let total = order.total.map(|t| t.amount).unwrap_or(0.0);
                println!(
                    "   📊 Order {} - ${:.2} - {} - {}",
                    order.id,
                    total,
                    order.status,
                    order.created_at.format("%Y-%m-%d")
                );

                if high_value_count >= 10 {
                    println!("   ⏹️ Stopping after 10 high-value orders for demo");
                    break;
                }
            }
            Err(e) => {
                eprintln!("   ❌ Error: {}", e);
                break;
            }
        }
    }

    println!("   ✅ Found {} high-value orders", high_value_count);
    Ok(())
}

/// Example 3: Process orders in batches for bulk operations
async fn process_orders_in_batches(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = client
        .orders()
        .list()
        .status(OrderStatus::Fulfilled)
        .limit(100) // Larger batches for bulk processing
        .stream();

    let mut batch = Vec::new();
    let batch_size = 10;
    let mut total_processed = 0;

    println!("   Processing fulfilled orders in batches of {}...", batch_size);

    while let Some(result) = stream.next().await {
        match result {
            Ok(order) => {
                batch.push(order);

                if batch.len() >= batch_size {
                    // Process the batch
                    process_batch(&batch).await?;
                    total_processed += batch.len();
                    println!("   ✅ Processed batch of {} orders (total: {})", batch.len(), total_processed);
                    batch.clear();

                    // Add a small delay between batches to be nice to the API
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            }
            Err(e) => {
                eprintln!("   ❌ Error in batch processing: {}", e);
                // Process remaining batch before erroring
                if !batch.is_empty() {
                    process_batch(&batch).await?;
                    total_processed += batch.len();
                }
                return Err(e.into());
            }
        }
    }

    // Process any remaining orders in the final batch
    if !batch.is_empty() {
        process_batch(&batch).await?;
        total_processed += batch.len();
        println!("   ✅ Processed final batch of {} orders", batch.len());
    }

    println!("   🎯 Total orders processed: {}", total_processed);
    Ok(())
}

/// Simulate batch processing
async fn process_batch(orders: &[stateset_models::order::Order]) -> Result<(), Box<dyn std::error::Error>> {
    // Simulate some bulk operation like updating analytics, sending notifications, etc.
    for order in orders {
        // Simulate processing time
        tokio::time::sleep(Duration::from_millis(5)).await;
    }
    Ok(())
}

/// Example 4: Count orders efficiently without fetching all data
async fn count_orders(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    println!("   Counting orders by status...");

    // Count pending orders
    let pending_count = client
        .orders()
        .list()
        .status(OrderStatus::Pending)
        .count()
        .await?;

    // Count fulfilled orders
    let fulfilled_count = client
        .orders()
        .list()
        .status(OrderStatus::Fulfilled)
        .count()
        .await?;

    // Count cancelled orders
    let cancelled_count = client
        .orders()
        .list()
        .status(OrderStatus::Cancelled)
        .count()
        .await?;

    println!("   📊 Order counts:");
    println!("      Pending: {}", pending_count);
    println!("      Fulfilled: {}", fulfilled_count);
    println!("      Cancelled: {}", cancelled_count);
    println!("      Total: {}", pending_count + fulfilled_count + cancelled_count);

    Ok(())
}

/// Example 5: Demonstrate error handling in streams
async fn handle_stream_errors(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    println!("   Demonstrating error handling with resilient streaming...");

    let mut stream = client
        .orders()
        .list()
        .limit(25)
        .stream();

    let mut success_count = 0;
    let mut error_count = 0;
    let mut processed = 0;

    while let Some(result) = stream.next().await {
        processed += 1;
        
        match result {
            Ok(_order) => {
                success_count += 1;
                
                // Simulate that we only want to process a few for the demo
                if success_count >= 5 {
                    println!("   ✅ Successfully processed {} orders", success_count);
                    break;
                }
            }
            Err(e) => {
                error_count += 1;
                println!("   ⚠️ Error #{}: {} (Type: {:?})", error_count, e, e.status_code());
                
                // Implement error handling strategy
                if e.is_retryable() {
                    println!("      🔄 Error is retryable, stream will continue...");
                } else {
                    println!("      💥 Error is not retryable, stopping stream");
                    break;
                }

                // Don't let errors accumulate too much
                if error_count >= 3 {
                    println!("      🛑 Too many errors, stopping for safety");
                    break;
                }
            }
        }
    }

    println!("   📈 Streaming summary:");
    println!("      Items processed: {}", processed);
    println!("      Successful: {}", success_count);
    println!("      Errors: {}", error_count);

    Ok(())
}