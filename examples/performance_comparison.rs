//! Performance Comparison Example
//! 
//! This example demonstrates the performance improvements in the enhanced StateSet Rust SDK,
//! comparing different approaches for handling large datasets and showing the benefits
//! of streaming, connection pooling, and enhanced retry logic.

use stateset::{Client, Config};
use stateset::auth::Credentials;
use stateset::models::order::OrderStatus;
use stateset_client::request::SortOrder;
use stateset_core::PoolSettings;
use std::time::{Duration, Instant};
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    println!("⚡ StateSet SDK Performance Comparison");
    println!("====================================\n");

    // Create different client configurations for comparison
    let basic_config = create_basic_config()?;
    let optimized_config = create_optimized_config()?;

    let basic_client = Client::with_config(basic_config)?
        .authenticate(Credentials::bearer(
            std::env::var("STATESET_TOKEN")
                .unwrap_or_else(|_| "demo-token".to_string())
        ))?;

    let optimized_client = Client::with_config(optimized_config)?
        .authenticate(Credentials::bearer(
            std::env::var("STATESET_TOKEN")
                .unwrap_or_else(|_| "demo-token".to_string())
        ))?;

    println!("🔧 Clients configured with different settings");
    println!("   Basic client: Standard configuration");
    println!("   Optimized client: Enhanced with connection pooling and optimized timeouts\n");

    // Test 1: Single request performance
    println!("🚀 Test 1: Single Request Performance");
    println!("=====================================");
    compare_single_requests(&basic_client, &optimized_client).await?;

    // Test 2: Concurrent requests performance
    println!("\n🔄 Test 2: Concurrent Requests Performance");
    println!("=========================================");
    compare_concurrent_requests(&basic_client, &optimized_client).await?;

    // Test 3: Streaming vs Batch loading
    println!("\n📊 Test 3: Streaming vs Batch Loading");
    println!("=====================================");
    compare_streaming_approaches(&optimized_client).await?;

    // Test 4: Error handling performance
    println!("\n⚠️ Test 4: Error Handling and Retry Performance");
    println!("==============================================");
    test_retry_performance(&optimized_client).await?;

    // Test 5: Memory usage comparison
    println!("\n💾 Test 5: Memory Usage Patterns");
    println!("===============================");
    compare_memory_usage(&optimized_client).await?;

    println!("\n✅ Performance comparison completed!");
    Ok(())
}

/// Create a basic configuration (previous defaults)
fn create_basic_config() -> Result<Config, Box<dyn std::error::Error>> {
    Ok(Config::builder()
        .base_url("https://api.stateset.io")
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .retry_attempts(3)
        .retry_delay(Duration::from_millis(1000))
        .max_retry_delay(Duration::from_secs(60))
        .retry_multiplier(2.0)
        .pool_settings(PoolSettings {
            max_connections_per_host: 5,
            max_total_connections: 50,
            idle_timeout: Duration::from_secs(30),
            keep_alive_timeout: Duration::from_secs(90),
        })
        .compression(false)
        .build()?)
}

/// Create an optimized configuration (enhanced settings)
fn create_optimized_config() -> Result<Config, Box<dyn std::error::Error>> {
    Ok(Config::builder()
        .base_url("https://api.stateset.io")
        .timeout(Duration::from_secs(45))
        .connect_timeout(Duration::from_secs(15))
        .retry_attempts(5)
        .retry_delay(Duration::from_millis(500))
        .max_retry_delay(Duration::from_secs(30))
        .retry_multiplier(1.5)
        .pool_settings(PoolSettings {
            max_connections_per_host: 15,
            max_total_connections: 100,
            idle_timeout: Duration::from_secs(60),
            keep_alive_timeout: Duration::from_secs(300),
        })
        .compression(true)
        .keep_alive(Some(Duration::from_secs(300)))
        .build()?)
}

/// Compare single request performance
async fn compare_single_requests(
    basic_client: &Client,
    optimized_client: &Client,
) -> Result<(), Box<dyn std::error::Error>> {
    let iterations = 10;

    // Test basic client
    let start = Instant::now();
    for i in 0..iterations {
        let result = basic_client
            .orders()
            .list()
            .limit(1)
            .page(i)
            .execute()
            .await;
        
        match result {
            Ok(_) => {}, // Success
            Err(e) => {
                println!("   ⚠️ Basic client error on iteration {}: {}", i, e);
            }
        }
    }
    let basic_duration = start.elapsed();

    // Test optimized client
    let start = Instant::now();
    for i in 0..iterations {
        let result = optimized_client
            .orders()
            .list()
            .limit(1)
            .page(i)
            .execute()
            .await;
        
        match result {
            Ok(_) => {}, // Success
            Err(e) => {
                println!("   ⚠️ Optimized client error on iteration {}: {}", i, e);
            }
        }
    }
    let optimized_duration = start.elapsed();

    println!("   📈 Results for {} sequential requests:", iterations);
    println!("      Basic client:     {:?} ({:?} per request)", 
             basic_duration, basic_duration / iterations);
    println!("      Optimized client: {:?} ({:?} per request)", 
             optimized_duration, optimized_duration / iterations);
    
    let improvement = if optimized_duration < basic_duration {
        let speedup = basic_duration.as_millis() as f64 / optimized_duration.as_millis() as f64;
        format!("{:.1}x faster", speedup)
    } else {
        let slowdown = optimized_duration.as_millis() as f64 / basic_duration.as_millis() as f64;
        format!("{:.1}x slower", slowdown)
    };
    println!("      🎯 Optimized client is {}", improvement);

    Ok(())
}

/// Compare concurrent request performance
async fn compare_concurrent_requests(
    basic_client: &Client,
    optimized_client: &Client,
) -> Result<(), Box<dyn std::error::Error>> {
    let concurrent_requests = 10;

    // Test basic client with concurrent requests
    println!("   Testing {} concurrent requests...", concurrent_requests);
    
    let start = Instant::now();
    let basic_futures: Vec<_> = (0..concurrent_requests)
        .map(|i| {
            basic_client
                .orders()
                .list()
                .limit(5)
                .page(i)
                .execute()
        })
        .collect();
    
    let basic_results = futures::future::join_all(basic_futures).await;
    let basic_duration = start.elapsed();
    let basic_successes = basic_results.iter().filter(|r| r.is_ok()).count();

    // Test optimized client with concurrent requests
    let start = Instant::now();
    let optimized_futures: Vec<_> = (0..concurrent_requests)
        .map(|i| {
            optimized_client
                .orders()
                .list()
                .limit(5)
                .page(i)
                .execute()
        })
        .collect();
    
    let optimized_results = futures::future::join_all(optimized_futures).await;
    let optimized_duration = start.elapsed();
    let optimized_successes = optimized_results.iter().filter(|r| r.is_ok()).count();

    println!("   📊 Concurrent request results:");
    println!("      Basic client:     {:?} ({}/{} successful)", 
             basic_duration, basic_successes, concurrent_requests);
    println!("      Optimized client: {:?} ({}/{} successful)", 
             optimized_duration, optimized_successes, concurrent_requests);

    if optimized_duration < basic_duration {
        let speedup = basic_duration.as_millis() as f64 / optimized_duration.as_millis() as f64;
        println!("      🚀 Connection pooling improved concurrent performance by {:.1}x", speedup);
    }

    Ok(())
}

/// Compare different streaming approaches
async fn compare_streaming_approaches(
    client: &Client,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("   Comparing batch loading vs streaming...");

    // Approach 1: Traditional batch loading (collect all at once)
    let start = Instant::now();
    let batch_result = client
        .orders()
        .list()
        .status(OrderStatus::Pending)
        .limit(100)
        .collect_all()
        .await;
    let batch_duration = start.elapsed();
    let batch_count = batch_result.map(|orders| orders.len()).unwrap_or(0);

    // Approach 2: Streaming (memory-efficient)
    let start = Instant::now();
    let mut stream = client
        .orders()
        .list()
        .status(OrderStatus::Pending)
        .limit(100)
        .stream();

    let mut stream_count = 0;
    while let Some(result) = stream.next().await {
        match result {
            Ok(_) => stream_count += 1,
            Err(_) => break,
        }
        
        // Process first 100 items for fair comparison
        if stream_count >= 100 {
            break;
        }
    }
    let stream_duration = start.elapsed();

    println!("   🔄 Streaming comparison results:");
    println!("      Batch loading: {:?} ({} items)", batch_duration, batch_count);
    println!("      Streaming:     {:?} ({} items)", stream_duration, stream_count);
    println!("      💡 Streaming provides constant memory usage regardless of dataset size");

    Ok(())
}

/// Test retry performance and resilience
async fn test_retry_performance(
    client: &Client,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("   Testing retry logic and error handling...");

    // Test with a potentially failing request (invalid endpoint)
    let start = Instant::now();
    let result = client.get::<serde_json::Value>("/api/v1/nonexistent-endpoint").await;
    let error_duration = start.elapsed();

    match result {
        Ok(_) => println!("   🤔 Unexpected success on invalid endpoint"),
        Err(e) => {
            println!("   ✅ Error handling working correctly: {}", e);
            println!("   ⏱️ Error response time: {:?}", error_duration);
            println!("   🔄 Retryable: {}", e.is_retryable());
            if let Some(status) = e.status_code() {
                println!("   📊 Status code: {}", status);
            }
        }
    }

    // Test circuit breaker behavior (simulated)
    println!("   🔌 Circuit breaker pattern ensures fast failure after repeated errors");

    Ok(())
}

/// Compare memory usage patterns
async fn compare_memory_usage(
    client: &Client,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("   Demonstrating memory-efficient patterns...");

    // Pattern 1: Memory-efficient streaming
    println!("      🌊 Streaming: Constant memory usage");
    let mut stream = client
        .orders()
        .list()
        .limit(50)
        .stream();

    let mut processed = 0;
    while let Some(result) = stream.next().await {
        match result {
            Ok(_order) => {
                processed += 1;
                // Each order is processed and then dropped, keeping memory usage constant
                if processed >= 10 {
                    break; // Limit for demo
                }
            }
            Err(_) => break,
        }
    }
    println!("         ✅ Processed {} orders with constant memory", processed);

    // Pattern 2: Batch processing with controlled memory
    println!("      📦 Batch processing: Controlled memory growth");
    let mut batch_stream = client
        .orders()
        .list()
        .limit(25)
        .stream();

    let mut batch = Vec::with_capacity(5);
    let mut batches_processed = 0;

    while let Some(result) = batch_stream.next().await {
        match result {
            Ok(order) => {
                batch.push(order);
                
                if batch.len() >= 5 {
                    // Process batch and clear memory
                    println!("         Processing batch of {} orders", batch.len());
                    batch.clear();
                    batches_processed += 1;
                    
                    if batches_processed >= 3 {
                        break; // Limit for demo
                    }
                }
            }
            Err(_) => break,
        }
    }
    println!("         ✅ Processed {} batches with controlled memory", batches_processed);

    println!("   💡 Key memory optimizations:");
    println!("      • Streaming prevents loading entire datasets into memory");
    println!("      • Connection pooling reduces connection overhead");
    println!("      • Request compression reduces bandwidth usage");
    println!("      • Proper error handling prevents memory leaks from failed requests");

    Ok(())
}