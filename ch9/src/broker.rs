use crate::{Message, Event, storage::Storage};
use std::sync::Arc;
use std::sync::Mutex;

/// The central message broker
/// 
/// Coordinates message flow from producers to consumers while maintaining
/// clear directional boundaries
pub struct Broker {
    storage: Arc<Mutex<Storage>>,
    next_offset: Arc<Mutex<u64>>,
}

impl Broker {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(Mutex::new(Storage::new())),
            next_offset: Arc::new(Mutex::new(0)),
        }
    }
    
    /// Publish a message (called by producers)
    /// 
    /// This represents data flowing DOWN from producer to broker
    pub fn publish(&self, message: Message) -> Result<u64, String> {
        let mut storage = self.storage.lock().unwrap();
        let mut offset_counter = self.next_offset.lock().unwrap();
        
        let offset = *offset_counter;
        *offset_counter += 1;
        
        let event = Event::new(message, offset);
        storage.append(&event.message.topic, event)?;
        
        Ok(offset)
    }
    
    /// Fetch events for a consumer (called by consumers)
    /// 
    /// This represents data flowing DOWN from broker to consumer
    pub fn fetch(&self, topic: &str, from_offset: u64, max_events: usize) -> Result<Vec<Event>, String> {
        let storage = self.storage.lock().unwrap();
        storage.fetch(topic, from_offset, max_events)
    }
    
    /// Get the latest offset for a topic
    pub fn latest_offset(&self, topic: &str) -> u64 {
        let storage = self.storage.lock().unwrap();
        storage.latest_offset(topic)
    }
}

impl Default for Broker {
    fn default() -> Self {
        Self::new()
    }
}