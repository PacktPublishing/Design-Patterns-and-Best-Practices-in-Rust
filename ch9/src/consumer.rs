use crate::{Event, Broker};
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
    pub fn poll(&mut self) -> Result<Option<Event>, String> {
        let events = self.broker.fetch(&self.topic, self.offset, 1)?;
        
        if let Some(event) = events.into_iter().next() {
            self.offset = event.offset + 1;
            Ok(Some(event))
        } else {
            Ok(None)
        }
    }
    
    /// Poll for multiple events at once
    pub fn poll_batch(&mut self, max_events: usize) -> Result<Vec<Event>, String> {
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