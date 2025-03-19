// decorator.rs - Decorator pattern implementation

use std::collections::HashMap;
use std::time::Instant;
use crate::expression::Expression;

// Logger trait for logging operations
pub trait Logger {
    fn log(&self, message: &str);
}

// Console logger implementation
pub struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("[LOG] {}", message);
    }
}

// A decorator for expressions that logs evaluation
pub struct LoggingExpression {
    inner: Box<dyn Expression>,
    logger: Box<dyn Logger>,
}

impl LoggingExpression {
    pub fn new(inner: Box<dyn Expression>, logger: Box<dyn Logger>) -> Self {
        Self { inner, logger }
    }
}

impl Expression for LoggingExpression {
    fn evaluate(&self, variables: &HashMap<String, f64>) -> Result<f64, String> {
        self.logger.log(&format!("Evaluating: {}", self.inner.to_string()));
        let result = self.inner.evaluate(variables);
        match &result {
            Ok(value) => self.logger.log(&format!("Result: {}", value)),
            Err(err) => self.logger.log(&format!("Error: {}", err)),
        }
        result
    }
    
    fn to_string(&self) -> String {
        self.inner.to_string()
    }
    
    fn precedence(&self) -> u8 {
        self.inner.precedence()
    }
}

// A decorator that times evaluation
pub struct TimingExpression {
    inner: Box<dyn Expression>,
}

impl TimingExpression {
    pub fn new(inner: Box<dyn Expression>) -> Self {
        Self { inner }
    }
}

impl Expression for TimingExpression {
    fn evaluate(&self, variables: &HashMap<String, f64>) -> Result<f64, String> {
        let start = Instant::now();
        let result = self.inner.evaluate(variables);
        let duration = start.elapsed();
        println!("Evaluation took: {:?}", duration);
        result
    }
    
    fn to_string(&self) -> String {
        self.inner.to_string()
    }
    
    fn precedence(&self) -> u8 {
        self.inner.precedence()
    }
}

// A decorator that caches evaluation results
pub struct CachingExpression {
    inner: Box<dyn Expression>,
    // In a real implementation, we might use a more sophisticated caching strategy
    // and handle variable-dependent caching properly
    last_result: Option<f64>,
}

impl CachingExpression {
    pub fn new(inner: Box<dyn Expression>) -> Self {
        Self { 
            inner,
            last_result: None,
        }
    }
    
    pub fn invalidate_cache(&mut self) {
        self.last_result = None;
    }
}

impl Expression for CachingExpression {
    fn evaluate(&self, variables: &HashMap<String, f64>) -> Result<f64, String> {
        // In a real implementation, we would need to check if variables have changed
        // For this example, we're keeping it simple
        if let Some(result) = self.last_result {
            return Ok(result);
        }
        
        let result = self.inner.evaluate(variables)?;
        // In a real implementation, we'd use interior mutability for thread safety
        // But for demonstration, we're using Option directly
        let mut_self = unsafe { &mut *(self as *const Self as *mut Self) };
        mut_self.last_result = Some(result);
        
        Ok(result)
    }
    
    fn to_string(&self) -> String {
        self.inner.to_string()
    }
    
    fn precedence(&self) -> u8 {
        self.inner.precedence()
    }
}

// A decorator that validates the result range
pub struct RangeValidatingExpression {
    inner: Box<dyn Expression>,
    min: f64,
    max: f64,
}

impl RangeValidatingExpression {
    pub fn new(inner: Box<dyn Expression>, min: f64, max: f64) -> Self {
        Self { inner, min, max }
    }
}

impl Expression for RangeValidatingExpression {
    fn evaluate(&self, variables: &HashMap<String, f64>) -> Result<f64, String> {
        let result = self.inner.evaluate(variables)?;
        
        if result < self.min {
            Err(format!("Result {} is less than minimum {}", result, self.min))
        } else if result > self.max {
            Err(format!("Result {} is greater than maximum {}", result, self.max))
        } else {
            Ok(result)
        }
    }
    
    fn to_string(&self) -> String {
        format!("validate({}, min={}, max={})", 
                self.inner.to_string(), self.min, self.max)
    }
    
    fn precedence(&self) -> u8 {
        self.inner.precedence()
    }
}
