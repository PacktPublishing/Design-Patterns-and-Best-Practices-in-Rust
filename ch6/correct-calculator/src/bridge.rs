// bridge.rs - Bridge pattern implementation

use crate::expression::Expression;

// Abstraction for a calculator display
pub trait Display {
    fn show_result(&self, result: f64);
    fn show_error(&self, error: &str);
    fn show_expression(&self, expression: &dyn Expression);
}

// Implementation for different display formats
pub trait DisplayImplementation {
    fn display_text(&self, text: &str);
    fn display_formatted(&self, value: f64, format: &str);
}

// Concrete display that uses a specific implementation
pub struct CalculatorDisplay {
    implementation: Box<dyn DisplayImplementation>,
}

impl CalculatorDisplay {
    pub fn new(implementation: Box<dyn DisplayImplementation>) -> Self {
        Self { implementation }
    }
}

impl Display for CalculatorDisplay {
    fn show_result(&self, result: f64) {
        self.implementation.display_formatted(result, "Result: {:.10g}");
    }
    
    fn show_error(&self, error: &str) {
        self.implementation.display_text(&format!("Error: {}", error));
    }
    
    fn show_expression(&self, expression: &dyn Expression) {
        self.implementation.display_text(&format!("Expression: {}", expression.to_string()));
    }
}

// Different implementations for the display
pub struct ConsoleDisplay;

impl DisplayImplementation for ConsoleDisplay {
    fn display_text(&self, text: &str) {
        println!("{}", text);
    }
    
    fn display_formatted(&self, value: f64, format: &str) {
        println!("{}", format.replace("{:.10g}", &format!("{:.10}", value)));
    }
}

pub struct HtmlDisplay;

impl DisplayImplementation for HtmlDisplay {
    fn display_text(&self, text: &str) {
        println!("<div>{}</div>", text.replace("<", "&lt;").replace(">", "&gt;"));
    }
    
    fn display_formatted(&self, value: f64, format: &str) {
        let formatted = format.replace("{:.10g}", &format!("{:.10}", value));
        println!("<div class=\"result\">{}</div>", formatted);
    }
}

pub struct JsonDisplay;

impl DisplayImplementation for JsonDisplay {
    fn display_text(&self, text: &str) {
        println!("{{\"text\": \"{}\"}}", text.replace("\"", "\\\""));
    }
    
    fn display_formatted(&self, value: f64, format: &str) {
        let formatted = format!("{:.10}", value);
        println!("{{\"result\": {}}}", formatted);
    }
}

// More complex bridge example for expression evaluation

// Abstract interface for evaluation strategies
pub trait EvaluationStrategy {
    fn evaluate(&self, expression: &dyn Expression, variables: &std::collections::HashMap<String, f64>) -> Result<f64, String>;
}

// Different evaluation strategies (implementors)
pub struct StandardEvaluator;

impl EvaluationStrategy for StandardEvaluator {
    fn evaluate(&self, expression: &dyn Expression, variables: &std::collections::HashMap<String, f64>) -> Result<f64, String> {
        // Basic evaluation without optimizations
        expression.evaluate(variables)
    }
}

pub struct OptimizingEvaluator {
    cache: std::collections::HashMap<String, f64>,
}

impl OptimizingEvaluator {
    pub fn new() -> Self {
        Self {
            cache: std::collections::HashMap::new(),
        }
    }
}

impl EvaluationStrategy for OptimizingEvaluator {
    fn evaluate(&self, expression: &dyn Expression, variables: &std::collections::HashMap<String, f64>) -> Result<f64, String> {
        // Check if we've evaluated this expression before
        let key = format!("{:?}:{}", expression.to_string(), variables.len());
        
        // In a real implementation, we'd properly account for variable values in the key
        // For demonstration, this is simplified
        if let Some(cached_result) = self.cache.get(&key) {
            return Ok(*cached_result);
        }
        
        // Evaluate and cache the result
        let result = expression.evaluate(variables)?;
        
        // In a real implementation, we'd use interior mutability for thread safety
        let mut_self = unsafe { &mut *(self as *const Self as *mut Self) };
        mut_self.cache.insert(key, result);
        
        Ok(result)
    }
}

// Abstraction for evaluation
pub struct Evaluator {
    strategy: Box<dyn EvaluationStrategy>,
}

impl Evaluator {
    pub fn new(strategy: Box<dyn EvaluationStrategy>) -> Self {
        Self { strategy }
    }
    
    pub fn evaluate(&self, expression: &dyn Expression, variables: &std::collections::HashMap<String, f64>) -> Result<f64, String> {
        self.strategy.evaluate(expression, variables)
    }
    
    pub fn change_strategy(&mut self, strategy: Box<dyn EvaluationStrategy>) {
        self.strategy = strategy;
    }
}
