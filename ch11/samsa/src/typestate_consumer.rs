//! TypeState pattern implementation for consumer lifecycle
//!
//! This module demonstrates the TypeState pattern by encoding
//! consumer states in the type system to prevent invalid operations.

use std::marker::PhantomData;
use std::sync::Arc;
use crate::broker::Broker;
use crate::types::{TopicId, ConsumerId};
use crate::message::Event;
use crate::error::SamsaError;

/// Type-level state markers for consumer lifecycle
pub mod states {
    /// Consumer has been created but not connected
    #[derive(Debug)]
    pub struct Disconnected;
    
    /// Consumer is connected to the broker
    #[derive(Debug)]
    pub struct Connected;
    
    /// Consumer is subscribed to a topic
    #[derive(Debug)]
    pub struct Subscribed;
    
    /// Consumer is paused and not receiving messages
    #[derive(Debug)]
    pub struct Paused;
}

/// Connection information for a consumer
#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub broker_address: String,
    pub consumer_group: Option<String>,
}

impl ConnectionInfo {
    pub fn new(broker_address: String, consumer_group: Option<String>) -> Self {
        Self {
            broker_address,
            consumer_group,
        }
    }
}

/// Consumer parameterized by state
///
/// The state parameter ensures only valid operations are available
/// at compile time based on the current consumer state.
#[derive(Debug)]
pub struct Consumer<State> {
    broker: Arc<Broker>,
    topic: Option<TopicId>,
    consumer_id: ConsumerId,
    connection_info: Option<ConnectionInfo>,
    state: PhantomData<State>,
}

/// Type aliases for different consumer states
pub type DisconnectedConsumer = Consumer<states::Disconnected>;
pub type ConnectedConsumer = Consumer<states::Connected>;
pub type SubscribedConsumer = Consumer<states::Subscribed>;
pub type PausedConsumer = Consumer<states::Paused>;

/// Implementation for disconnected consumers
impl Consumer<states::Disconnected> {
    /// Create a new disconnected consumer
    pub fn new(consumer_id: ConsumerId, broker: Arc<Broker>) -> Self {
        Self {
            broker,
            topic: None,
            consumer_id,
            connection_info: None,
            state: PhantomData,
        }
    }
    
    /// Connect to the broker, transitioning to Connected state
    pub fn connect(
        self,
        connection_info: ConnectionInfo
    ) -> Result<Consumer<states::Connected>, SamsaError> {
        // Simulate connection logic
        if connection_info.broker_address.is_empty() {
            return Err(SamsaError::connection("Empty broker address"));
        }
        
        println!("Consumer {} connecting to {}", 
                 self.consumer_id, 
                 connection_info.broker_address);
        
        Ok(Consumer {
            broker: self.broker,
            topic: self.topic,
            consumer_id: self.consumer_id,
            connection_info: Some(connection_info),
            state: PhantomData,
        })
    }
}

/// Implementation for connected consumers
impl Consumer<states::Connected> {
    /// Subscribe to a topic, transitioning to Subscribed state
    pub fn subscribe(
        mut self,
        topic: TopicId
    ) -> Result<Consumer<states::Subscribed>, SamsaError> {
        // Check if already subscribed
        if self.topic.is_some() {
            return Err(SamsaError::consumer("Consumer is already subscribed"));
        }
        
        // Simulate subscription logic
        println!("Consumer {} subscribing to topic {}", self.consumer_id, topic);
        
        self.topic = Some(topic);
        
        Ok(Consumer {
            broker: self.broker,
            topic: self.topic,
            consumer_id: self.consumer_id,
            connection_info: self.connection_info,
            state: PhantomData,
        })
    }
    
    /// Disconnect, returning to Disconnected state
    pub fn disconnect(self) -> Consumer<states::Disconnected> {
        println!("Consumer {} disconnecting", self.consumer_id);
        
        Consumer {
            broker: self.broker,
            topic: None,
            consumer_id: self.consumer_id,
            connection_info: None,
            state: PhantomData,
        }
    }
}

/// Implementation for subscribed consumers
impl Consumer<states::Subscribed> {
    /// Receive the next message (if available)
    pub fn receive(&self) -> Option<Event> {
        if let Some(ref topic) = self.topic {
            println!("Consumer {} receiving from topic {}", self.consumer_id, topic);
            // In a real implementation, this would poll the broker
            None
        } else {
            None
        }
    }
    
    /// Pause message consumption, transitioning to Paused state
    pub fn pause(self) -> Consumer<states::Paused> {
        println!("Consumer {} paused", self.consumer_id);
        
        Consumer {
            broker: self.broker,
            topic: self.topic,
            consumer_id: self.consumer_id,
            connection_info: self.connection_info,
            state: PhantomData,
        }
    }
    
    /// Unsubscribe from the topic, returning to Connected state
    pub fn unsubscribe(mut self) -> Consumer<states::Connected> {
        if let Some(ref topic) = self.topic {
            println!("Consumer {} unsubscribing from topic {}", self.consumer_id, topic);
        }
        
        self.topic = None;
        
        Consumer {
            broker: self.broker,
            topic: self.topic,
            consumer_id: self.consumer_id,
            connection_info: self.connection_info,
            state: PhantomData,
        }
    }
    
    /// Disconnect, returning to Disconnected state
    pub fn disconnect(self) -> Consumer<states::Disconnected> {
        println!("Consumer {} disconnecting", self.consumer_id);
        
        Consumer {
            broker: self.broker,
            topic: None,
            consumer_id: self.consumer_id,
            connection_info: None,
            state: PhantomData,
        }
    }
}

/// Implementation for paused consumers
impl Consumer<states::Paused> {
    /// Resume message consumption, returning to Subscribed state
    pub fn resume(self) -> Consumer<states::Subscribed> {
        println!("Consumer {} resumed", self.consumer_id);
        
        Consumer {
            broker: self.broker,
            topic: self.topic,
            consumer_id: self.consumer_id,
            connection_info: self.connection_info,
            state: PhantomData,
        }
    }
    
    /// Unsubscribe while paused, returning to Connected state
    pub fn unsubscribe(mut self) -> Consumer<states::Connected> {
        if let Some(ref topic) = self.topic {
            println!("Consumer {} unsubscribing from topic {} while paused", 
                     self.consumer_id, topic);
        }
        
        self.topic = None;
        
        Consumer {
            broker: self.broker,
            topic: self.topic,
            consumer_id: self.consumer_id,
            connection_info: self.connection_info,
            state: PhantomData,
        }
    }
    
    /// Disconnect while paused, returning to Disconnected state
    pub fn disconnect(self) -> Consumer<states::Disconnected> {
        println!("Consumer {} disconnecting while paused", self.consumer_id);
        
        Consumer {
            broker: self.broker,
            topic: None,
            consumer_id: self.consumer_id,
            connection_info: None,
            state: PhantomData,
        }
    }
}

/// Common operations available to all consumer states
impl<State> Consumer<State> {
    /// Get the consumer ID
    pub fn id(&self) -> &ConsumerId {
        &self.consumer_id
    }
    
    /// Get the current topic (if subscribed)
    pub fn topic(&self) -> Option<&TopicId> {
        self.topic.as_ref()
    }
    
    /// Check if the consumer has connection info
    pub fn is_connected(&self) -> bool {
        self.connection_info.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::broker::Broker;

    #[test]
    fn consumer_lifecycle() {
        let broker = Arc::new(Broker::new());
        let consumer_id = ConsumerId::new("test-consumer").unwrap();
        let topic = TopicId::new("test.topic").unwrap();
        
        // Start disconnected
        let consumer = Consumer::new(consumer_id, broker);
        
        // Connect
        let connection_info = ConnectionInfo::new("localhost:9092".to_string(), None);
        let consumer = consumer.connect(connection_info).unwrap();
        
        // Subscribe
        let consumer = consumer.subscribe(topic).unwrap();
        
        // Pause and resume
        let consumer = consumer.pause();
        let consumer = consumer.resume();
        
        // Unsubscribe and disconnect
        let consumer = consumer.unsubscribe();
        let _consumer = consumer.disconnect();
    }
    
    #[test]
    fn invalid_transitions_prevented_at_compile_time() {
        // The typestate pattern prevents invalid operations at compile time
        // These operations would not compile on an unsubscribed consumer:
        // consumer.receive(); // Only available for subscribed consumers
        // consumer.pause();   // Only available for subscribed consumers
        // consumer.resume();  // Only available for paused consumers
    }
}