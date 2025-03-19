// adapter.rs - Adapter pattern implementation

use std::f64::consts::PI;
use crate::config::AngleMode;
use crate::expression::Expression;
use std::collections::HashMap;

// Interface for scientific calculations
pub trait ScientificOperations {
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
            AngleMode::Degrees => (angle * PI / 180.0).sin(),
        }
    }
    
    fn cos(&self, angle: f64) -> f64 {
        match self.angle_mode {
            AngleMode::Radians => angle.cos(),
            AngleMode::Degrees => (angle * PI / 180.0).cos(),
        }
    }
    
    fn tan(&self, angle: f64) -> f64 {
        match self.angle_mode {
            AngleMode::Radians => angle.tan(),
            AngleMode::Degrees => (angle * PI / 180.0).tan(),
        }
    }
    
    fn log(&self, value: f64, base: f64) -> Result<f64, String> {
        if value <= 0.0 {
            return Err("Cannot take logarithm of non-positive number".to_string());
        }
        
        if base <= 0.0 || base == 1.0 {
            return Err("Invalid logarithm base".to_string());
        }
        
        Ok((value.ln()) / (base.ln()))
    }
}

// Adapter for a hypothetical external math library
pub struct ExternalLibraryAdapter {
    // In a real implementation, this would contain a reference to the external library
    angle_mode: AngleMode,
}

impl ExternalLibraryAdapter {
    pub fn new(angle_mode: AngleMode) -> Self {
        Self { angle_mode }
    }
    
    // This would be a helper that converts to the format needed by the external library
    fn convert_angle(&self, angle: f64) -> f64 {
        // The external library might only work with radians
        match self.angle_mode {
            AngleMode::Radians => angle,
            AngleMode::Degrees => angle * PI / 180.0,
        }
    }
}

impl ScientificOperations for ExternalLibraryAdapter {
    fn sin(&self, angle: f64) -> f64 {
        // In a real implementation, we would call the external library's function
        // For this example, we'll just use Rust's built-in function
        let converted_angle = self.convert_angle(angle);
        converted_angle.sin()
    }
    
    fn cos(&self, angle: f64) -> f64 {
        let converted_angle = self.convert_angle(angle);
        converted_angle.cos()
    }
    
    fn tan(&self, angle: f64) -> f64 {
        let converted_angle = self.convert_angle(angle);
        converted_angle.tan()
    }
    
    fn log(&self, value: f64, base: f64) -> Result<f64, String> {
        // Simulate calling an external library function
        if value <= 0.0 {
            return Err("Cannot take logarithm of non-positive number".to_string());
        }
        
        if base <= 0.0 || base == 1.0 {
            return Err("Invalid logarithm base".to_string());
        }
        
        Ok((value.ln()) / (base.ln()))
    }
}

// Adapters to connect different expression types

// Adapter for using ScientificOperations with Expression
pub struct ScientificFunctionExpression {
    operation: Box<dyn Fn(f64) -> f64>,
    arg_expression: Box<dyn Expression>,
    description: String,
}

impl ScientificFunctionExpression {
    pub fn new_sin(
        scientific_ops: Box<dyn ScientificOperations>,
        arg_expression: Box<dyn Expression>,
    ) -> Self {
        // We need to move the scientific_ops into the closure
        // This is a bit tricky in Rust without interior mutability
        let operation = Box::new(move |angle: f64| scientific_ops.sin(angle));
        
        Self {
            operation,
            arg_expression,
            description: "sin".to_string(),
        }
    }
    
    pub fn new_cos(
        scientific_ops: Box<dyn ScientificOperations>,
        arg_expression: Box<dyn Expression>,
    ) -> Self {
        let operation = Box::new(move |angle: f64| scientific_ops.cos(angle));
        
        Self {
            operation,
            arg_expression,
            description: "cos".to_string(),
        }
    }
    
    pub fn new_tan(
        scientific_ops: Box<dyn ScientificOperations>,
        arg_expression: Box<dyn Expression>,
    ) -> Self {
        let operation = Box::new(move |angle: f64| scientific_ops.tan(angle));
        
        Self {
            operation,
            arg_expression,
            description: "tan".to_string(),
        }
    }
}

impl Expression for ScientificFunctionExpression {
    fn evaluate(&self, variables: &HashMap<String, f64>) -> Result<f64, String> {
        let arg_value = self.arg_expression.evaluate(variables)?;
        Ok((self.operation)(arg_value))
    }
    
    fn to_string(&self) -> String {
        format!("{}({})", self.description, self.arg_expression.to_string())
    }
    
    fn precedence(&self) -> u8 {
        // Function calls have highest precedence
        4
    }
}

// Two-way adapter: allows Expression to be used where ScientificOperations is expected
pub struct ExpressionScientificAdapter {
    sin_expr: Box<dyn Expression>,
    cos_expr: Box<dyn Expression>,
    tan_expr: Box<dyn Expression>,
    log_expr: Box<dyn Expression>,
}

impl ExpressionScientificAdapter {
    pub fn new(
        sin_expr: Box<dyn Expression>,
        cos_expr: Box<dyn Expression>,
        tan_expr: Box<dyn Expression>,
        log_expr: Box<dyn Expression>,
    ) -> Self {
        Self {
            sin_expr,
            cos_expr,
            tan_expr,
            log_expr,
        }
    }
}

impl ScientificOperations for ExpressionScientificAdapter {
    fn sin(&self, angle: f64) -> f64 {
        // Create a variables map with the angle as a variable
        let mut variables = HashMap::new();
        variables.insert("x".to_string(), angle);
        
        // Evaluate the sin expression with this variable
        match self.sin_expr.evaluate(&variables) {
            Ok(result) => result,
            Err(_) => 0.0, // In a real implementation, we'd handle errors better
        }
    }
    
    fn cos(&self, angle: f64) -> f64 {
        let mut variables = HashMap::new();
        variables.insert("x".to_string(), angle);
        
        match self.cos_expr.evaluate(&variables) {
            Ok(result) => result,
            Err(_) => 0.0,
        }
    }
    
    fn tan(&self, angle: f64) -> f64 {
        let mut variables = HashMap::new();
        variables.insert("x".to_string(), angle);
        
        match self.tan_expr.evaluate(&variables) {
            Ok(result) => result,
            Err(_) => 0.0,
        }
    }
    
    fn log(&self, value: f64, base: f64) -> Result<f64, String> {
        let mut variables = HashMap::new();
        variables.insert("x".to_string(), value);
        variables.insert("base".to_string(), base);
        
        self.log_expr.evaluate(&variables)
    }
}
