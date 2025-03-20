// bridge.rs - Bridge pattern implementation

use std::collections::HashMap;
use crate::expression::Expression;

// Display interface (abstraction)
pub trait Display {
    fn show_result(&self, result: f64);
    fn show_error(&self, error: &str);
    fn show_expression(&self, expression: &dyn Expression);
    fn show_message(&self, message: &str);
}

// Concrete display implementation
pub struct ConsoleDisplay;

impl Display for ConsoleDisplay {
    fn show_result(&self, result: f64) {
        println!("Result: {}", result);
    }
    
    fn show_error(&self, error: &str) {
        println!("Error: {}", error);
    }
    
    fn show_expression(&self, expression: &dyn Expression) {
        println!("Expression: {}", expression.to_string());
    }
    
    fn show_message(&self, message: &str) {
        println!("{}", message);
    }
}

// Evaluator (abstraction)
pub struct Evaluator {
    strategy: Box<dyn EvaluationStrategy>,
}

// Implementor for evaluation
pub trait EvaluationStrategy {
    fn evaluate(&self, expression: &dyn Expression, variables: &HashMap<String, f64>) -> Result<f64, String>;
}

// Concrete implementor
pub struct StandardEvaluator;

impl EvaluationStrategy for StandardEvaluator {
    fn evaluate(&self, expression: &dyn Expression, variables: &HashMap<String, f64>) -> Result<f64, String> {
        expression.evaluate(variables)
    }
}

impl Evaluator {
    pub fn new(strategy: Box<dyn EvaluationStrategy>) -> Self {
        Self { strategy }
    }
    
    pub fn evaluate(&self, expression: &dyn Expression, variables: &HashMap<String, f64>) -> Result<f64, String> {
        self.strategy.evaluate(expression, variables)
    }
    
    pub fn change_strategy(&mut self, strategy: Box<dyn EvaluationStrategy>) {
        self.strategy = strategy;
    }
}
