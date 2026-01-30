//! Function pipeline patterns for Samsa
//!
//! This module demonstrates functional programming patterns using
//! iterator chains, custom combinators, and lazy evaluation.

use crate::message::{Message, Event, current_timestamp};
use std::collections::HashMap;

/// Statistics about subscription events
#[derive(Debug, Default)]
pub struct SubscriptionStats {
    pub total_valid: usize,
    pub subscriptions_by_topic: HashMap<String, usize>,
    pub recent_count: usize,
}

/// A subscription event in the system
#[derive(Debug, Clone)]
pub struct SubscriptionEvent {
    pub user_id: u64,
    pub topic: String,
    pub timestamp: u64,
    pub subscription_type: SubscriptionType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SubscriptionType {
    Subscribe,
    Unsubscribe,
    Invalid,
}

impl SubscriptionEvent {
    pub fn is_valid(&self) -> bool {
        self.subscription_type != SubscriptionType::Invalid 
            && !self.topic.is_empty()
            && self.user_id > 0
    }
    
    pub fn is_subscription(&self) -> bool {
        self.subscription_type == SubscriptionType::Subscribe
    }
}

/// Extension trait for subscription event processing
pub trait SubscriptionProcessing: Iterator<Item = SubscriptionEvent> + Sized {
    fn valid_subscriptions(self) -> impl Iterator<Item = SubscriptionEvent> {
        self.filter(|event| event.is_valid())
            .filter(|event| event.is_subscription())
    }
    
    fn count_by_topic(self) -> HashMap<String, usize> {
        self.fold(HashMap::new(), |mut acc, event| {
            *acc.entry(event.topic).or_insert(0) += 1;
            acc
        })
    }
    
    fn recent_events(self, cutoff_timestamp: u64) -> impl Iterator<Item = SubscriptionEvent> {
        self.filter(move |event| event.timestamp >= cutoff_timestamp)
    }
}

impl<I> SubscriptionProcessing for I where I: Iterator<Item = SubscriptionEvent> {}

/// Process subscription events using function pipelines
pub fn process_subscription_events(events: Vec<SubscriptionEvent>) -> SubscriptionStats {
    let valid_events: Vec<_> = events
        .iter()
        .filter(|event| event.is_valid())
        .cloned()
        .collect();
    
    let subscriptions_by_topic = valid_events
        .iter()
        .filter(|event| event.is_subscription())
        .map(|event| &event.topic)
        .fold(HashMap::new(), |mut acc, topic| {
            *acc.entry(topic.clone()).or_insert(0) += 1;
            acc
        });
    
    SubscriptionStats {
        total_valid: valid_events.len(),
        subscriptions_by_topic,
        recent_count: 0,
    }
}

/// Process recent subscriptions using custom combinators
pub fn analyze_recent_subscriptions(
    events: Vec<SubscriptionEvent>, 
    cutoff_timestamp: u64
) -> HashMap<String, usize> {
    events
        .into_iter()
        .recent_events(cutoff_timestamp)
        .valid_subscriptions()
        .count_by_topic()
}

/// Message processing pipeline for filtering and transformation
pub struct MessagePipeline<F> {
    filters: Vec<F>,
}

impl<F> Default for MessagePipeline<F>
where
    F: Fn(&Message) -> bool,
 {
    fn default() -> Self {
        Self::new()
    }
}

impl<F> MessagePipeline<F>
where
    F: Fn(&Message) -> bool,
{
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
        }
    }
    
    pub fn add_filter(mut self, filter: F) -> Self {
        self.filters.push(filter);
        self
    }
    
    pub fn process(&self, messages: Vec<Message>) -> Vec<Message> {
        messages
            .into_iter()
            .filter(|msg| self.filters.iter().all(|f| f(msg)))
            .collect()
    }
}

/// Event transformation pipeline
pub trait EventTransformation: Iterator<Item = Event> + Sized {
    fn with_topic_prefix(self, prefix: &str) -> impl Iterator<Item = Event> {
        let prefix = prefix.to_string();
        self.filter(move |event| event.message.topic.starts_with(&prefix))
    }
    
    fn transform_values<F>(self, f: F) -> impl Iterator<Item = Event>
    where
        F: Fn(Vec<u8>) -> Vec<u8>,
    {
        self.map(move |mut event| {
            event.message.value = f(event.message.value);
            event
        })
    }
    
    fn batch(self, size: usize) -> BatchIterator<Self> {
        BatchIterator::new(self, size)
    }
}

impl<I> EventTransformation for I where I: Iterator<Item = Event> {}

/// Iterator for batching events
pub struct BatchIterator<I: Iterator<Item = Event>> {
    iter: I,
    batch_size: usize,
}

impl<I: Iterator<Item = Event>> BatchIterator<I> {
    fn new(iter: I, batch_size: usize) -> Self {
        Self { iter, batch_size }
    }
}

impl<I: Iterator<Item = Event>> Iterator for BatchIterator<I> {
    type Item = Vec<Event>;
    
    fn next(&mut self) -> Option<Self::Item> {
        let mut batch = Vec::with_capacity(self.batch_size);
        
        for _ in 0..self.batch_size {
            match self.iter.next() {
                Some(event) => batch.push(event),
                None => break,
            }
        }
        
        if batch.is_empty() {
            None
        } else {
            Some(batch)
        }
    }
}

/// Advanced pipeline operations for complex transformations
type MessageBatchOperation = Box<dyn Fn(Vec<Message>) -> Vec<Message>>;

pub struct AdvancedPipeline {
    operations: Vec<MessageBatchOperation>,
}

impl Default for AdvancedPipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl AdvancedPipeline {
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }
    
    pub fn filter<F>(mut self, predicate: F) -> Self
    where
        F: Fn(&Message) -> bool + 'static,
    {
        self.operations.push(Box::new(move |messages| {
            messages.into_iter().filter(|m| predicate(m)).collect()
        }));
        self
    }
    
    pub fn map<F>(mut self, mapper: F) -> Self
    where
        F: Fn(Message) -> Message + 'static,
    {
        self.operations.push(Box::new(move |messages| {
            messages.into_iter().map(&mapper).collect()
        }));
        self
    }
    
    pub fn flat_map<F>(mut self, mapper: F) -> Self
    where
        F: Fn(Message) -> Vec<Message> + 'static,
    {
        self.operations.push(Box::new(move |messages| {
            messages.into_iter().flat_map(&mapper).collect()
        }));
        self
    }
    
    pub fn execute(self, messages: Vec<Message>) -> Vec<Message> {
        self.operations.into_iter().fold(messages, |acc, op| op(acc))
    }
}

/// Functional composition helpers
pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

pub fn pipe<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    compose(f, g)
}

/// Create a pipeline of message transformations
pub fn create_message_enrichment_pipeline() -> impl Fn(Message) -> Message {
    let add_timestamp = |mut msg: Message| {
        msg.value.extend_from_slice(b"|timestamp:");
        msg.value.extend_from_slice(current_timestamp().to_string().as_bytes());
        msg
    };
    
    let normalize_topic = |mut msg: Message| {
        msg.topic = msg.topic.to_lowercase();
        msg
    };
    
    let add_size_metadata = |mut msg: Message| {
        let size = msg.value.len();
        msg.key = Some(format!("size:{}", size));
        msg
    };
    
    pipe(add_timestamp, pipe(normalize_topic, add_size_metadata))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscription_processing() {
        let events = vec![
            SubscriptionEvent {
                user_id: 1,
                topic: "news".to_string(),
                timestamp: 100,
                subscription_type: SubscriptionType::Subscribe,
            },
            SubscriptionEvent {
                user_id: 2,
                topic: "news".to_string(),
                timestamp: 200,
                subscription_type: SubscriptionType::Subscribe,
            },
            SubscriptionEvent {
                user_id: 3,
                topic: "sports".to_string(),
                timestamp: 300,
                subscription_type: SubscriptionType::Subscribe,
            },
        ];
        
        let stats = process_subscription_events(events);
        assert_eq!(stats.total_valid, 3);
        assert_eq!(stats.subscriptions_by_topic.get("news"), Some(&2));
        assert_eq!(stats.subscriptions_by_topic.get("sports"), Some(&1));
    }
    
    #[test]
    fn test_pipeline_composition() {
        let pipeline = create_message_enrichment_pipeline();
        
        let msg = Message::new("TEST.TOPIC", None, b"Hello");
        let enriched = pipeline(msg);
        
        assert_eq!(enriched.topic, "test.topic");
        assert!(enriched.value.starts_with(b"Hello|timestamp:"));
        assert!(enriched.key.is_some());
    }
}