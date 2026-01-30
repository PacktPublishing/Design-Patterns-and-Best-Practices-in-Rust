//! Integration tests for the complete Samsa service
//!
//! These tests verify that all components work together correctly
//! in realistic scenarios.

use samsa::{
    // Core functionality
    Message, Broker, Producer, Consumer,
    // Configuration
    SamsaConfig, BrokerConfig, ConfigBuilder,
    // Service management
    BrokerService, ServiceManager,
    // Resources
    ConnectionPool,
};
use std::sync::Arc;
use std::time::Duration;

#[test]
fn test_end_to_end_pubsub() {
    // Create broker
    let broker = Arc::new(Broker::new());

    // Create producer and consumer
    let producer = Producer::new(broker.clone());
    let mut consumer = Consumer::from_beginning(broker.clone(), "test.topic");

    // Publish messages
    let offset1 = producer.send_text("test.topic", "Message 1").unwrap();
    let offset2 = producer.send_text("test.topic", "Message 2").unwrap();
    let offset3 = producer.send_text("test.topic", "Message 3").unwrap();

    assert_eq!(offset1, 0);
    assert_eq!(offset2, 1);
    assert_eq!(offset3, 2);

    // Consume messages
    let event1 = consumer.poll().unwrap().unwrap();
    assert_eq!(event1.offset, 0);
    assert_eq!(event1.message.as_text().unwrap(), "Message 1");

    let event2 = consumer.poll().unwrap().unwrap();
    assert_eq!(event2.offset, 1);

    let event3 = consumer.poll().unwrap().unwrap();
    assert_eq!(event3.offset, 2);

    // No more messages
    assert!(consumer.poll().unwrap().is_none());
}

#[test]
fn test_multiple_producers_single_consumer() {
    let broker = Arc::new(Broker::new());

    let producer1 = Producer::new(broker.clone());
    let producer2 = Producer::new(broker.clone());
    let mut consumer = Consumer::from_beginning(broker.clone(), "shared.topic");

    // Both producers publish to same topic
    producer1.send_text("shared.topic", "From Producer 1").unwrap();
    producer2.send_text("shared.topic", "From Producer 2").unwrap();
    producer1.send_text("shared.topic", "From Producer 1 again").unwrap();

    // Consumer receives all messages in order
    let events = consumer.poll_batch(10).unwrap();
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].offset, 0);
    assert_eq!(events[1].offset, 1);
    assert_eq!(events[2].offset, 2);
}

#[test]
fn test_multiple_consumers_independent_positions() {
    let broker = Arc::new(Broker::new());
    let producer = Producer::new(broker.clone());

    // Publish messages first
    for i in 0..5 {
        producer.send_text("test.topic", &format!("Message {}", i)).unwrap();
    }

    // Create two consumers
    let mut consumer1 = Consumer::from_beginning(broker.clone(), "test.topic");
    let mut consumer2 = Consumer::from_beginning(broker.clone(), "test.topic");

    // Consumer 1 reads 2 messages
    consumer1.poll().unwrap();
    consumer1.poll().unwrap();
    assert_eq!(consumer1.current_offset(), 2);

    // Consumer 2 reads 4 messages
    for _ in 0..4 {
        consumer2.poll().unwrap();
    }
    assert_eq!(consumer2.current_offset(), 4);

    // They maintain independent positions
    let event1 = consumer1.poll().unwrap().unwrap();
    assert_eq!(event1.offset, 2);

    let event2 = consumer2.poll().unwrap().unwrap();
    assert_eq!(event2.offset, 4);
}

#[test]
fn test_configuration_loading_and_validation() {
    // Test default configuration
    let config = SamsaConfig::default();
    assert_eq!(config.broker.port, 8080);
    assert_eq!(config.broker.max_connections, 1000);

    // Test builder pattern with validation
    let result = ConfigBuilder::new()
        .port(9000).unwrap()
        .max_connections(500).unwrap()
        .enable_metrics(true)
        .build();

    assert_eq!(result.port, 9000);
    assert_eq!(result.max_connections, 500);
    assert!(result.enable_metrics);

    // Test invalid port validation
    let invalid = ConfigBuilder::new().port(80);
    assert!(invalid.is_err());
}

#[test]
fn test_service_lifecycle() {
    let config = SamsaConfig::default();
    let mut manager = ServiceManager::start(config).unwrap();

    // Service should be running
    let message = Message::text("test.topic", "Hello");
    let offset = manager.publish(message).unwrap();
    assert_eq!(offset, 0);

    // Check uptime
    std::thread::sleep(Duration::from_millis(10));
    let uptime = manager.uptime();
    assert!(uptime.as_millis() >= 10);

    // Stop service
    manager.stop().unwrap();

    // Should not be able to publish after stop
    let message2 = Message::text("test.topic", "World");
    assert!(manager.publish(message2).is_err());
}

#[test]
fn test_broker_service_creation() {
    let config = BrokerConfig::default();
    let service = BrokerService::new(config).unwrap();

    // Service should accept messages
    let message = Message::text("test.topic", "Test message");
    let offset = service.publish(message).unwrap();
    assert_eq!(offset, 0);
}

#[test]
fn test_connection_pool_exhaustion_and_recovery() {
    let pool = ConnectionPool::new(2);

    // Acquire max connections
    let _guard1 = pool.acquire().unwrap();
    let _guard2 = pool.acquire().unwrap();

    // Pool should be exhausted
    assert!(pool.acquire().is_err());

    // Drop one guard
    drop(_guard1);

    // Should be able to acquire again
    let _guard3 = pool.acquire().unwrap();
}

#[test]
fn test_consumer_seek_and_replay() {
    let broker = Arc::new(Broker::new());
    let producer = Producer::new(broker.clone());
    let mut consumer = Consumer::from_beginning(broker.clone(), "replay.topic");

    // Publish messages
    for i in 0..10 {
        producer.send_text("replay.topic", &format!("Message {}", i)).unwrap();
    }

    // Read first 5
    for _ in 0..5 {
        consumer.poll().unwrap();
    }
    assert_eq!(consumer.current_offset(), 5);

    // Seek back to beginning
    consumer.seek(0);
    assert_eq!(consumer.current_offset(), 0);

    // Can replay messages
    let event = consumer.poll().unwrap().unwrap();
    assert_eq!(event.offset, 0);
    assert_eq!(event.message.as_text().unwrap(), "Message 0");

    // Seek to specific position
    consumer.seek(7);
    let event = consumer.poll().unwrap().unwrap();
    assert_eq!(event.offset, 7);
    assert_eq!(event.message.as_text().unwrap(), "Message 7");
}

#[test]
fn test_message_ordering_across_topics() {
    let broker = Arc::new(Broker::new());
    let producer = Producer::new(broker.clone());

    // Publish to multiple topics interleaved
    producer.send_text("topic.a", "A1").unwrap();
    producer.send_text("topic.b", "B1").unwrap();
    producer.send_text("topic.a", "A2").unwrap();
    producer.send_text("topic.c", "C1").unwrap();
    producer.send_text("topic.b", "B2").unwrap();

    // Consumers for each topic
    let mut consumer_a = Consumer::from_beginning(broker.clone(), "topic.a");
    let mut consumer_b = Consumer::from_beginning(broker.clone(), "topic.b");
    let mut consumer_c = Consumer::from_beginning(broker.clone(), "topic.c");

    // Each topic maintains its message order
    let events_a = consumer_a.poll_batch(10).unwrap();
    assert_eq!(events_a.len(), 2);
    assert_eq!(events_a[0].message.as_text().unwrap(), "A1");
    assert_eq!(events_a[1].message.as_text().unwrap(), "A2");

    let events_b = consumer_b.poll_batch(10).unwrap();
    assert_eq!(events_b.len(), 2);
    assert_eq!(events_b[0].message.as_text().unwrap(), "B1");
    assert_eq!(events_b[1].message.as_text().unwrap(), "B2");

    let events_c = consumer_c.poll_batch(10).unwrap();
    assert_eq!(events_c.len(), 1);
    assert_eq!(events_c[0].message.as_text().unwrap(), "C1");
}

#[test]
fn test_high_volume_throughput() {
    let broker = Arc::new(Broker::new());
    let producer = Producer::new(broker.clone());
    let mut consumer = Consumer::from_beginning(broker.clone(), "high.volume");

    // Publish many messages
    let message_count = 1000;
    for i in 0..message_count {
        producer.send_text("high.volume", &format!("Message {}", i)).unwrap();
    }

    // Consume all in batches
    let mut total_received = 0;
    while total_received < message_count {
        let events = consumer.poll_batch(100).unwrap();
        if events.is_empty() {
            break;
        }
        total_received += events.len();
    }

    assert_eq!(total_received, message_count);
}

#[test]
fn test_graceful_shutdown_with_raii() {
    let config = SamsaConfig::default();

    {
        let manager = ServiceManager::start(config).unwrap();
        let message = Message::text("test.topic", "Test");
        manager.publish(message).unwrap();

        // Manager drops here, should shutdown gracefully
    }

    // If we got here without panic, RAII cleanup worked
}
