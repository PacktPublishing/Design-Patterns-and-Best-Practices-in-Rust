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
    fn test_producer_creation() {
        let broker = Arc::new(Broker::new());
        let _producer = Producer::new(broker);
    }

    #[test]
    fn test_send_message() {
        let broker = Arc::new(Broker::new());
        let producer = Producer::new(broker.clone());

        let message = Message::text("test.topic", "Hello");
        let offset = producer.send(message).unwrap();

        assert_eq!(offset, 0);
        assert_eq!(broker.latest_offset("test.topic"), 1);
    }

    #[test]
    fn test_send_text() {
        let broker = Arc::new(Broker::new());
        let producer = Producer::new(broker.clone());

        let offset = producer.send_text("test.topic", "Hello").unwrap();

        assert_eq!(offset, 0);
        assert_eq!(broker.latest_offset("test.topic"), 1);
    }

    #[test]
    fn test_send_keyed() {
        let broker = Arc::new(Broker::new());
        let producer = Producer::new(broker.clone());

        let offset = producer.send_keyed("test.topic", "user-123", b"Hello").unwrap();

        assert_eq!(offset, 0);
    }

    #[test]
    fn test_multiple_sends() {
        let broker = Arc::new(Broker::new());
        let producer = Producer::new(broker.clone());

        let offset1 = producer.send_text("test.topic", "First").unwrap();
        let offset2 = producer.send_text("test.topic", "Second").unwrap();
        let offset3 = producer.send_text("test.topic", "Third").unwrap();

        assert_eq!(offset1, 0);
        assert_eq!(offset2, 1);
        assert_eq!(offset3, 2);
    }

    #[test]
    fn test_send_to_multiple_topics() {
        let broker = Arc::new(Broker::new());
        let producer = Producer::new(broker.clone());

        let offset_a1 = producer.send_text("topic.a", "Message A").unwrap();
        let offset_b1 = producer.send_text("topic.b", "Message B").unwrap();
        let offset_a2 = producer.send_text("topic.a", "Message A2").unwrap();

        assert_eq!(offset_a1, 0);
        assert_eq!(offset_b1, 0);
        assert_eq!(offset_a2, 1);

        assert_eq!(broker.latest_offset("topic.a"), 2);
        assert_eq!(broker.latest_offset("topic.b"), 1);
    }
}