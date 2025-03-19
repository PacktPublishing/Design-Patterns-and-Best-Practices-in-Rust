// config.rs - Configuration (alternative to Singleton)

use std::sync::OnceLock;
use crate::token::NumberFormat;

#[derive(Debug, Clone)]
pub enum AngleMode {
    Degrees,
    Radians,
}

#[derive(Debug, Clone)]
pub struct CalculatorConfig {
    pub precision: u32,
    pub angle_mode: AngleMode,
    pub notation: NumberFormat,
}

impl Default for CalculatorConfig {
    fn default() -> Self {
        Self {
            precision: 10,
            angle_mode: AngleMode::Radians,
            notation: NumberFormat::Decimal,
        }
    }
}

impl CalculatorConfig {
    // Factory methods for common configurations
    pub fn scientific() -> Self {
        Self {
            precision: 15,
            angle_mode: AngleMode::Radians,
            notation: NumberFormat::Scientific,
        }
    }

    pub fn engineering() -> Self {
        Self {
            notation: NumberFormat::Engineering,
            ..Default::default()
        }
    }
}

// Constants
pub const DEFAULT_PRECISION: u32 = 10;
pub const MAX_PRECISION: u32 = 100;

// If we need a global configuration (alternative to Singleton)
static CONFIG: OnceLock<CalculatorConfig> = OnceLock::new();

pub fn get_global_config() -> &'static CalculatorConfig {
    CONFIG.get_or_init(|| {
        // In a real application, this might load from a file or environment
        CalculatorConfig::default()
    })
}

// Thread-safe calculator with shared config
use std::sync::{Arc, Mutex};

pub struct CalculatorPool {
    shared_config: Arc<CalculatorConfig>,
    // In a real application, this would store calculator instances
    _calculators: Vec<()>,
}

impl CalculatorPool {
    pub fn new(config: CalculatorConfig) -> Self {
        Self {
            shared_config: Arc::new(config),
            _calculators: Vec::new(),
        }
    }

    pub fn get_config(&self) -> Arc<CalculatorConfig> {
        Arc::clone(&self.shared_config)
    }
}
