// adapter.rs - Adapter pattern implementation

use crate::config::AngleMode;

// Science operations interface
pub trait ScientificOperations: Send + Sync {
    fn sin(&self, angle: f64) -> f64;
    fn cos(&self, angle: f64) -> f64;
    fn tan(&self, angle: f64) -> f64;
    fn log(&self, value: f64, base: f64) -> Result<f64, String>;
}

// Standard implementation using Rust's math functions
pub struct StandardScientificOperations {
    pub angle_mode: AngleMode,
}

impl ScientificOperations for StandardScientificOperations {
    fn sin(&self, angle: f64) -> f64 {
        match self.angle_mode {
            AngleMode::Radians => angle.sin(),
            AngleMode::Degrees => (angle * std::f64::consts::PI / 180.0).sin(),
        }
    }
    
    fn cos(&self, angle: f64) -> f64 {
        match self.angle_mode {
            AngleMode::Radians => angle.cos(),
            AngleMode::Degrees => (angle * std::f64::consts::PI / 180.0).cos(),
        }
    }
    
    fn tan(&self, angle: f64) -> f64 {
        match self.angle_mode {
            AngleMode::Radians => angle.tan(),
            AngleMode::Degrees => (angle * std::f64::consts::PI / 180.0).tan(),
        }
    }
    
    fn log(&self, value: f64, base: f64) -> Result<f64, String> {
        if value <= 0.0 {
            return Err("Cannot take logarithm of non-positive number".to_string());
        }
        if base <= 0.0 || base == 1.0 {
            return Err("Invalid logarithm base".to_string());
        }
        
        Ok(value.log(base))
    }
}

// Adapter for an external library (simplified)
pub struct ExternalLibraryAdapter {
    pub angle_mode: AngleMode,
}

impl ExternalLibraryAdapter {
    pub fn new(angle_mode: AngleMode) -> Self {
        Self { angle_mode }
    }
}

impl ScientificOperations for ExternalLibraryAdapter {
    fn sin(&self, angle: f64) -> f64 {
        // In a real adapter, this would call the external library
        match self.angle_mode {
            AngleMode::Radians => angle.sin(),
            AngleMode::Degrees => angle.to_radians().sin(),
        }
    }
    
    fn cos(&self, angle: f64) -> f64 {
        match self.angle_mode {
            AngleMode::Radians => angle.cos(),
            AngleMode::Degrees => angle.to_radians().cos(),
        }
    }
    
    fn tan(&self, angle: f64) -> f64 {
        match self.angle_mode {
            AngleMode::Radians => angle.tan(),
            AngleMode::Degrees => angle.to_radians().tan(),
        }
    }
    
    fn log(&self, value: f64, base: f64) -> Result<f64, String> {
        if value <= 0.0 || base <= 0.0 || base == 1.0 {
            return Err("Invalid logarithm arguments".to_string());
        }
        
        Ok(value.log(base))
    }
}
