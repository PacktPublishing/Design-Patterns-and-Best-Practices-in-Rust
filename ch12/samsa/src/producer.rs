use crate::{Message, Broker};
use crate::error::Result;
use std::sync::Arc;

/// Producer sends messages to topics via the broker
/// 
/// Producers only push data downward - they never receive data back
pub struct Producer {
    broker: Arc<Broker>,
}

impl Producer {
    pub fn new(broker: Arc<Broker>) -> Self {
        Self { broker }
    }
    
    /// Send a message to a topic
    /// 
    /// Returns the offset where the message was stored
    pub fn send(&self, message: Message) -> Result<u64> {
        self.broker.publish(message)
    }
    
    /// Send a simple text message
    pub fn send_text(&self, topic: &str, text: &str) -> Result<u64> {
        let message = Message::text(topic, text);
        self.send(message)
    }
    
    /// Send a keyed message for partitioning
    pub fn send_keyed(&self, topic: &str, key: &str, value: &[u8]) -> Result<u64> {
        let message = Message::keyed(topic, key, value);
        self.send(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_text() {
        let broker = Arc::new(Broker::new());
        let producer = Producer::new(broker.clone());

        let offset = producer.send_text("test.topic", "Hello").unwrap();
        assert_eq!(offset, 0);

        let offset = producer.send_text("test.topic", "World").unwrap();
        assert_eq!(offset, 1);
    }

    #[test]
    fn test_send_message() {
        let broker = Arc::new(Broker::new());
        let producer = Producer::new(broker.clone());

        let message = Message::text("test.topic", "Test");
        let offset = producer.send(message).unwrap();
        assert_eq!(offset, 0);
    }

    #[test]
    fn test_send_keyed() {
        let broker = Arc::new(Broker::new());
        let producer = Producer::new(broker.clone());

        let offset = producer.send_keyed("test.topic", "key1", b"value1").unwrap();
        assert_eq!(offset, 0);

        let offset = producer.send_keyed("test.topic", "key2", b"value2").unwrap();
        assert_eq!(offset, 1);
    }

    #[test]
    fn test_send_to_different_topics() {
        let broker = Arc::new(Broker::new());
        let producer = Producer::new(broker.clone());

        let offset_a = producer.send_text("topic.a", "Message A").unwrap();
        let offset_b = producer.send_text("topic.b", "Message B").unwrap();
        let offset_a2 = producer.send_text("topic.a", "Message A2").unwrap();

        // Per-topic offsets
        assert_eq!(offset_a, 0);
        assert_eq!(offset_b, 0);
        assert_eq!(offset_a2, 1);
    }

    #[test]
    fn test_multiple_producers() {
        let broker = Arc::new(Broker::new());
        let producer1 = Producer::new(broker.clone());
        let producer2 = Producer::new(broker.clone());

        let offset1 = producer1.send_text("test.topic", "From Producer 1").unwrap();
        let offset2 = producer2.send_text("test.topic", "From Producer 2").unwrap();

        assert_eq!(offset1, 0);
        assert_eq!(offset2, 1);
    }

    #[test]
    fn test_producer_broker_integration() {
        let broker = Arc::new(Broker::new());
        let producer = Producer::new(broker.clone());

        // Send messages
        producer.send_text("test.topic", "Message 1").unwrap();
        producer.send_text("test.topic", "Message 2").unwrap();

        // Verify in broker
        let events = broker.fetch("test.topic", 0, 10).unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].message.as_text().unwrap(), "Message 1");
        assert_eq!(events[1].message.as_text().unwrap(), "Message 2");
    }
}