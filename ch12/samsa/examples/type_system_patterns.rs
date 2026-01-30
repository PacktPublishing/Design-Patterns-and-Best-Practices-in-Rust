//! Example demonstrating type system patterns in Samsa
//!
//! This example shows:
//! - NewType pattern for type safety
//! - Parse Don't Validate for input validation
//! - TypeState pattern for consumer lifecycle
//! - Sealed traits for message schemas

use samsa::*;
use std::sync::Arc;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("=== Samsa Type System Patterns Demo ===\n");

    // 1. NewType Pattern - Type-safe identifiers
    demonstrate_newtype_pattern()?;
    
    // 2. Parse Don't Validate - Validated construction
    demonstrate_parse_dont_validate()?;
    
    // 3. TypeState Pattern - Compile-time state management
    demonstrate_typestate_pattern()?;
    
    // 4. Sealed Traits - Controlled extensibility
    demonstrate_sealed_traits()?;

    Ok(())
}

fn demonstrate_newtype_pattern() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("1. NewType Pattern for Type Safety");
    println!("==================================");
    
    // Valid topic creation
    let topic = TopicId::new("user.events")?;
    println!("✓ Created valid topic: {}", topic);
    
    let consumer_id = ConsumerId::new("consumer-1")?;
    println!("✓ Created valid consumer ID: {}", consumer_id);
    
    let message_id = MessageId::new(12345);
    println!("✓ Created message ID: {}", message_id);

    // Invalid inputs would be caught at construction time
    match TopicId::new("") {
        Err(e) => println!("✗ Empty topic rejected: {}", e),
        Ok(_) => println!("✗ Should have failed!"),
    }
    
    match TopicId::new("user events") { // Space not allowed
        Err(e) => println!("✗ Invalid characters rejected: {}", e),
        Ok(_) => println!("✗ Should have failed!"),
    }
    
    println!();
    Ok(())
}

fn demonstrate_parse_dont_validate() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("2. Parse Don't Validate Pattern");
    println!("===============================");
    
    // Once constructed, we know these are valid
    let topic = TopicId::new("orders.created")?;
    let consumer_id = ConsumerId::new("order-processor")?;
    
    println!("✓ Topic '{}' is guaranteed valid", topic.as_str());
    println!("✓ Consumer '{}' is guaranteed valid", consumer_id.as_str());
    
    // No need to re-validate when using these values
    // The type system ensures they're correct
    println!("✓ Can safely use values without re-validation");
    
    // Demonstrate validation at boundaries
    let user_inputs = vec![
        "valid.topic".to_string(),
        "".to_string(),  // Invalid: empty
        "a".repeat(200), // Invalid: too long
        "invalid topic".to_string(), // Invalid: space
    ];
    
    println!("\nValidating user inputs:");
    for input in user_inputs {
        match TopicId::new(&input) {
            Ok(topic) => println!("  '{}' -> ✓ Valid: {}", input, topic),
            Err(e) => println!("  '{}' -> ✗ Invalid: {}",
                             if input.len() > 20 { &input[..20] } else { &input }, e),
        }
    }
    
    println!();
    Ok(())
}

fn demonstrate_typestate_pattern() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("3. TypeState Pattern for Consumer Lifecycle");
    println!("==========================================");
    
    let broker = Arc::new(Broker::new());
    let consumer_id = ConsumerId::new("lifecycle-demo")?;
    let topic = TopicId::new("demo.messages")?;
    
    // Start with a disconnected consumer
    println!("Creating disconnected consumer...");
    let consumer = DisconnectedConsumer::new(consumer_id, broker);
    
    // These operations would not compile on a disconnected consumer:
    // consumer.receive(); // Only available for subscribed consumers
    // consumer.pause();   // Only available for subscribed consumers
    
    // Connect to broker
    println!("Connecting to broker...");
    let connection_info = ConnectionInfo::new("localhost:9092".to_string(), None);
    let consumer = consumer.connect(connection_info)?;

    // Now we can subscribe (but still can't receive messages)
    println!("Subscribing to topic...");
    let consumer = consumer.subscribe(topic)?;
    
    // Now we can receive messages
    println!("Consumer can now receive messages!");
    if let Some(event) = consumer.receive() {
        println!("Received event: {:?}", event);
    } else {
        println!("No messages available");
    }
    
    // Pause the consumer
    println!("Pausing consumer...");
    let consumer = consumer.pause();
    
    // Resume the consumer
    println!("Resuming consumer...");
    let consumer = consumer.resume();
    
    // Clean shutdown
    println!("Unsubscribing and disconnecting...");
    let consumer = consumer.unsubscribe();
    let _consumer = consumer.disconnect();
    
    println!("✓ Consumer lifecycle completed successfully");
    println!("✓ Invalid state transitions prevented at compile time");
    println!();
    
    Ok(())
}

fn demonstrate_sealed_traits() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("4. Sealed Traits for Message Schemas");
    println!("====================================");

    // Create a JSON message
    let json_content = serde_json::json!({
        "user_id": 12345,
        "event_type": "login",
        "timestamp": 1640995200,
        "metadata": {
            "ip_address": "192.168.1.100",
            "user_agent": "Mozilla/5.0"
        }
    });

    // Create a typed message with JSON schema
    let message_id = MessageId::new(1001);
    let typed_message = TypedMessage::<JsonSchema>::new(message_id, json_content)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    println!("✓ Created typed JSON message with schema: {}", typed_message.schema_id());
    println!("  Message ID: {}", typed_message.id);
    println!("  Content: {:?}", typed_message.content);

    // Create a message handler for JSON
    let json_handler = MessageHandler::<JsonSchema>::new();
    json_handler.handle(&typed_message)
        .map_err(|e| format!("Handler error: {}", e))?;

    // Create a text message
    let text_content = "System notification: High memory usage detected".to_string();

    let text_message_id = MessageId::new(2001);
    let text_message = TypedMessage::<TextSchema>::new(text_message_id, text_content)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    println!("\n✓ Created text message with schema: {}", text_message.schema_id());

    // Create a handler for text messages
    let text_handler = MessageHandler::<TextSchema>::new();
    text_handler.handle(&text_message)
        .map_err(|e| format!("Handler error: {}", e))?;

    // Demonstrate serialization/deserialization
    println!("\nTesting JSON serialization:");
    let bytes = typed_message.to_bytes();
    println!("  Serialized to {} bytes", bytes.len());

    // Test round-trip
    let json_handler = MessageHandler::<JsonSchema>::new();
    match json_handler.handle_bytes(MessageId::new(1002), &bytes) {
        Ok(()) => println!("  ✓ Deserialization successful"),
        Err(e) => println!("  ✗ Deserialization failed: {}", e),
    }

    println!("\n✓ Sealed trait pattern ensures type safety");
    println!("✓ Only predefined schemas can be used");
    println!("✓ Compile-time guarantees about message structure");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn newtype_prevents_mixing_types() {
        let topic = TopicId::new("test").unwrap();
        let consumer_id = ConsumerId::new("test").unwrap();
        
        // Even though both contain the same string,
        // they are different types and cannot be mixed
        
        // This would not compile:
        // let mixed_up: TopicId = consumer_id;
        
        assert_eq!(topic.as_str(), "test");
        assert_eq!(consumer_id.as_str(), "test");
        assert_ne!(std::mem::discriminant(&topic), std::mem::discriminant(&consumer_id));
    }
    
    #[test]
    fn typestate_prevents_invalid_operations() {
        let broker = Arc::new(Broker::new());
        let consumer_id = ConsumerId::new("test").unwrap();
        
        let consumer = DisconnectedConsumer::new(consumer_id, broker);
        
        // These would not compile:
        // consumer.receive(); // Only for subscribed
        // consumer.pause();   // Only for subscribed
        // consumer.resume();  // Only for paused
        
        // This is the only valid operation for disconnected state:
        let connection_info = ConnectionInfo::new("localhost:9092".to_string(), None);
        let _connected = consumer.connect(connection_info).unwrap();
    }
}