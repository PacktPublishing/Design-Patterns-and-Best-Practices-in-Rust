//! Type-safe wrappers for Samsa system
//!
//! This module demonstrates the NewType pattern for creating
//! compile-time guarantees around domain concepts.

use crate::error::{SamsaError, Result};
use std::fmt;

/// A type-safe wrapper for topic names
/// 
/// Ensures topics are valid at construction time and
/// provides a clear API boundary
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TopicId(String);

impl TopicId {
    /// Create a new TopicId, validating the input
    /// 
    /// Topics must be non-empty and contain only valid characters
    pub fn new(topic: impl AsRef<str>) -> Result<Self> {
        let topic = topic.as_ref();
        
        if topic.is_empty() {
            return Err(SamsaError::topic("Topic name cannot be empty"));
        }
        
        if topic.len() > 128 {
            return Err(SamsaError::topic(format!("Topic name too long: {} characters (max 128)", topic.len())));
        }
        
        // Check for invalid characters
        if topic.chars().any(|c| !c.is_alphanumeric() && c != '.' && c != '_' && c != '-') {
            return Err(SamsaError::topic("Topic name contains invalid characters"));
        }
        
        Ok(TopicId(topic.to_string()))
    }
    
    /// Get the topic name as a string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }
    
    /// Convert to owned String
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for TopicId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A type-safe wrapper for consumer IDs
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConsumerId(String);

impl ConsumerId {
    /// Create a new ConsumerId with validation
    pub fn new(id: impl AsRef<str>) -> Result<Self> {
        let id = id.as_ref();
        
        if id.is_empty() {
            return Err(SamsaError::consumer("Consumer ID cannot be empty"));
        }
        
        if id.len() > 64 {
            return Err(SamsaError::consumer(format!("Consumer ID too long: {} characters (max 64)", id.len())));
        }
        
        Ok(ConsumerId(id.to_string()))
    }
    
    /// Get the consumer ID as a string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ConsumerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A type-safe wrapper for message IDs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MessageId(u64);

impl MessageId {
    /// Create a new MessageId
    pub fn new(id: u64) -> Self {
        MessageId(id)
    }
    
    /// Get the raw ID value
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for MessageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn topic_id_validation() {
        // Valid topics
        assert!(TopicId::new("user.events").is_ok());
        assert!(TopicId::new("system_alerts").is_ok());
        assert!(TopicId::new("order-processing").is_ok());

        // Invalid topics
        assert!(TopicId::new("").is_err());
        assert!(TopicId::new("user events").is_err());
        assert!(TopicId::new(&"a".repeat(129)).is_err());
    }

    #[test]
    fn consumer_id_validation() {
        assert!(ConsumerId::new("consumer-1").is_ok());
        assert!(ConsumerId::new("").is_err());
        assert!(ConsumerId::new(&"a".repeat(65)).is_err());
    }
}