//! Configuration management demonstrating Result and Option patterns
//!
//! This module shows how to use Result and Option types for:
//! - Robust configuration loading with error handling
//! - Builder pattern with validation
//! - Configuration composition and fallbacks
//! - Parse, don't validate principle

use crate::error::{SamsaError, Result};
use std::path::Path;
use std::time::Duration;
use std::num::NonZeroUsize;

/// Complete Samsa service configuration
#[derive(Debug, Clone)]
pub struct SamsaConfig {
    pub broker: BrokerConfig,
    pub log_level: String,
    pub storage_path: String,
}

impl SamsaConfig {
    /// Load configuration from a file with fallback chain
    pub fn load() -> Result<Self> {
        // Try multiple configuration locations in order
        Self::load_from_file("samsa.conf")
            .or_else(|_| Self::load_from_file("/etc/samsa/samsa.conf"))
            .or_else(|_| Self::load_from_file("/usr/local/etc/samsa.conf"))
            .or_else(|_| {
                eprintln!("Warning: Using default configuration");
                Ok(Self::default())
            })
    }

    /// Load configuration from a specific file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|_| SamsaError::config("Configuration file not found"))?;

        Self::parse_config(&content)
    }

    /// Parse configuration from text content
    fn parse_config(content: &str) -> Result<Self> {
        let mut config = Self::default();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                match key.trim() {
                    "broker_port" => {
                        let port = value.trim().parse()
                            .map_err(|_| SamsaError::config("Invalid port"))?;
                        config.broker.port = port;
                    }
                    "max_connections" => {
                        let max = value.trim().parse()
                            .map_err(|_| SamsaError::config("Invalid max_connections"))?;
                        config.broker.max_connections = max;
                    }
                    "log_level" => {
                        config.log_level = value.trim().to_string();
                    }
                    "storage_path" => {
                        config.storage_path = value.trim().to_string();
                    }
                    "buffer_size" => {
                        let size = value.trim().parse()
                            .map_err(|_| SamsaError::config("Invalid buffer_size"))?;
                        config.broker.buffer_size = NonZeroUsize::new(size)
                            .ok_or_else(|| SamsaError::config("buffer_size must be positive"))?;
                    }
                    "connection_timeout" => {
                        let timeout: u64 = value.trim().parse()
                            .map_err(|_| SamsaError::config("Invalid connection_timeout"))?;
                        config.broker.connection_timeout = Duration::from_secs(timeout);
                    }
                    _ => {} // Ignore unknown keys
                }
            }
        }

        config.validate()?;
        Ok(config)
    }

    /// Validate configuration
    fn validate(&self) -> Result<()> {
        self.broker.validate()?;

        if self.log_level.is_empty() {
            return Err(SamsaError::config("log_level cannot be empty"));
        }

        Ok(())
    }
}

impl Default for SamsaConfig {
    fn default() -> Self {
        Self {
            broker: BrokerConfig::default(),
            log_level: "info".to_string(),
            storage_path: "memory://".to_string(),
        }
    }
}

/// Broker-specific configuration
#[derive(Debug, Clone)]
pub struct BrokerConfig {
    pub port: u16,
    pub max_connections: usize,
    pub buffer_size: NonZeroUsize,
    pub connection_timeout: Duration,
    pub enable_metrics: bool,
}

impl BrokerConfig {
    /// Create a new builder for BrokerConfig
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    /// Validate broker configuration
    pub fn validate(&self) -> Result<()> {
        if self.port == 0 {
            return Err(SamsaError::config("Port cannot be zero"));
        }

        if self.port < 1024 {
            return Err(SamsaError::config("Port cannot be in reserved range (0-1023)"));
        }

        if self.max_connections == 0 {
            return Err(SamsaError::config("Max connections must be positive"));
        }

        Ok(())
    }
}

impl Default for BrokerConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            max_connections: 1000,
            buffer_size: NonZeroUsize::new(4096).unwrap(),
            connection_timeout: Duration::from_secs(30),
            enable_metrics: false,
        }
    }
}

/// Builder for BrokerConfig demonstrating builder pattern with validation
pub struct ConfigBuilder {
    port: Option<u16>,
    max_connections: Option<usize>,
    buffer_size: Option<NonZeroUsize>,
    connection_timeout: Option<Duration>,
    enable_metrics: bool,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            port: None,
            max_connections: None,
            buffer_size: None,
            connection_timeout: None,
            enable_metrics: false,
        }
    }

    pub fn port(mut self, port: u16) -> Result<Self> {
        if port < 1024 {
            return Err(SamsaError::config("Port cannot be in reserved range"));
        }
        self.port = Some(port);
        Ok(self)
    }

    pub fn max_connections(mut self, max: usize) -> Result<Self> {
        if max == 0 {
            return Err(SamsaError::config("Max connections must be positive"));
        }
        self.max_connections = Some(max);
        Ok(self)
    }

    pub fn buffer_size(mut self, size: usize) -> Result<Self> {
        self.buffer_size = Some(
            NonZeroUsize::new(size)
                .ok_or_else(|| SamsaError::config("Buffer size must be positive"))?
        );
        Ok(self)
    }

    pub fn connection_timeout(mut self, timeout: Duration) -> Self {
        self.connection_timeout = Some(timeout);
        self
    }

    pub fn enable_metrics(mut self, enable: bool) -> Self {
        self.enable_metrics = enable;
        self
    }

    pub fn build(self) -> BrokerConfig {
        BrokerConfig {
            port: self.port.unwrap_or(8080),
            max_connections: self.max_connections.unwrap_or(1000),
            buffer_size: self.buffer_size.unwrap_or_else(|| NonZeroUsize::new(4096).unwrap()),
            connection_timeout: self.connection_timeout.unwrap_or(Duration::from_secs(30)),
            enable_metrics: self.enable_metrics,
        }
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = SamsaConfig::default();
        assert_eq!(config.broker.port, 8080);
        assert_eq!(config.broker.max_connections, 1000);
    }

    #[test]
    fn test_config_builder() {
        let config = BrokerConfig::builder()
            .port(9000).unwrap()
            .max_connections(500).unwrap()
            .enable_metrics(true)
            .build();

        assert_eq!(config.port, 9000);
        assert_eq!(config.max_connections, 500);
        assert!(config.enable_metrics);
    }

    #[test]
    fn test_validation() {
        let mut config = BrokerConfig::default();
        assert!(config.validate().is_ok());

        config.port = 0;
        assert!(config.validate().is_err());

        config.port = 500; // Reserved range
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_builder_validation() {
        // Attempting to set a reserved port should fail
        let result = ConfigBuilder::new().port(80);
        assert!(result.is_err());

        // Valid port should succeed
        let result = ConfigBuilder::new().port(8080);
        assert!(result.is_ok());
    }
}
