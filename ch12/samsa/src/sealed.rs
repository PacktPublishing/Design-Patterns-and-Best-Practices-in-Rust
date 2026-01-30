//! Sealed trait pattern for message schemas
//!
//! This module demonstrates the sealed trait pattern to create
//! type-safe message handling with controlled extensibility.

use crate::types::MessageId;
use crate::error::{SamsaError, Result};

/// Private module that contains the sealing trait
/// 
/// This prevents external crates from implementing MessageSchema
mod private {
    pub trait Sealed {}
}

/// A sealed trait for message schemas
/// 
/// Only types within this crate can implement MessageSchema,
/// ensuring API stability and type safety.
pub trait MessageSchema: private::Sealed {
    /// The Rust type that represents this message
    type Message: Clone + std::fmt::Debug;
    
    /// Serialize a message to bytes
    fn serialize(message: &Self::Message) -> Vec<u8>;
    
    /// Deserialize bytes to a message
    fn deserialize(bytes: &[u8]) -> Result<Self::Message>;

    /// Get the schema identifier
    fn schema_id() -> &'static str;

    /// Validate a message according to schema rules
    fn validate(message: &Self::Message) -> Result<()>;
}

/// A type-safe message container
/// 
/// Messages are parameterized by their schema, ensuring
/// compile-time guarantees about message structure.
#[derive(Debug, Clone)]
pub struct TypedMessage<S: MessageSchema> {
    pub id: MessageId,
    pub content: S::Message,
    pub schema_type: std::marker::PhantomData<S>,
}

impl<S: MessageSchema> TypedMessage<S> {
    /// Create a new typed message
    pub fn new(id: MessageId, content: S::Message) -> Result<Self> {
        S::validate(&content)?;

        Ok(TypedMessage {
            id,
            content,
            schema_type: std::marker::PhantomData,
        })
    }
    
    /// Serialize the message to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        S::serialize(&self.content)
    }
    
    /// Get the schema identifier
    pub fn schema_id(&self) -> &'static str {
        S::schema_id()
    }
}

// Predefined message schemas

/// JSON schema using serde_json for flexible message structures
pub struct JsonSchema;

impl private::Sealed for JsonSchema {}

impl MessageSchema for JsonSchema {
    type Message = serde_json::Value;

    fn serialize(message: &Self::Message) -> Vec<u8> {
        serde_json::to_vec(message).unwrap_or_default()
    }

    fn deserialize(bytes: &[u8]) -> Result<Self::Message> {
        serde_json::from_slice(bytes)
            .map_err(|e| SamsaError::schema(format!("JSON deserialization failed: {}", e)))
    }

    fn schema_id() -> &'static str {
        "json_v1"
    }

    fn validate(message: &Self::Message) -> Result<()> {
        // JSON is valid if it can be represented as serde_json::Value
        if message.is_null() {
            return Err(SamsaError::schema("Message cannot be null"));
        }
        Ok(())
    }
}

/// Text schema for simple string messages
pub struct TextSchema;

impl private::Sealed for TextSchema {}

impl MessageSchema for TextSchema {
    type Message = String;

    fn serialize(message: &Self::Message) -> Vec<u8> {
        message.as_bytes().to_vec()
    }

    fn deserialize(bytes: &[u8]) -> Result<Self::Message> {
        String::from_utf8(bytes.to_vec())
            .map_err(|e| SamsaError::schema(format!("UTF-8 deserialization failed: {}", e)))
    }

    fn schema_id() -> &'static str {
        "text_v1"
    }

    fn validate(message: &Self::Message) -> Result<()> {
        if message.is_empty() {
            return Err(SamsaError::schema("Required field missing: message content"));
        }

        if message.len() > 10_000 {
            return Err(SamsaError::schema(format!("Field 'message' too long: {} characters", message.len())));
        }

        Ok(())
    }
}

/// Type-safe message handler that works with any valid schema
pub struct MessageHandler<S: MessageSchema> {
    schema_type: std::marker::PhantomData<S>,
}

impl<S: MessageSchema> Default for MessageHandler<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: MessageSchema> MessageHandler<S> {
    /// Create a new message handler for a specific schema
    pub fn new() -> Self {
        Self {
            schema_type: std::marker::PhantomData,
        }
    }
    
    /// Process a typed message
    pub fn handle(&self, message: &TypedMessage<S>) -> Result<()> {
        println!("Handling message with schema: {}", message.schema_id());
        println!("Message content: {:?}", message.content);
        Ok(())
    }

    /// Deserialize and handle raw bytes
    pub fn handle_bytes(&self, id: MessageId, bytes: &[u8]) -> Result<()> {
        let content = S::deserialize(bytes)?;
        let message = TypedMessage::new(id, content)?;
        self.handle(&message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn json_schema_validation() {
        let valid_json = json!({
            "user_id": 123,
            "event_type": "login",
            "timestamp": 1234567890
        });

        assert!(JsonSchema::validate(&valid_json).is_ok());

        let null_json = serde_json::Value::Null;
        assert!(JsonSchema::validate(&null_json).is_err());
    }

    #[test]
    fn json_schema_serialization() {
        let message = json!({
            "action": "publish",
            "topic": "events",
            "data": {"key": "value"}
        });

        let bytes = JsonSchema::serialize(&message);
        assert!(!bytes.is_empty());

        let deserialized = JsonSchema::deserialize(&bytes).unwrap();
        assert_eq!(deserialized["action"], "publish");
        assert_eq!(deserialized["topic"], "events");
    }

    #[test]
    fn text_schema_validation() {
        let valid_text = "Hello, Samsa!".to_string();
        assert!(TextSchema::validate(&valid_text).is_ok());

        let empty_text = String::new();
        assert!(TextSchema::validate(&empty_text).is_err());

        let too_long = "x".repeat(10_001);
        assert!(TextSchema::validate(&too_long).is_err());
    }

    #[test]
    fn text_schema_serialization() {
        let message = "Test message".to_string();

        let bytes = TextSchema::serialize(&message);
        assert!(!bytes.is_empty());

        let deserialized = TextSchema::deserialize(&bytes).unwrap();
        assert_eq!(deserialized, "Test message");
    }

    #[test]
    fn typed_message_json() {
        let content = json!({
            "user_id": 456,
            "event_type": "logout"
        });

        let message_id = MessageId::new(1);
        let typed_message = TypedMessage::<JsonSchema>::new(message_id, content).unwrap();

        assert_eq!(typed_message.schema_id(), "json_v1");
    }

    #[test]
    fn typed_message_text() {
        let content = "Hello from Samsa".to_string();

        let message_id = MessageId::new(2);
        let typed_message = TypedMessage::<TextSchema>::new(message_id, content).unwrap();

        assert_eq!(typed_message.schema_id(), "text_v1");
    }

    #[test]
    fn message_handler_json() {
        let handler = MessageHandler::<JsonSchema>::new();

        let content = json!({
            "event": "purchase",
            "amount": 99.99
        });

        let message_id = MessageId::new(3);
        let message = TypedMessage::<JsonSchema>::new(message_id, content).unwrap();

        assert!(handler.handle(&message).is_ok());
    }

    #[test]
    fn message_handler_text() {
        let handler = MessageHandler::<TextSchema>::new();

        let content = "System notification".to_string();

        let message_id = MessageId::new(4);
        let message = TypedMessage::<TextSchema>::new(message_id, content).unwrap();

        assert!(handler.handle(&message).is_ok());
    }
}