//! Samsa - A simple publish/subscribe microservice
//! 
//! This crate demonstrates architectural patterns through building
//! a working message broker system.

pub use message::{Message, Event};
pub use broker::Broker;
pub use producer::Producer;
pub use consumer::Consumer;

mod message;
mod broker;
mod producer;
mod consumer;
mod storage;