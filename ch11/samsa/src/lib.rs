//! Samsa - A simple publish/subscribe microservice (Chapter 11)
//!
//! This crate builds on Chapters 9-10 by adding functional programming patterns:
//! - Function pipelines for data transformation
//! - Generics as type classes for enhanced abstractions
//! - Advanced pattern matching for control flow
//! - Closure patterns for configurable behavior

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

// Chapter 11 additions - Functional Programming Patterns
pub use pipeline::{
    SubscriptionEvent, SubscriptionStats, SubscriptionProcessing,
    MessagePipeline, EventTransformation, AdvancedPipeline,
};
pub use type_classes::{
    Activatable, Suspendable, Cancellable, MessageDeliverable,
    Subscription, SubscriptionManager,
};
pub use pattern_matching::{
    RichMessage, MessageContent, Priority, ProcessingResult,
    ConnectionState, ConnectionEvent, ConfigValue, DatabaseConfig,
};
pub use closures::{
    MessageFilter, MessageRouter, EventBus, SystemEvent, Pipeline,
};

mod error;
mod message;
mod broker;
mod producer;
mod consumer;
mod storage;

// Chapter 10 modules
pub mod types;
pub mod typestate_consumer;
pub mod sealed;

// Chapter 11 modules - Functional Programming
pub mod pipeline;
pub mod type_classes;
pub mod pattern_matching;
pub mod closures;