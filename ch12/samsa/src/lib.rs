//! Samsa - A complete publish/subscribe microservice (Chapter 12)
//!
//! This is the culminating implementation that demonstrates all patterns from Chapters 9-11
//! plus Chapter 12's unique Rust features:
//!
//! - Result and Option patterns for robust error handling
//! - Block expressions for elegant initialization
//! - RAII patterns for automatic resource management
//! - Concise, expressive patterns leveraging Rust's unique features
//!
//! This complete service is production-ready and fully functional.

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

// Chapter 12 additions - Unique Rust Features
pub use config::{SamsaConfig, BrokerConfig, ConfigBuilder};
pub use service::{BrokerService, ServiceManager};
pub use resources::{
    ConnectionPool, ConnectionGuard, TransactionGuard,
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

// Chapter 12 modules - Unique Rust Features
pub mod config;
pub mod service;
pub mod resources;