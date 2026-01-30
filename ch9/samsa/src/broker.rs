use crate::{storage::Storage, Event, Message};
use crate::error::Result;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

/// The central message broker
///
/// Coordinates message flow from producers to consumers while maintaining
/// clear directional boundaries
#[derive(Debug)]
pub struct Broker {
    storage: Arc<Mutex<Storage>>,
    topic_offsets: Arc<Mutex<HashMap<String, u64>>>,
}

impl Broker {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(Mutex::new(Storage::new())),
            topic_offsets: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Publish a message (called by producers)
    ///
    /// This represents data flowing DOWN from producer to broker
    pub fn publish(&self, message: Message) -> Result<u64> {
        let mut storage = self.storage.lock().unwrap();
        let mut offsets = self.topic_offsets.lock().unwrap();

        // Clone topic once from message
        let topic = message.topic.clone();

        // Entry API requires owned String, so we clone topic again
        // This is unavoidable: both HashMaps need owned keys
        let offset = offsets.entry(topic.clone()).or_insert(0);
        let current_offset = *offset;
        *offset += 1;

        let event = Event::new(message, current_offset);
        // Pass owned topic to storage (storage won't need to clone)
        storage.append(topic, event)?;

        Ok(current_offset)
    }

    /// Fetch events for a consumer (called by consumers)
    ///
    /// This represents data flowing DOWN from broker to consumer
    /// Returns Arc<Event> to avoid cloning message payloads
    pub fn fetch(
        &self,
        topic: &str,
        from_offset: u64,
        max_events: usize,
    ) -> Result<Vec<Arc<Event>>> {
        let storage = self.storage.lock().unwrap();
        storage.fetch(topic, from_offset, max_events)
    }

    /// Get the latest offset for a topic
    pub fn latest_offset(&self, topic: &str) -> u64 {
        let offsets = self.topic_offsets.lock().unwrap();
        *offsets.get(topic).unwrap_or(&0)
    }
}

impl Default for Broker {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_broker_creation() {
        let broker = Broker::new();
        assert_eq!(broker.latest_offset("test.topic"), 0);
    }

    #[test]
    fn test_publish_single_message() {
        let broker = Broker::new();
        let message = Message::text("test.topic", "Hello");

        let offset = broker.publish(message).unwrap();
        assert_eq!(offset, 0);
        assert_eq!(broker.latest_offset("test.topic"), 1);
    }

    #[test]
    fn test_publish_multiple_messages() {
        let broker = Broker::new();

        let offset1 = broker.publish(Message::text("test.topic", "First")).unwrap();
        let offset2 = broker.publish(Message::text("test.topic", "Second")).unwrap();
        let offset3 = broker.publish(Message::text("test.topic", "Third")).unwrap();

        assert_eq!(offset1, 0);
        assert_eq!(offset2, 1);
        assert_eq!(offset3, 2);
        assert_eq!(broker.latest_offset("test.topic"), 3);
    }

    #[test]
    fn test_publish_to_different_topics() {
        let broker = Broker::new();

        let offset_a1 = broker.publish(Message::text("topic.a", "Message A1")).unwrap();
        let offset_b1 = broker.publish(Message::text("topic.b", "Message B1")).unwrap();
        let offset_a2 = broker.publish(Message::text("topic.a", "Message A2")).unwrap();

        assert_eq!(offset_a1, 0);
        assert_eq!(offset_b1, 0);
        assert_eq!(offset_a2, 1);

        assert_eq!(broker.latest_offset("topic.a"), 2);
        assert_eq!(broker.latest_offset("topic.b"), 1);
    }

    #[test]
    fn test_fetch_events() {
        let broker = Broker::new();

        broker.publish(Message::text("test.topic", "First")).unwrap();
        broker.publish(Message::text("test.topic", "Second")).unwrap();
        broker.publish(Message::text("test.topic", "Third")).unwrap();

        let events = broker.fetch("test.topic", 0, 10).unwrap();
        assert_eq!(events.len(), 3);
        assert_eq!(events[0].offset, 0);
        assert_eq!(events[1].offset, 1);
        assert_eq!(events[2].offset, 2);
    }

    #[test]
    fn test_fetch_with_offset() {
        let broker = Broker::new();

        broker.publish(Message::text("test.topic", "First")).unwrap();
        broker.publish(Message::text("test.topic", "Second")).unwrap();
        broker.publish(Message::text("test.topic", "Third")).unwrap();

        // Fetch from offset 1
        let events = broker.fetch("test.topic", 1, 10).unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].offset, 1);
        assert_eq!(events[1].offset, 2);
    }

    #[test]
    fn test_fetch_with_max_events() {
        let broker = Broker::new();

        broker.publish(Message::text("test.topic", "First")).unwrap();
        broker.publish(Message::text("test.topic", "Second")).unwrap();
        broker.publish(Message::text("test.topic", "Third")).unwrap();

        // Fetch max 2 events
        let events = broker.fetch("test.topic", 0, 2).unwrap();
        assert_eq!(events.len(), 2);
    }

    #[test]
    fn test_fetch_nonexistent_topic() {
        let broker = Broker::new();
        let events = broker.fetch("nonexistent", 0, 10).unwrap();
        assert_eq!(events.len(), 0);
    }

    #[test]
    fn test_default_trait() {
        let broker: Broker = Default::default();
        assert_eq!(broker.latest_offset("any.topic"), 0);
    }
}