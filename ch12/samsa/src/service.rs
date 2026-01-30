//! Service layer demonstrating block expressions and RAII
//!
//! This module shows:
//! - Block expressions for conditional initialization
//! - Service lifecycle management
//! - Cascading cleanup on shutdown
//! - Error handling patterns in service context

use crate::config::{SamsaConfig, BrokerConfig};
use crate::resources::ConnectionPool;
use crate::error::{SamsaError, Result};
use crate::{Broker, Message};
use std::sync::Arc;
use std::time::Duration;

/// Complete broker service with managed lifecycle
pub struct BrokerService {
    broker: Arc<Broker>,
    connection_pool: Option<Arc<ConnectionPool>>,
    config: BrokerConfig,
}

impl BrokerService {
    /// Create a new broker service using block expressions for initialization
    pub fn new(config: BrokerConfig) -> Result<Self> {
        // Block expression for conditional storage initialization
        let broker = {
            let broker = Broker::new();
            Arc::new(broker)
        };

        // Block expression for optional connection pool
        let connection_pool = {
            if config.max_connections > 0 {
                Some(ConnectionPool::new(config.max_connections))
            } else {
                None
            }
        };

        // Validate the service is ready
        let service = Self {
            broker,
            connection_pool,
            config,
        };

        service.validate()?;
        Ok(service)
    }

    /// Validate service state
    fn validate(&self) -> Result<()> {
        if self.config.port == 0 {
            return Err(SamsaError::service("Port cannot be zero"));
        }
        Ok(())
    }

    /// Publish a message through the service
    pub fn publish(&self, message: Message) -> Result<u64> {
        self.broker.publish(message)
    }

    /// Get service metrics (if enabled)
    pub fn metrics_enabled(&self) -> bool {
        self.config.enable_metrics
    }

    /// Graceful shutdown
    pub fn shutdown(self) -> Result<()> {
        // Explicit drop order for clean shutdown
        drop(self.connection_pool);
        drop(self.broker);
        Ok(())
    }
}

/// Service manager with cascading cleanup
pub struct ServiceManager {
    broker_service: Option<BrokerService>,
    start_time: std::time::Instant,
}

impl ServiceManager {
    /// Create and start a new service manager
    pub fn start(config: SamsaConfig) -> Result<Self> {
        // Block expression for service initialization
        let broker_service = {
            let service = BrokerService::new(config.broker)?;
            Some(service)
        };

        Ok(Self {
            broker_service,
            start_time: std::time::Instant::now(),
        })
    }

    /// Get service uptime
    pub fn uptime(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Publish a message
    pub fn publish(&self, message: Message) -> Result<u64> {
        self.broker_service
            .as_ref()
            .ok_or(SamsaError::service("Service not running"))?
            .publish(message)
    }

    /// Stop the service
    pub fn stop(&mut self) -> Result<()> {
        if let Some(service) = self.broker_service.take() {
            service.shutdown()?;
        }
        Ok(())
    }
}

impl Drop for ServiceManager {
    /// Cascading cleanup: resources are dropped in order
    fn drop(&mut self) {
        // Attempt graceful shutdown
        if let Some(service) = self.broker_service.take() {
            let _ = service.shutdown();
        }
    }
}

/// Request processing pipeline demonstrating block expressions
pub fn process_request(message: Message, service: &BrokerService) -> Result<ProcessingResult> {
    // Block expression for validation
    let validated = {
        if message.topic.is_empty() {
            return Err(SamsaError::service("Empty topic"));
        }
        if message.value.is_empty() {
            return Err(SamsaError::service("Empty message"));
        }
        message
    };

    // Block expression for processing with metrics
    let offset = {
        let result = service.publish(validated)?;

        if service.metrics_enabled() {
            // Would record metrics here
        }

        result
    };

    Ok(ProcessingResult {
        offset,
        processed_at: std::time::Instant::now(),
    })
}

/// Result of message processing
#[derive(Debug)]
pub struct ProcessingResult {
    pub offset: u64,
    pub processed_at: std::time::Instant,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ConfigBuilder;

    #[test]
    fn test_service_creation() {
        let config = ConfigBuilder::new()
            .port(9000).unwrap()
            .max_connections(10).unwrap()
            .build();

        let service = BrokerService::new(config).unwrap();
        assert!(service.validate().is_ok());
    }

    #[test]
    fn test_service_manager() {
        let config = SamsaConfig::default();
        let mut manager = ServiceManager::start(config).unwrap();

        let message = Message::text("test.topic", "Hello");
        let result = manager.publish(message);
        assert!(result.is_ok());

        // Clean shutdown
        manager.stop().unwrap();

        // Service is stopped
        let message2 = Message::text("test.topic", "World");
        assert!(manager.publish(message2).is_err());
    }

    #[test]
    fn test_service_uptime() {
        let config = SamsaConfig::default();
        let manager = ServiceManager::start(config).unwrap();

        std::thread::sleep(Duration::from_millis(10));
        let uptime = manager.uptime();
        assert!(uptime.as_millis() >= 10);
    }

    #[test]
    fn test_cascading_drop() {
        let config = SamsaConfig::default();
        let manager = ServiceManager::start(config).unwrap();

        // Dropping manager should cleanly shut down service
        drop(manager);
        // If this doesn't panic, cascading cleanup worked
    }
}
