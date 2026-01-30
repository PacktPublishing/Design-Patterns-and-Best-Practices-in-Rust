use crate::Event;
use crate::error::Result;
use std::collections::HashMap;
use std::sync::Arc;

/// In-memory storage for messages
///
/// Represents the bottom of our data flow - data only flows IN
#[derive(Debug)]
pub struct Storage {
    topics: HashMap<String, Vec<Arc<Event>>>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            topics: HashMap::new(),
        }
    }
    
    /// Append an event to a topic (data flowing DOWN)
    pub fn append(&mut self, topic: String, event: Event) -> Result<()> {
        self.topics
            .entry(topic)
            .or_default()
            .push(Arc::new(event));
        Ok(())
    }
    
    /// Fetch events from a topic (data flowing UP to caller)
    ///
    /// Note: This is the one place where data flows upward, but it's a query
    /// operation, not state mutation. The storage itself doesn't change.
    /// Returns `Arc<Event>` to avoid cloning message payloads.
    pub fn fetch(&self, topic: &str, from_offset: u64, max_events: usize) -> Result<Vec<Arc<Event>>> {
        let events = self.topics
            .get(topic)
            .map(|events| {
                events.iter()
                    .filter(|event| event.offset >= from_offset)
                    .take(max_events)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default();

        Ok(events)
    }
    
    /// Get the latest offset for a topic
    ///
    /// Note: This method is part of the Storage API contract. While not currently
    /// called in the implementation (Broker manages offsets), it demonstrates the
    /// interface that storage backends should provide.
    #[allow(dead_code)]
    pub fn latest_offset(&self, topic: &str) -> u64 {
        self.topics
            .get(topic)
            .and_then(|events| events.last())
            .map(|event| event.offset + 1)
            .unwrap_or(0)
    }
}