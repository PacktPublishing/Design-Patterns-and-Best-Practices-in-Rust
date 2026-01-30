//! Samsa - A simple publish/subscribe microservice
//!
//! This crate demonstrates architectural patterns through building
//! a working message broker system.

// Core error handling
pub use error::{SamsaError, Result};

pub use message::{Message, Event};
pub use broker::Broker;
pub use producer::Producer;
pub use consumer::Consumer;

mod error;
mod message;
mod broker;
mod producer;
mod consumer;
mod storage;