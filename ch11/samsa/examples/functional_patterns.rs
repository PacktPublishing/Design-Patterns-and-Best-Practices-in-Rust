//! Example demonstrating functional programming patterns in Samsa
//!
//! This example shows:
//! - Function pipelines for data transformation
//! - Generics as type classes for enhanced TypeState
//! - Advanced pattern matching for message routing
//! - Closure patterns for configurable behavior

use samsa::*;
use samsa::pipeline::*;
use samsa::type_classes::*;
use samsa::pattern_matching::*;
use samsa::closures::*;
use std::collections::HashMap;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("=== Samsa Functional Programming Patterns Demo ===\n");

    // 1. Function Pipelines
    demonstrate_function_pipelines()?;

    // 2. Generics as Type Classes
    demonstrate_type_classes()?;

    // 3. Advanced Pattern Matching
    demonstrate_pattern_matching()?;

    // 4. Closure Patterns
    demonstrate_closure_patterns()?;

    Ok(())
}

fn demonstrate_function_pipelines() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("1. Function Pipelines for Data Transformation");
    println!("============================================");
    
    // Create subscription events for processing
    let events = vec![
        SubscriptionEvent {
            user_id: 123,
            topic: "news.technology".to_string(),
            timestamp: current_timestamp(),
            subscription_type: SubscriptionType::Subscribe,
        },
        SubscriptionEvent {
            user_id: 456,
            topic: "news.sports".to_string(),
            timestamp: current_timestamp(),
            subscription_type: SubscriptionType::Subscribe,
        },
        SubscriptionEvent {
            user_id: 789,
            topic: "alerts.system".to_string(),
            timestamp: current_timestamp() - 7200, // 2 hours ago
            subscription_type: SubscriptionType::Subscribe,
        },
        SubscriptionEvent {
            user_id: 0, // Invalid
            topic: "".to_string(),
            timestamp: current_timestamp(),
            subscription_type: SubscriptionType::Invalid,
        },
    ];
    
    // Process using basic pipeline
    let stats = process_subscription_events(events.clone());
    println!("✓ Processed {} valid subscriptions", stats.total_valid);
    println!("  Topics: {:?}", stats.subscriptions_by_topic);
    
    // Process recent subscriptions using custom combinators
    let cutoff = current_timestamp() - 3600; // 1 hour ago
    let recent_stats = analyze_recent_subscriptions(events, cutoff);
    println!("✓ Recent subscriptions by topic: {:?}", recent_stats);
    
    // Demonstrate message processing pipeline
    let messages = vec![
        Message::new("USER.LOGIN", None, b"User 123 logged in"),
        Message::new("system.alert", None, b"Critical error occurred"),
        Message::new("api.request", None, vec![0u8; 60000]), // Large message
    ];
    
    let pipeline = create_message_pipeline();
    
    for msg in messages {
        match pipeline.execute(msg) {
            Ok(processed) => println!("✓ Processed message: {}", processed.topic),
            Err(e) => println!("✗ Pipeline error: {}", e),
        }
    }
    
    println!();
    Ok(())
}

fn demonstrate_type_classes() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("2. Generics as Type Classes");
    println!("===========================");
    
    // Create subscription manager
    let mut manager = SubscriptionManager::new();
    
    // Create subscriptions using type classes
    let sub_id1 = manager.create_subscription(123, "news.tech".to_string())?;
    let sub_id2 = manager.create_subscription(456, "alerts".to_string())?;
    
    println!("✓ Created subscriptions: {} and {}", sub_id1, sub_id2);
    
    // Suspend a subscription
    manager.suspend_subscription(sub_id1, "User request".to_string())?;
    println!("✓ Suspended subscription {}", sub_id1);
    
    // Broadcast messages
    manager.broadcast_message("news.tech", "New technology announcement");
    manager.broadcast_message("alerts", "System maintenance scheduled");
    
    // Demonstrate type class constraints
    let pending = Subscription::new(999, 789, "test.topic".to_string());
    
    // Only pending subscriptions can be activated
    let active = pending.activate()?;
    println!("✓ Activated subscription: {}", active.id);
    
    // Only active subscriptions can receive messages
    let delivered = try_deliver_message(&active, "Hello, subscriber!");
    println!("✓ Message delivery: {}", if delivered { "success" } else { "failed" });
    
    // Demonstrate different subscription states
    let suspended = active.suspend("Testing".to_string());
    let delivered = try_deliver_message(&suspended, "This should fail");
    println!("✓ Suspended delivery: {}", if delivered { "success" } else { "failed (expected)" });
    
    println!();
    Ok(())
}

fn demonstrate_pattern_matching() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("3. Advanced Pattern Matching");
    println!("============================");
    
    // Create various message types for pattern matching
    let messages = vec![
        RichMessage {
            id: 1,
            topic: "alert.critical".to_string(),
            content: MessageContent::Text("System failure detected".to_string()),
            priority: Priority::Critical,
            metadata: HashMap::new(),
            timestamp: current_timestamp(),
        },
        RichMessage {
            id: 2,
            topic: "api.users".to_string(),
            content: MessageContent::Json(create_sample_json()),
            priority: Priority::Normal,
            metadata: {
                let mut map = HashMap::new();
                map.insert("encrypted".to_string(), "true".to_string());
                map
            },
            timestamp: current_timestamp(),
        },
        RichMessage {
            id: 3,
            topic: "data.backup".to_string(),
            content: MessageContent::Binary(vec![0u8; 15000]),
            priority: Priority::Low,
            metadata: HashMap::new(),
            timestamp: current_timestamp(),
        },
    ];
    
    // Route messages using pattern matching
    for message in &messages {
        let result = message.route_message();
        println!("✓ Message {} routing: {:?}", message.id, result);
        
        let classification = classify_message(message);
        println!("  Classification: {}", classification);
        
        let routes = route_by_patterns(message);
        println!("  Routes: {:?}", routes);
    }
    
    // Demonstrate connection state machine
    println!("\nConnection State Machine:");
    let mut state = ConnectionState::Disconnected;
    let events = vec![
        ConnectionEvent::Connect,
        ConnectionEvent::Connect, // Connection established
        ConnectionEvent::Authenticate(123, "token123".to_string()),
        ConnectionEvent::Error(500),
        ConnectionEvent::Retry,
    ];
    
    for event in events {
        let (new_state, actions) = handle_connection_event(state, event);
        println!("✓ State transition, actions: {:?}", actions);
        state = new_state;
    }
    
    println!();
    Ok(())
}

fn demonstrate_closure_patterns() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("4. Closure Patterns for Configurable Behavior");
    println!("==============================================");
    
    // Create message filters using closures
    let large_filter = MessageFilter::new(
        "Large Messages".to_string(),
        |msg| msg.value.len() > 100
    );
    
    let system_filter = MessageFilter::new(
        "System Topics".to_string(), 
        |msg| msg.topic.starts_with("system.")
    );
    
    // Compose filters
    let composite_filter = large_filter.and(system_filter);
    
    let test_message = Message::new("system.alerts", None, vec![0u8; 200]);
    println!("✓ Composite filter '{}' matches: {}", 
             composite_filter.name(),
             composite_filter.matches(&test_message));
    
    // Demonstrate routing strategies
    let router = create_routing_strategies();
    
    let messages = vec![
        Message::new("system.logging", None, b"Log entry"),
        Message::new("api.users", None, b"User data"),
        Message::new("uploads", None, vec![0u8; 20000]), // Large message
    ];
    
    for message in messages {
        let routes = router.route_message(&message);
        println!("✓ Message to '{}' routed to: {:?}", message.topic, routes);
    }
    
    // Demonstrate event bus
    let event_bus = EventBus::new();
    
    // Subscribe to events
    event_bus.subscribe("message", |event: &SystemEvent| {
        if let SystemEvent::MessageReceived { message_id, topic } = event {
            println!("  📨 Message {} received on topic {}", message_id, topic);
        }
    });
    
    event_bus.subscribe("user", |event: &SystemEvent| {
        match event {
            SystemEvent::UserConnected { user_id } => {
                println!("  👤 User {} connected", user_id);
            }
            SystemEvent::UserDisconnected { user_id } => {
                println!("  👤 User {} disconnected", user_id);
            }
            _ => {}
        }
    });
    
    // Publish events
    event_bus.publish("message", &SystemEvent::MessageReceived {
        message_id: 1001,
        topic: "user.activity".to_string(),
    });
    
    event_bus.publish("user", &SystemEvent::UserConnected { user_id: 123 });
    event_bus.publish("user", &SystemEvent::UserDisconnected { user_id: 123 });
    
    // Demonstrate processing pipeline
    let pipeline = Pipeline::new()
        .add_validation(
            |msg: &Message| !msg.topic.is_empty(),
            "Topic cannot be empty".to_string()
        )
        .add_transformation(|mut msg: Message| {
            msg.topic = msg.topic.to_lowercase();
            msg
        })
        .add_stage(|msg: Message| {
            if msg.value.len() > 1000 {
                Err("Message too large".to_string().into())
            } else {
                Ok(msg)
            }
        });
    
    let test_msg = Message::new("TEST.TOPIC", None, b"Hello World");
    match pipeline.execute(test_msg) {
        Ok(processed) => println!("✓ Pipeline processed message: {}", processed.topic),
        Err(e) => println!("✗ Pipeline error: {}", e),
    }
    
    println!();
    Ok(())
}

fn create_sample_json() -> serde_json::Value {
    serde_json::json!({
        "user_id": 123,
        "action": "login",
        "type": "authentication"
    })
}

fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functional_patterns_integration() {
        // Test that all patterns work together
        
        // Create events and process with pipelines
        let events = vec![
            SubscriptionEvent {
                user_id: 1,
                topic: "test".to_string(),
                timestamp: current_timestamp(),
                subscription_type: SubscriptionType::Subscribe,
            }
        ];
        
        let stats = process_subscription_events(events);
        assert_eq!(stats.total_valid, 1);
        
        // Test type classes
        let pending = Subscription::new(1, 123, "test".to_string());
        let active = pending.activate().unwrap();
        assert!(try_deliver_message(&active, "test"));
        
        // Test pattern matching
        let msg = RichMessage {
            id: 1,
            topic: "system.test".to_string(),
            content: MessageContent::Text("test".to_string()),
            priority: Priority::High,
            metadata: HashMap::new(),
            timestamp: current_timestamp(),
        };
        
        assert_eq!(msg.route_message(), ProcessingResult::Processed);
        
        // Test closures
        let filter = MessageFilter::new("test".to_string(), |_| true);
        let test_msg = Message::new("test", None, b"test");
        assert!(filter.matches(&test_msg));
    }
}