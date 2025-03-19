// Ch3: Avoiding Ownership Concerns

// This file demonstrates the anti-pattern of avoiding ownership concerns
// in a calculator implementation, followed by a better approach.

use std::cell::RefCell;

// COMMON STRUCTURES

struct CalculationResult {
    expression: String,
    result: f64,
}

// BAD APPROACH: Using traits that don't respect ownership

// A trait for components that need to access the history
trait HistoryViewer {
    fn view_history(&self) -> &[CalculationResult];
    fn get_last_result(&self) -> Option<f64>;
}

// A trait for components that need to modify the history
trait HistoryManager {
    fn add_to_history(&self, expression: String, result: f64);
    fn clear_history(&self);
}

// BAD IMPLEMENTATION: Trying to ignore ownership

struct BadCalculator {
    history: RefCell<Vec<CalculationResult>>,
    current_expression: RefCell<Option<String>>,
}

impl BadCalculator {
    fn new() -> Self {
        Self {
            history: RefCell::new(Vec::new()),
            current_expression: RefCell::new(None),
        }
    }

    fn evaluate(&self, expression: &str) -> Result<f64, String> {
        // Simple evaluation for demonstration
        let result = match expression.trim() {
            "1+1" => 2.0,
            "2+2" => 4.0,
            _ => return Err("Unknown expression".to_string()),
        };

        self.add_to_history(expression.to_string(), result);
        Ok(result)
    }
}

impl HistoryViewer for BadCalculator {
    fn view_history(&self) -> &[CalculationResult] {
        // This is problematic - borrowing from RefCell but returning a reference
        // In a complete implementation, this would be more complex
        &self.history.borrow()
    }

    fn get_last_result(&self) -> Option<f64> {
        self.history.borrow().last().map(|r| r.result)
    }
}

impl HistoryManager for BadCalculator {
    // Using interior mutability to hack around ownership
    fn add_to_history(&self, expression: String, result: f64) {
        self.history.borrow_mut().push(CalculationResult {
            expression,
            result,
        });
    }

    fn clear_history(&self) {
        self.history.borrow_mut().clear();
    }
}

// BETTER APPROACH: Respect ownership

struct Calculator {
    history: Vec<CalculationResult>,
    current_expression: Option<String>,
}

impl Calculator {
    fn new() -> Self {
        Self {
            history: Vec::new(),
            current_expression: None,
        }
    }

    // Methods that only need to read use &self
    fn view_history(&self) -> &[CalculationResult] {
        &self.history
    }

    fn get_last_result(&self) -> Option<f64> {
        self.history.last().map(|r| r.result)
    }

    // Methods that need to modify use &mut self
    fn add_to_history(&mut self, expression: String, result: f64) {
        self.history.push(CalculationResult {
            expression,
            result,
        });
    }

    fn evaluate(&mut self, expression: &str) -> Result<f64, String> {
        // Simple evaluation for demonstration
        let result = match expression.trim() {
            "1+1" => 2.0,
            "2+2" => 4.0,
            _ => return Err("Unknown expression".to_string()),
        };

        self.add_to_history(expression.to_string(), result);
        Ok(result)
    }
}

// For sharing read-only access to history
struct HistoryView<'a> {
    entries: &'a [CalculationResult],
}

impl Calculator {
    fn create_history_view(&self) -> HistoryView<'_> {
        HistoryView {
            entries: &self.history,
        }
    }
}

fn main() {
    println!("--- BAD APPROACH: Using RefCell to Avoid Ownership Concerns ---");
    let bad_calc = BadCalculator::new();
    
    match bad_calc.evaluate("1+1") {
        Ok(result) => println!("1+1 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match bad_calc.evaluate("2+2") {
        Ok(result) => println!("2+2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // This works but has runtime cost and potential panics
    println!("Last result: {:?}", bad_calc.get_last_result());
    
    println!("\n--- BETTER APPROACH: Respecting Ownership ---");
    let mut calc = Calculator::new();
    
    match calc.evaluate("1+1") {
        Ok(result) => println!("1+1 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match calc.evaluate("2+2") {
        Ok(result) => println!("2+2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // This is clearer, safer, and more efficient
    println!("Last result: {:?}", calc.get_last_result());
    
    // Create a read-only view for sharing
    let history_view = calc.create_history_view();
    println!("History entries: {}", history_view.entries.len());
}
