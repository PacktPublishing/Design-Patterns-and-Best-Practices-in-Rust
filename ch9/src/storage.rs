use crate::Event;
use std::collections::HashMap;

/// In-memory storage for messages
/// 
/// Represents the bottom of our data flow - data only flows IN
pub struct Storage {
    topics: HashMap<String, Vec<Event>>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            topics: HashMap::new(),
        }
    }
    
    /// Append an event to a topic (data flowing DOWN)
    pub fn append(&mut self, topic: &str, event: Event) -> Result<(), String> {
        self.topics
            .entry(topic.to_string())
            .or_insert_with(Vec::new)
            .push(event);
        Ok(())
    }
    
    /// Fetch events from a topic (data flowing UP to caller)
    /// 
    /// Note: This is the one place where data flows upward, but it's a query
    /// operation, not state mutation. The storage itself doesn't change.
    pub fn fetch(&self, topic: &str, from_offset: u64, max_events: usize) -> Result<Vec<Event>, String> {
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
    pub fn latest_offset(&self, topic: &str) -> u64 {
        self.topics
            .get(topic)
            .and_then(|events| events.last())
            .map(|event| event.offset + 1)
            .unwrap_or(0)
    }
}