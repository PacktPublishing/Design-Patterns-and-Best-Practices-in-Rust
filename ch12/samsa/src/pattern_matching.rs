//! Advanced pattern matching patterns
//!
//! This module demonstrates sophisticated pattern matching techniques
//! for control flow, data extraction, and state machines.

use crate::message::current_timestamp;
use crate::error::{Result, SamsaError};
use std::collections::HashMap;

/// Message content types for pattern matching
#[derive(Debug, Clone)]
pub enum MessageContent {
    Text(String),
    Binary(Vec<u8>),
    Json(serde_json::Value),
    Empty,
}

/// Message priority levels
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Priority {
    Low,
    Normal,
    High,
    Critical,
}

/// Enhanced message type for pattern matching
#[derive(Debug, Clone)]
pub struct RichMessage {
    pub id: u64,
    pub topic: String,
    pub content: MessageContent,
    pub priority: Priority,
    pub metadata: HashMap<String, String>,
    pub timestamp: u64,
}

/// Processing results
#[derive(Debug, PartialEq)]
pub enum ProcessingResult {
    Processed,
    Queued,
    Rejected(String),
    RequiresAuth(String),
}

impl RichMessage {
    /// Route message using advanced pattern matching
    pub fn route_message(&self) -> ProcessingResult {
        match (&self.priority, &self.content, self.topic.as_str()) {
            // Critical messages always get immediate processing
            (Priority::Critical, _, _) => {
                println!("Processing critical message immediately");
                ProcessingResult::Processed
            }
            
            // High priority text messages to system topics
            (Priority::High, MessageContent::Text(text), topic) if topic.starts_with("system.") => {
                if text.len() > 1000 {
                    ProcessingResult::Queued
                } else {
                    ProcessingResult::Processed
                }
            }
            
            // JSON messages to API topics require authentication
            (_, MessageContent::Json(_), topic) if topic.starts_with("api.") => {
                ProcessingResult::RequiresAuth(topic.to_string())
            }
            
            // Binary messages over 10KB get queued
            (_, MessageContent::Binary(data), _) if data.len() > 10240 => {
                ProcessingResult::Queued
            }
            
            // Empty messages are rejected
            (_, MessageContent::Empty, _) => {
                ProcessingResult::Rejected("Empty content not allowed".to_string())
            }
            
            // Default case for normal processing
            _ => ProcessingResult::Processed,
        }
    }
}

/// Processed message after extraction
#[derive(Debug)]
pub struct ProcessedMessage {
    pub original_id: u64,
    pub user_id: u64,
    pub action: String,
    pub topic: String,
    pub auth_token: Option<String>,
}

/// Extract and transform message data
pub fn preprocess_message(message: &RichMessage) -> Option<ProcessedMessage> {
    match (&message.content, &message.metadata) {
        // Extract structured data from JSON messages
        (MessageContent::Json(json), metadata) => {
            extract_json_fields(json).map(|(user_id, action)| ProcessedMessage {
                original_id: message.id,
                user_id,
                action,
                topic: message.topic.clone(),
                auth_token: metadata.get("auth_token").cloned(),
            })
        }
        
        // Handle text messages with specific metadata patterns
        (MessageContent::Text(_), metadata) 
            if metadata.contains_key("user_id") && metadata.contains_key("action") => {
            if let (Ok(user_id), Some(action)) = (
                metadata["user_id"].parse::<u64>(),
                metadata.get("action")
            ) {
                Some(ProcessedMessage {
                    original_id: message.id,
                    user_id,
                    action: action.clone(),
                    topic: message.topic.clone(),
                    auth_token: metadata.get("auth_token").cloned(),
                })
            } else {
                None
            }
        }
        
        // All other patterns return None
        _ => None,
    }
}

fn extract_json_fields(json: &serde_json::Value) -> Option<(u64, String)> {
    match (json.get("user_id"), json.get("action")) {
        (Some(serde_json::Value::Number(id)), Some(serde_json::Value::String(action))) => {
            id.as_u64().map(|user_id| (user_id, action.clone()))
        }
        _ => None,
    }
}

/// Connection state machine using pattern matching
#[derive(Debug, Clone)]
pub enum ConnectionState {
    Disconnected,
    Connecting { attempt: u32 },
    Connected { since: u64 },
    Authenticated { user_id: u64, session_token: String },
    Error { error_code: u32, retry_after: Option<u64> },
}

#[derive(Debug)]
pub enum ConnectionEvent {
    Connect,
    Disconnect,
    Authenticate(u64, String),
    Error(u32),
    Retry,
    Timeout,
}

/// Handle connection state transitions
pub fn handle_connection_event(
    state: ConnectionState, 
    event: ConnectionEvent
) -> (ConnectionState, Vec<String>) {
    let mut actions = Vec::new();
    
    let new_state = match (state, event) {
        // From Disconnected state
        (ConnectionState::Disconnected, ConnectionEvent::Connect) => {
            actions.push("Starting connection".to_string());
            ConnectionState::Connecting { attempt: 1 }
        }
        
        // From Connecting state with retry logic
        (ConnectionState::Connecting { attempt }, ConnectionEvent::Timeout) if attempt < 3 => {
            actions.push(format!("Retrying connection (attempt {})", attempt + 1));
            ConnectionState::Connecting { attempt: attempt + 1 }
        }
        
        (ConnectionState::Connecting { attempt }, ConnectionEvent::Timeout) => {
            actions.push(format!("Max attempts reached ({})", attempt));
            ConnectionState::Error { error_code: 1001, retry_after: Some(30) }
        }
        
        (ConnectionState::Connecting { .. }, ConnectionEvent::Connect) => {
            actions.push("Connection established".to_string());
            ConnectionState::Connected { since: current_timestamp() }
        }
        
        // From Connected state
        (ConnectionState::Connected { .. }, ConnectionEvent::Authenticate(user_id, token)) => {
            actions.push(format!("User {} authenticated", user_id));
            ConnectionState::Authenticated { user_id, session_token: token }
        }
        
        // Error transitions from any state
        (_, ConnectionEvent::Error(code)) => {
            actions.push(format!("Error occurred: {}", code));
            ConnectionState::Error { 
                error_code: code, 
                retry_after: if code < 2000 { Some(10) } else { None } 
            }
        }
        
        // Invalid transitions - maintain current state
        (current_state, _) => {
            actions.push("Invalid transition ignored".to_string());
            current_state
        }
    };
    
    (new_state, actions)
}

/// Configuration value types
#[derive(Debug)]
pub enum ConfigValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<ConfigValue>),
    Object(HashMap<String, ConfigValue>),
}

/// Database configuration
#[derive(Debug)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub max_connections: u32,
    pub ssl_enabled: bool,
}

/// Parse database config using pattern matching
pub fn parse_database_config(config: &ConfigValue) -> Result<DatabaseConfig> {
    match config {
        ConfigValue::Object(obj) => {
            let host = match obj.get("host") {
                Some(ConfigValue::String(h)) if !h.is_empty() => h.clone(),
                _ => return Err(SamsaError::config("Missing or invalid host")),
            };
            
            let port = match obj.get("port") {
                Some(ConfigValue::Number(p)) if *p > 0.0 && *p <= 65535.0 => *p as u16,
                _ => return Err(SamsaError::config("Missing or invalid port")),
            };
            
            let username = match obj.get("username") {
                Some(ConfigValue::String(u)) if !u.is_empty() => u.clone(),
                _ => return Err(SamsaError::config("Missing or invalid username")),
            };
            
            let password = match obj.get("password") {
                Some(ConfigValue::String(p)) => p.clone(),
                _ => return Err(SamsaError::config("Missing password")),
            };
            
            let max_connections = match obj.get("max_connections") {
                Some(ConfigValue::Number(n)) if *n > 0.0 => *n as u32,
                None => 10, // Default value
                _ => return Err(SamsaError::config("Invalid max_connections")),
            };
            
            let ssl_enabled = match obj.get("ssl_enabled") {
                Some(ConfigValue::Boolean(b)) => *b,
                None => false, // Default value
                _ => return Err(SamsaError::config("Invalid ssl_enabled")),
            };
            
            Ok(DatabaseConfig {
                host,
                port,
                username,
                password,
                max_connections,
                ssl_enabled,
            })
        }
        _ => Err(SamsaError::config("Configuration must be an object")),
    }
}

/// Message pattern for advanced matching
pub enum MessagePattern {
    Exact(String),
    Prefix(String),
    Suffix(String),
    Contains(String),
    Regex(String),
}

impl MessagePattern {
    /// Check if a message matches this pattern
    pub fn matches(&self, text: &str) -> bool {
        match self {
            MessagePattern::Exact(s) => text == s,
            MessagePattern::Prefix(s) => text.starts_with(s),
            MessagePattern::Suffix(s) => text.ends_with(s),
            MessagePattern::Contains(s) => text.contains(s),
            MessagePattern::Regex(_) => {
                // Simplified for demo - would use regex crate
                true
            }
        }
    }
}

/// Complex nested pattern matching example
pub fn classify_message(message: &RichMessage) -> String {
    match message {
        RichMessage {
            priority: Priority::Critical,
            content: MessageContent::Text(text),
            topic,
            ..
        } if topic.starts_with("alert.") => {
            format!("CRITICAL ALERT: {}", text)
        }
        
        RichMessage {
            content: MessageContent::Json(json),
            metadata,
            ..
        } if metadata.contains_key("encrypted") => {
            match json.get("type") {
                Some(serde_json::Value::String(t)) => format!("Encrypted {} message", t),
                _ => "Encrypted message".to_string(),
            }
        }
        
        RichMessage {
            priority: Priority::Low,
            timestamp,
            ..
        } if *timestamp < current_timestamp() - 3600 => {
            "Old low-priority message".to_string()
        }
        
        RichMessage {
            content: MessageContent::Binary(data),
            ..
        } => {
            format!("Binary message ({} bytes)", data.len())
        }
        
        _ => "Standard message".to_string(),
    }
}

/// Match on multiple patterns with guards
pub fn route_by_patterns(message: &RichMessage) -> Vec<&'static str> {
    let mut routes = Vec::new();
    
    // Match priority patterns
    match message.priority {
        Priority::Critical => routes.push("emergency_handler"),
        Priority::High if message.topic.starts_with("order.") => routes.push("priority_order_queue"),
        Priority::High => routes.push("high_priority_queue"),
        _ => {}
    }
    
    // Match content patterns
    match &message.content {
        MessageContent::Json(json) => {
            if json.get("transaction_id").is_some() {
                routes.push("transaction_processor");
            }
        }
        MessageContent::Binary(data) if data.len() > 1024 * 1024 => {
            routes.push("large_file_handler");
        }
        _ => {}
    }
    
    // Match metadata patterns
    match (
        message.metadata.get("source"),
        message.metadata.get("destination"),
    ) {
        (Some(src), Some(dst)) if src == dst => routes.push("loopback_handler"),
        (Some(src), _) if src.starts_with("external.") => routes.push("external_gateway"),
        _ => {}
    }
    
    if routes.is_empty() {
        routes.push("default_handler");
    }
    
    routes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_routing() {
        let message = RichMessage {
            id: 1,
            topic: "system.alerts".to_string(),
            content: MessageContent::Text("System overload".to_string()),
            priority: Priority::High,
            metadata: HashMap::new(),
            timestamp: current_timestamp(),
        };
        
        assert_eq!(message.route_message(), ProcessingResult::Processed);
    }
    
    #[test]
    fn test_connection_state_machine() {
        let (state, actions) = handle_connection_event(
            ConnectionState::Disconnected,
            ConnectionEvent::Connect,
        );
        
        assert!(matches!(state, ConnectionState::Connecting { attempt: 1 }));
        assert!(!actions.is_empty());
        
        let (state, _) = handle_connection_event(
            state,
            ConnectionEvent::Connect,
        );
        
        assert!(matches!(state, ConnectionState::Connected { .. }));
    }
    
    #[test]
    fn test_pattern_matching() {
        let patterns = vec![
            MessagePattern::Exact("hello".to_string()),
            MessagePattern::Prefix("sys".to_string()),
            MessagePattern::Contains("error".to_string()),
        ];
        
        assert!(patterns[0].matches("hello"));
        assert!(!patterns[0].matches("hello world"));
        assert!(patterns[1].matches("system"));
        assert!(patterns[2].matches("error occurred"));
    }
}