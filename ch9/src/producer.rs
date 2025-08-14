use crate::{Message, Broker};
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
    pub fn send(&self, message: Message) -> Result<u64, String> {
        self.broker.publish(message)
    }
    
    /// Send a simple text message
    pub fn send_text(&self, topic: &str, text: &str) -> Result<u64, String> {
        let message = Message::text(topic, text);
        self.send(message)
    }
    
    /// Send a keyed message for partitioning
    pub fn send_keyed(&self, topic: &str, key: &str, value: &[u8]) -> Result<u64, String> {
        let message = Message::keyed(topic, key, value);
        self.send(message)
    }
}