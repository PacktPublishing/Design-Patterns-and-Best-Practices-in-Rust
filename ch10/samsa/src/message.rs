/// A message flowing through the Samsa system
///
/// Messages are immutable once created and flow downward:
/// Producer -> Broker -> Consumer
#[derive(Debug, Clone)]
pub struct Message {
    pub topic: String,
    pub key: Option<String>,
    pub value: Vec<u8>,
    pub timestamp: u64,
}

impl Message {
    pub fn new(topic: impl Into<String>, key: Option<String>, value: impl Into<Vec<u8>>) -> Self {
        Self {
            topic: topic.into(),
            key,
            value: value.into(),
            timestamp: current_timestamp(),
        }
    }
    
    /// Create a simple text message
    pub fn text(topic: impl Into<String>, text: impl Into<String>) -> Self {
        Self::new(topic, None, text.into().into_bytes())
    }
    
    /// Create a message with a key for partitioning
    pub fn keyed(topic: impl Into<String>, key: impl Into<String>, value: impl Into<Vec<u8>>) -> Self {
        Self::new(topic, Some(key.into()), value)
    }
    
    /// Get the message value as a string (if valid UTF-8)
    pub fn as_text(&self) -> Option<&str> {
        std::str::from_utf8(&self.value).ok()
    }
}

/// An event represents a message with metadata added by the broker
/// 
/// Events flow from broker to consumers and include offset information
#[derive(Debug, Clone)]
pub struct Event {
    pub message: Message,
    pub offset: u64,
}

impl Event {
    pub fn new(message: Message, offset: u64) -> Self {
        Self { message, offset }
    }
}

pub fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}