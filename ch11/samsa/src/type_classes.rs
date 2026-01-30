//! Generics as Type Classes pattern implementation
//!
//! This module demonstrates using Rust's generics system to model
//! type classes from functional programming, enhancing the TypeState
//! pattern with more flexibility.

use std::marker::PhantomData;
use crate::message::current_timestamp;
use crate::error::{self, SamsaError};
use rand::random;

/// Type class for subscriptions that can be activated
pub trait Activatable {
    type Output;
    fn activate(self) -> Result<Self::Output, ActivationError>;
}

/// Type class for subscriptions that can be suspended  
pub trait Suspendable {
    type Output;
    fn suspend(self, reason: String) -> Self::Output;
}

/// Type class for subscriptions that can be canceled
pub trait Cancellable {
    type Output;
    fn cancel(self, reason: String) -> Self::Output;
}

/// Type class for subscriptions that can deliver messages
pub trait MessageDeliverable {
    fn can_deliver_messages(&self) -> bool;
    fn deliver_message(&self, message: &str) -> Result<(), DeliveryError>;
}

#[derive(Debug, Clone)]
pub enum ActivationError {
    InvalidUser,
    TopicNotFound,
    QuotaExceeded,
}

impl std::fmt::Display for ActivationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActivationError::InvalidUser => write!(f, "Invalid user"),
            ActivationError::TopicNotFound => write!(f, "Topic not found"),
            ActivationError::QuotaExceeded => write!(f, "Quota exceeded"),
        }
    }
}

impl std::error::Error for ActivationError {}

#[derive(Debug, Clone)]
pub enum DeliveryError {
    NetworkError,
    UserNotFound,
    TopicUnavailable,
}

/// State type markers
pub mod states {
    #[derive(Debug)]
    pub struct Pending;
    
    #[derive(Debug)]
    pub struct Active;
    
    #[derive(Debug)]
    pub struct Suspended;
    
    #[derive(Debug)]
    pub struct Cancelled;
}

/// Generic subscription with phantom state
#[derive(Debug)]
pub struct Subscription<S> {
    pub id: u64,
    pub user_id: u64,
    pub topic: String,
    pub created_at: u64,
    _state: PhantomData<S>,
}

impl Subscription<states::Pending> {
    pub fn new(id: u64, user_id: u64, topic: String) -> Self {
        Self {
            id,
            user_id,
            topic,
            created_at: current_timestamp(),
            _state: PhantomData,
        }
    }
}

impl Activatable for Subscription<states::Pending> {
    type Output = Subscription<states::Active>;
    
    fn activate(self) -> Result<Self::Output, ActivationError> {
        if self.user_id == 0 {
            return Err(ActivationError::InvalidUser);
        }
        
        if self.topic.is_empty() {
            return Err(ActivationError::TopicNotFound);
        }
        
        Ok(Subscription {
            id: self.id,
            user_id: self.user_id,
            topic: self.topic,
            created_at: self.created_at,
            _state: PhantomData,
        })
    }
}

impl Suspendable for Subscription<states::Active> {
    type Output = Subscription<states::Suspended>;
    
    fn suspend(self, _reason: String) -> Self::Output {
        Subscription {
            id: self.id,
            user_id: self.user_id,
            topic: self.topic,
            created_at: self.created_at,
            _state: PhantomData,
        }
    }
}

impl Cancellable for Subscription<states::Active> {
    type Output = Subscription<states::Cancelled>;
    
    fn cancel(self, _reason: String) -> Self::Output {
        Subscription {
            id: self.id,
            user_id: self.user_id,
            topic: self.topic,
            created_at: self.created_at,
            _state: PhantomData,
        }
    }
}

impl Cancellable for Subscription<states::Suspended> {
    type Output = Subscription<states::Cancelled>;
    
    fn cancel(self, _reason: String) -> Self::Output {
        Subscription {
            id: self.id,
            user_id: self.user_id,
            topic: self.topic,
            created_at: self.created_at,
            _state: PhantomData,
        }
    }
}

impl MessageDeliverable for Subscription<states::Active> {
    fn can_deliver_messages(&self) -> bool {
        true
    }
    
    fn deliver_message(&self, message: &str) -> Result<(), DeliveryError> {
        println!("Delivering message '{}' to subscription {}", message, self.id);
        Ok(())
    }
}

impl MessageDeliverable for Subscription<states::Suspended> {
    fn can_deliver_messages(&self) -> bool {
        false
    }
    
    fn deliver_message(&self, _message: &str) -> Result<(), DeliveryError> {
        Err(DeliveryError::TopicUnavailable)
    }
}

/// Generic function that works with any cancellable subscription
pub fn cancel_subscription_with_audit<S>(
    subscription: Subscription<S>,
    reason: String,
) -> Subscription<states::Cancelled>
where
    Subscription<S>: Cancellable<Output = Subscription<states::Cancelled>>,
{
    println!("Auditing cancellation of subscription {}: {}", subscription.id, reason);
    subscription.cancel(reason)
}

/// Generic function for message delivery with fallback
pub fn try_deliver_message<S>(
    subscription: &Subscription<S>,
    message: &str,
) -> bool
where
    Subscription<S>: MessageDeliverable,
{
    match subscription.deliver_message(message) {
        Ok(()) => {
            println!("Message delivered successfully");
            true
        }
        Err(e) => {
            println!("Message delivery failed: {:?}", e);
            false
        }
    }
}

/// Higher-order type class for monadic operations
///
/// This demonstrates the Monad pattern from functional programming.
/// Note: This is a simplified implementation for illustrational purposes.
pub trait Monad {
    type Item;

    fn pure(item: Self::Item) -> Self;
    fn bind<F, B>(self, f: F) -> B
    where
        F: FnOnce(Self::Item) -> B;
}

/// Result monad implementation
///
/// # Panics
///
/// **Warning:** This implementation panics when `bind()` is called on an `Err` value.
/// This is a simplified demonstration of the monad pattern. In production code,
/// use Rust's built-in `?` operator, `and_then()`, or `map()` instead.
impl<T, E> Monad for Result<T, E> {
    type Item = T;
    
    fn pure(item: Self::Item) -> Self {
        Ok(item)
    }
    
    fn bind<F, B>(self, f: F) -> B
    where
        F: FnOnce(Self::Item) -> B,
    {
        match self {
            Ok(val) => f(val),
            Err(_) => panic!("Cannot bind on Err"), // Simplified for demo
        }
    }
}

/// Type class for foldable collections
pub trait Foldable {
    type Item;
    
    fn fold_left<B, F>(self, init: B, f: F) -> B
    where
        F: Fn(B, Self::Item) -> B;
        
    fn fold_right<B, F>(self, init: B, f: F) -> B
    where
        F: Fn(Self::Item, B) -> B;
}

impl<T> Foldable for Vec<T> {
    type Item = T;
    
    fn fold_left<B, F>(self, init: B, f: F) -> B
    where
        F: Fn(B, Self::Item) -> B,
    {
        self.into_iter().fold(init, f)
    }
    
    fn fold_right<B, F>(self, init: B, f: F) -> B
    where
        F: Fn(Self::Item, B) -> B,
    {
        self.into_iter().rev().fold(init, |acc, item| f(item, acc))
    }
}

/// Type class for mappable functors
pub trait Functor {
    type Item;
    type Output<B>;
    
    fn map<B, F>(self, f: F) -> Self::Output<B>
    where
        F: FnOnce(Self::Item) -> B;
}

/// Option functor implementation
impl<T> Functor for Option<T> {
    type Item = T;
    type Output<B> = Option<B>;
    
    fn map<B, F>(self, f: F) -> Self::Output<B>
    where
        F: FnOnce(Self::Item) -> B,
    {
        self.map(f)
    }
}

/// Type class for filtering operations
pub trait Filterable {
    type Item;
    
    fn filter<F>(self, predicate: F) -> Self
    where
        F: Fn(&Self::Item) -> bool;
}

impl<T> Filterable for Vec<T> {
    type Item = T;
    
    fn filter<F>(self, predicate: F) -> Self
    where
        F: Fn(&Self::Item) -> bool,
    {
        self.into_iter().filter(|item| predicate(item)).collect()
    }
}

/// Subscription manager using type classes
pub struct SubscriptionManager {
    active_subscriptions: Vec<Subscription<states::Active>>,
    suspended_subscriptions: Vec<Subscription<states::Suspended>>,
    cancelled_subscriptions: Vec<Subscription<states::Cancelled>>,
}

impl Default for SubscriptionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SubscriptionManager {
    pub fn new() -> Self {
        Self {
            active_subscriptions: Vec::new(),
            suspended_subscriptions: Vec::new(),
            cancelled_subscriptions: Vec::new(),
        }
    }
    
    pub fn create_subscription(&mut self, user_id: u64, topic: String) -> Result<u64, ActivationError> {
        let id = random();
        let pending = Subscription::new(id, user_id, topic);
        let active = pending.activate()?;
        
        self.active_subscriptions.push(active);
        Ok(id)
    }
    
    pub fn suspend_subscription(&mut self, id: u64, reason: String) -> error::Result<()> {
        if let Some(pos) = self.active_subscriptions.iter().position(|s| s.id == id) {
            let subscription = self.active_subscriptions.remove(pos);
            let suspended = subscription.suspend(reason);
            self.suspended_subscriptions.push(suspended);
            Ok(())
        } else {
            Err(SamsaError::consumer("Subscription not found or not active"))
        }
    }

    pub fn cancel_subscription(&mut self, id: u64, reason: String) -> error::Result<()> {
        // Try to cancel from active subscriptions
        if let Some(pos) = self.active_subscriptions.iter().position(|s| s.id == id) {
            let subscription = self.active_subscriptions.remove(pos);
            let cancelled = subscription.cancel(reason);
            self.cancelled_subscriptions.push(cancelled);
            return Ok(());
        }

        // Try to cancel from suspended subscriptions
        if let Some(pos) = self.suspended_subscriptions.iter().position(|s| s.id == id) {
            let subscription = self.suspended_subscriptions.remove(pos);
            let cancelled = subscription.cancel(reason);
            self.cancelled_subscriptions.push(cancelled);
            return Ok(());
        }

        Err(SamsaError::consumer("Subscription not found"))
    }

    pub fn broadcast_message(&self, topic: &str, message: &str) {
        for subscription in &self.active_subscriptions {
            if subscription.topic == topic {
                try_deliver_message(subscription, message);
            }
        }
    }
}

/// Demonstrate type class constraints in generic functions
/// Note: This is a simplified example for demonstration
pub fn process_option_string(value: Option<String>) -> Option<usize> {
    println!("Processing value: {:?}", value);
    value.map(|s| {
        println!("Mapping string: {}", s);
        s.len()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscription_lifecycle() {
        let pending = Subscription::new(1, 123, "test.topic".to_string());
        let active = pending.activate().unwrap();
        let suspended = active.suspend("Maintenance".to_string());
        let _cancelled = cancel_subscription_with_audit(suspended, "User request".to_string());
    }
    
    #[test]
    fn test_message_delivery() {
        let pending = Subscription::new(2, 456, "events".to_string());
        let active = pending.activate().unwrap();
        
        assert!(try_deliver_message(&active, "Test message"));
        
        let suspended = active.suspend("Pause".to_string());
        assert!(!try_deliver_message(&suspended, "Should fail"));
    }
    
    #[test]
    fn test_type_class_operations() {
        // Foldable
        let numbers = vec![1, 2, 3, 4, 5];
        let sum = numbers.fold_left(0, |acc, x| acc + x);
        assert_eq!(sum, 15);
        
        // Functor
        let opt = Some("hello".to_string());
        let mapped = opt.map(|s| s.len());
        assert_eq!(mapped, Some(5));
        
        // Filterable
        let items = vec![1, 2, 3, 4, 5];
        let evens = items.filter(|x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4]);
    }
}