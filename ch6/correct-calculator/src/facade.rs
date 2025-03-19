// facade.rs - Facade pattern implementation

use std::collections::HashMap;
use crate::token::{Token, Operator, Function};
use crate::expression::{Expression, ExpressionParser, NumberExpression};
use crate::adapter::ScientificOperations;
use crate::config::CalculatorConfig;

// Facade for the calculator system that simplifies complex operations
pub struct CalculatorFacade {
    parser: ExpressionParser,
    variables: HashMap<String, f64>,
    scientific_ops: Box<dyn ScientificOperations>,
    history: Vec<String>,
    config: CalculatorConfig,
}

impl CalculatorFacade {
    pub fn new(scientific_ops: Box<dyn ScientificOperations>, config: CalculatorConfig) -> Self {
        Self {
            parser: ExpressionParser,
            variables: HashMap::new(),
            scientific_ops,
            history: Vec::new(),
            config,
        }
    }

    // Simple interface for evaluating expressions
    pub fn evaluate(&mut self, expression: &str) -> Result<f64, String> {
        self.history.push(expression.to_string());
        
        // Handle special function commands
        if let Some(result) = self.handle_special_command(expression)? {
            return Ok(result);
        }
        
        // Tokenize the expression
        let tokens = self.tokenize(expression)?;
        
        // Parse tokens into an expression tree
        let expr = ExpressionParser::parse(&tokens)?;
        
        // Evaluate the expression
        let result = expr.evaluate(&self.variables)?;
        
        // Store result in a special variable
        self.variables.insert("ans".to_string(), result);
        
        Ok(result)
    }
    
    // Simplified method to tokenize a string
    fn tokenize(&self, expression: &str) -> Result<Vec<Token>, String> {
        // This is a simple tokenizer for demonstration
        // In a real calculator, we would have a more sophisticated parser
        let mut tokens = Vec::new();
        
        for part in expression.split_whitespace() {
            tokens.push(Token::from_str(part)?);
        }
        
        Ok(tokens)
    }
    
    // Handle special commands like sin, cos, etc.
    fn handle_special_command(&mut self, command: &str) -> Result<Option<f64>, String> {
        // Parse commands of the form "sin x" or "log 10 2"
        let parts: Vec<&str> = command.split_whitespace().collect();
        
        if parts.len() < 2 {
            return Ok(None); // Not a special command
        }
        
        match parts[0] {
            "sin" => {
                let arg = self.parse_value(parts[1])?;
                Ok(Some(self.scientific_ops.sin(arg)))
            },
            "cos" => {
                let arg = self.parse_value(parts[1])?;
                Ok(Some(self.scientific_ops.cos(arg)))
            },
            "tan" => {
                let arg = self.parse_value(parts[1])?;
                Ok(Some(self.scientific_ops.tan(arg)))
            },
            "log" => {
                if parts.len() < 3 {
                    return Err("log requires two arguments: value and base".to_string());
                }
                let value = self.parse_value(parts[1])?;
                let base = self.parse_value(parts[2])?;
                self.scientific_ops.log(value, base)
                    .map(Some)
            },
            _ => Ok(None), // Not a special command
        }
    }
    
    // Helper to parse a value (number or variable)
    fn parse_value(&self, s: &str) -> Result<f64, String> {
        // Try to parse as a number
        if let Ok(num) = s.parse::<f64>() {
            return Ok(num);
        }
        
        // Try to look up as a variable
        if let Some(value) = self.variables.get(s) {
            return Ok(*value);
        }
        
        Err(format!("Unknown value: {}", s))
    }
    
    // Other simplified interfaces
    
    pub fn set_variable(&mut self, name: &str, value: f64) {
        self.variables.insert(name.to_string(), value);
    }
    
    pub fn get_variable(&self, name: &str) -> Option<f64> {
        self.variables.get(name).copied()
    }
    
    pub fn get_history(&self) -> &[String] {
        &self.history
    }
    
    // Specialized methods for common calculations
    
    pub fn calculate_quadratic(&mut self, a: f64, b: f64, c: f64) -> Result<(f64, f64), String> {
        let discriminant = b * b - 4.0 * a * c;
        
        if discriminant < 0.0 {
            return Err("No real solutions".to_string());
        }
        
        let sqrt_discriminant = discriminant.sqrt();
        let x1 = (-b + sqrt_discriminant) / (2.0 * a);
        let x2 = (-b - sqrt_discriminant) / (2.0 * a);
        
        Ok((x1, x2))
    }
    
    pub fn calculate_pythagorean(&self, a: f64, b: f64) -> f64 {
        (a * a + b * b).sqrt()
    }
    
    // Method to create expressions easily
    pub fn expression_from_string(&self, expr_str: &str) -> Result<Box<dyn Expression>, String> {
        let tokens = self.tokenize(expr_str)?;
        ExpressionParser::parse(&tokens)
    }
}
