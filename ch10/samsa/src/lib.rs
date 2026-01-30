//! Samsa - A simple publish/subscribe microservice (Chapter 10)
//!
//! This crate builds on Chapter 9 by adding type system patterns:
//! - NewType pattern for type-safe domain concepts
//! - Parse Don't Validate for input validation
//! - TypeState pattern for lifecycle management
//! - Sealed traits for controlled extensibility

// Core error handling - unified across all modules
pub use error::{SamsaError, Result};

pub use message::{Message, Event};
pub use broker::Broker;
pub use producer::Producer;
pub use consumer::Consumer;

// Chapter 10 additions - Type System Patterns
pub use types::{TopicId, ConsumerId, MessageId};
pub use typestate_consumer::{
    Consumer as TypedConsumer,
    DisconnectedConsumer, ConnectedConsumer, SubscribedConsumer, PausedConsumer,
    ConnectionInfo,
};
pub use sealed::{
    MessageSchema, TypedMessage, MessageHandler,
    JsonSchema, TextSchema,
};

mod error;
mod message;
mod broker;
mod producer;
mod consumer;
mod storage;

// New modules for Chapter 10
pub mod types;
pub mod typestate_consumer;
pub mod sealed;