use crate::{Event, Broker};
use crate::error::Result;
use std::sync::Arc;

/// Consumer receives messages from topics via the broker
/// 
/// Consumers only pull data - they never push data back upstream
pub struct Consumer {
    broker: Arc<Broker>,
    topic: String,
    offset: u64,
}

impl Consumer {
    pub fn new(broker: Arc<Broker>, topic: impl Into<String>) -> Self {
        let topic = topic.into();
        let offset = broker.latest_offset(&topic);
        
        Self {
            broker,
            topic,
            offset,
        }
    }
    
    /// Create a consumer starting from the beginning of the topic
    pub fn from_beginning(broker: Arc<Broker>, topic: impl Into<String>) -> Self {
        Self {
            broker,
            topic: topic.into(),
            offset: 0,
        }
    }
    
    /// Poll for new events
    ///
    /// This pulls data DOWN from the broker to the consumer
    /// Returns Arc<Event> to avoid cloning message payloads
    pub fn poll(&mut self) -> Result<Option<Arc<Event>>> {
        let events = self.broker.fetch(&self.topic, self.offset, 1)?;

        if let Some(event) = events.into_iter().next() {
            self.offset = event.offset + 1;
            Ok(Some(event))
        } else {
            Ok(None)
        }
    }
    
    /// Poll for multiple events at once
    /// Returns Arc<Event> to avoid cloning message payloads
    pub fn poll_batch(&mut self, max_events: usize) -> Result<Vec<Arc<Event>>> {
        let events = self.broker.fetch(&self.topic, self.offset, max_events)?;

        if let Some(last_event) = events.last() {
            self.offset = last_event.offset + 1;
        }

        Ok(events)
    }
    
    /// Get the current offset position
    pub fn current_offset(&self) -> u64 {
        self.offset
    }
    
    /// Seek to a specific offset
    pub fn seek(&mut self, offset: u64) {
        self.offset = offset;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Producer;

    fn setup_broker_with_messages() -> Arc<Broker> {
        let broker = Arc::new(Broker::new());
        let producer = Producer::new(broker.clone());

        producer.send_text("test.topic", "Message 1").unwrap();
        producer.send_text("test.topic", "Message 2").unwrap();
        producer.send_text("test.topic", "Message 3").unwrap();

        broker
    }

    #[test]
    fn test_consumer_from_latest() {
        let broker = setup_broker_with_messages();
        let mut consumer = Consumer::new(broker.clone(), "test.topic");

        // Consumer starts at latest offset, so no messages available
        assert!(consumer.poll().unwrap().is_none());
        assert_eq!(consumer.current_offset(), 3);
    }

    #[test]
    fn test_consumer_from_beginning() {
        let broker = setup_broker_with_messages();
        let mut consumer = Consumer::from_beginning(broker.clone(), "test.topic");

        // Consumer starts at offset 0
        assert_eq!(consumer.current_offset(), 0);

        // Can poll all messages
        let event1 = consumer.poll().unwrap().unwrap();
        assert_eq!(event1.offset, 0);

        let event2 = consumer.poll().unwrap().unwrap();
        assert_eq!(event2.offset, 1);

        let event3 = consumer.poll().unwrap().unwrap();
        assert_eq!(event3.offset, 2);

        // No more messages
        assert!(consumer.poll().unwrap().is_none());
    }

    #[test]
    fn test_poll_batch() {
        let broker = setup_broker_with_messages();
        let mut consumer = Consumer::from_beginning(broker.clone(), "test.topic");

        let events = consumer.poll_batch(2).unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].offset, 0);
        assert_eq!(events[1].offset, 1);

        // Offset advanced
        assert_eq!(consumer.current_offset(), 2);

        // Get remaining message
        let events = consumer.poll_batch(10).unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].offset, 2);
    }

    #[test]
    fn test_seek() {
        let broker = setup_broker_with_messages();
        let mut consumer = Consumer::from_beginning(broker.clone(), "test.topic");

        // Seek to offset 1
        consumer.seek(1);
        assert_eq!(consumer.current_offset(), 1);

        let event = consumer.poll().unwrap().unwrap();
        assert_eq!(event.offset, 1);

        // Seek back to beginning
        consumer.seek(0);
        let event = consumer.poll().unwrap().unwrap();
        assert_eq!(event.offset, 0);
    }

    #[test]
    fn test_consumer_receives_new_messages() {
        let broker = Arc::new(Broker::new());
        let producer = Producer::new(broker.clone());

        // Create consumer before messages exist
        let mut consumer = Consumer::from_beginning(broker.clone(), "test.topic");

        // Produce messages
        producer.send_text("test.topic", "Message 1").unwrap();
        producer.send_text("test.topic", "Message 2").unwrap();

        // Consumer can poll them
        let event1 = consumer.poll().unwrap().unwrap();
        assert_eq!(event1.offset, 0);

        let event2 = consumer.poll().unwrap().unwrap();
        assert_eq!(event2.offset, 1);

        // No more messages yet
        assert!(consumer.poll().unwrap().is_none());

        // Produce another message
        producer.send_text("test.topic", "Message 3").unwrap();

        // Consumer can poll it
        let event3 = consumer.poll().unwrap().unwrap();
        assert_eq!(event3.offset, 2);
    }

    #[test]
    fn test_multiple_consumers() {
        let broker = setup_broker_with_messages();

        let mut consumer1 = Consumer::from_beginning(broker.clone(), "test.topic");
        let mut consumer2 = Consumer::from_beginning(broker.clone(), "test.topic");

        // Both consumers can independently consume messages
        let event1a = consumer1.poll().unwrap().unwrap();
        let event1b = consumer2.poll().unwrap().unwrap();

        assert_eq!(event1a.offset, 0);
        assert_eq!(event1b.offset, 0);

        // They maintain independent positions
        consumer1.poll().unwrap();
        assert_eq!(consumer1.current_offset(), 2);
        assert_eq!(consumer2.current_offset(), 1);
    }
}