//! Complete Samsa Service Example
//!
//! This example demonstrates the fully functional Samsa pub/sub service
//! with all patterns from Chapters 9-12:
//! - Architectural patterns (Ch 9)
//! - Type system patterns (Ch 10)
//! - Functional patterns (Ch 11)
//! - Unique Rust features (Ch 12)

use samsa::{
    // Core types (Ch 9)
    Message,
    // Configuration (Ch 12)
    SamsaConfig, ConfigBuilder,
    // Service management (Ch 12)
    BrokerService, ServiceManager,
    // Resources (Ch 12)
    ConnectionPool,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Samsa Complete Service Demo ===\n");

    // PATTERN: Configuration with fallbacks (Ch 12)
    println!("1. Loading Configuration...");
    let config = load_configuration()?;
    println!("   ✓ Configuration loaded successfully");
    println!("   - Broker port: {}", config.broker.port);
    println!("   - Max connections: {}", config.broker.max_connections);
    println!();

    // PATTERN: Service lifecycle with RAII (Ch 12)
    println!("2. Starting Service...");
    let mut service_manager = ServiceManager::start(config)?;
    println!("   ✓ Service started successfully");
    println!();

    // PATTERN: Connection pool management (Ch 12)
    println!("3. Managing Resources...");
    demonstrate_connection_pool()?;
    println!();

    // PATTERN: Publishing messages (Ch 9)
    println!("4. Publishing Messages...");
    publish_messages(&service_manager)?;
    println!();

    // PATTERN: Service metrics and monitoring
    println!("5. Service Status:");
    println!("   - Uptime: {:?}", service_manager.uptime());
    println!();

    // PATTERN: Graceful shutdown (Ch 12)
    println!("6. Shutting Down...");
    service_manager.stop()?;
    println!("   ✓ Service stopped gracefully");
    println!();

    println!("=== Demo Complete ===");

    Ok(())
}

/// Load configuration with fallback pattern
fn load_configuration() -> Result<SamsaConfig, Box<dyn std::error::Error>> {
    // Try to load from file, fall back to default
    match SamsaConfig::load() {
        Ok(config) => {
            println!("   Loaded from file");
            Ok(config)
        }
        Err(_) => {
            println!("   Using default configuration");

            // Build custom configuration using builder pattern
            let broker_config = ConfigBuilder::new()
                .port(8080)?
                .max_connections(100)?
                .buffer_size(4096)?
                .enable_metrics(true)
                .build();

            Ok(SamsaConfig {
                broker: broker_config,
                log_level: "info".to_string(),
                storage_path: "memory://".to_string(),
            })
        }
    }
}

/// Demonstrate connection pool with RAII
fn demonstrate_connection_pool() -> Result<(), Box<dyn std::error::Error>> {
    let pool = ConnectionPool::new(5);

    {
        // Acquire connections - automatically returned on drop
        let _conn1 = pool.acquire()?;
        let _conn2 = pool.acquire()?;
        println!("   ✓ Acquired 2 connections");

        // Connections automatically returned when scope ends
    }

    println!("   ✓ Connections returned to pool (RAII)");

    Ok(())
}

/// Publish test messages
fn publish_messages(manager: &ServiceManager) -> Result<(), Box<dyn std::error::Error>> {
    let messages = vec![
        ("user.events", "User logged in"),
        ("user.events", "User updated profile"),
        ("system.alerts", "High memory usage"),
        ("order.events", "Order #12345 placed"),
        ("order.events", "Order #12345 confirmed"),
    ];

    for (topic, content) in messages {
        let message = Message::text(topic, content);
        let offset = manager.publish(message)?;
        println!("   ✓ Published to '{}': offset {}", topic, offset);
    }

    Ok(())
}

/// Demonstrate error handling patterns
///
/// This function shows various error handling techniques but is not called from main.
/// It's included to demonstrate patterns readers can apply in their own code.
#[allow(dead_code)]
fn demonstrate_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    println!("Error Handling Patterns:");

    // Pattern: Result composition with ?
    let config = SamsaConfig::load()?;

    // Pattern: Option to Result conversion
    let _port = Some(config.broker.port)
        .filter(|&p| p > 1024)
        .ok_or("Invalid port")?;

    // Pattern: Error context with map_err
    let _service = BrokerService::new(config.broker)
        .map_err(|e| format!("Failed to start service: {}", e))?;

    Ok(())
}

/// Demonstrate block expression patterns
///
/// This function shows idiomatic Rust block expressions but is not called from main.
/// It's included to demonstrate patterns readers can apply in their own code.
#[allow(dead_code)]
fn demonstrate_block_expressions() {
    // Pattern: Conditional initialization
    let storage_type = {
        let use_memory = true;
        if use_memory {
            "memory://"
        } else {
            "disk://data"
        }
    };

    println!("Storage: {}", storage_type);

    // Pattern: Complex initialization in block
    let _config = {
        let mut builder = ConfigBuilder::new();
        builder = builder.port(9000).unwrap();
        builder = builder.max_connections(50).unwrap();
        builder.build()
    };
}

/// Demonstrate transaction guard pattern
///
/// This function shows RAII-based transaction handling but is not called from main.
/// It's included to demonstrate patterns readers can apply in their own code.
#[allow(dead_code)]
fn demonstrate_transactions() {
    use samsa::resources::TransactionGuard;

    let mut data = vec![1, 2, 3];

    {
        let mut transaction = TransactionGuard::begin(
            &mut data,
            Box::new(|d| {
                // Rollback: restore to original state
                *d = vec![1, 2, 3];
            })
        );

        // Modify data within transaction
        transaction.data_mut().push(4);
        transaction.data_mut().push(5);

        // Commit makes changes permanent
        transaction.commit();
    }

    assert_eq!(data, vec![1, 2, 3, 4, 5]);
    println!("Transaction committed: {:?}", data);
}
