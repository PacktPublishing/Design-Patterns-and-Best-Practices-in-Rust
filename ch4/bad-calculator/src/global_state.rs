// Ch4: Global State Anti-pattern
//
// This file demonstrates the anti-pattern of using global state
// in a calculator implementation, followed by a better approach.

use std::sync::Mutex;
use std::collections::HashMap;
use lazy_static::lazy_static;

// BAD APPROACH: Using global state

// Global variables for the calculator state
lazy_static! {
    static ref VARIABLES: Mutex<HashMap<String, f64>> = Mutex::new(HashMap::new());
    static ref HISTORY: Mutex<Vec<String>> = Mutex::new(Vec::new());
    static ref LAST_RESULT: Mutex<Option<f64>> = Mutex::new(None);
}

// Functions that access global state
pub fn set_variable(name: &str, value: f64) {
    let mut vars = VARIABLES.lock().unwrap();
    vars.insert(name.to_string(), value);
}

pub fn get_variable(name: &str) -> Option<f64> {
    let vars = VARIABLES.lock().unwrap();
    vars.get(name).copied()
}

pub fn evaluate(expression: &str) -> Result<f64, String> {
    // Simple implementation for demonstration
    let parts: Vec<&str> = expression.split_whitespace().collect();
    
    if parts.len() != 3 {
        return Err("Invalid expression format".to_string());
    }
    
    let left = match parts[0].parse::<f64>() {
        Ok(n) => n,
        Err(_) => match get_variable(parts[0]) {
            Some(v) => v,
            None => return Err(format!("Unknown variable: {}", parts[0])),
        },
    };
    
    let right = match parts[2].parse::<f64>() {
        Ok(n) => n,
        Err(_) => match get_variable(parts[2]) {
            Some(v) => v,
            None => return Err(format!("Unknown variable: {}", parts[2])),
        },
    };
    
    let result = match parts[1] {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => {
            if right == 0.0 {
                return Err("Division by zero".to_string());
            }
            left / right
        },
        _ => return Err(format!("Unknown operator: {}", parts[1])),
    };
    
    // Update global state
    let mut history = HISTORY.lock().unwrap();
    history.push(expression.to_string());
    
    let mut last_result = LAST_RESULT.lock().unwrap();
    *last_result = Some(result);
    
    Ok(result)
}

pub fn get_history() -> Vec<String> {
    let history = HISTORY.lock().unwrap();
    history.clone()
}

pub fn clear_history() {
    let mut history = HISTORY.lock().unwrap();
    history.clear();
}

pub fn get_last_result() -> Option<f64> {
    let last_result = LAST_RESULT.lock().unwrap();
    *last_result
}

// BETTER APPROACH: Explicit state management

pub struct Calculator {
    variables: HashMap<String, f64>,
    history: Vec<String>,
    last_result: Option<f64>,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            history: Vec::new(),
            last_result: None,
        }
    }
    
    pub fn set_variable(&mut self, name: &str, value: f64) {
        self.variables.insert(name.to_string(), value);
    }
    
    pub fn get_variable(&self, name: &str) -> Option<f64> {
        self.variables.get(name).copied()
    }
    
    pub fn evaluate(&mut self, expression: &str) -> Result<f64, String> {
        // Similar implementation as the global version
        let parts: Vec<&str> = expression.split_whitespace().collect();
        
        if parts.len() != 3 {
            return Err("Invalid expression format".to_string());
        }
        
        let left = match parts[0].parse::<f64>() {
            Ok(n) => n,
            Err(_) => match self.get_variable(parts[0]) {
                Some(v) => v,
                None => return Err(format!("Unknown variable: {}", parts[0])),
            },
        };
        
        let right = match parts[2].parse::<f64>() {
            Ok(n) => n,
            Err(_) => match self.get_variable(parts[2]) {
                Some(v) => v,
                None => return Err(format!("Unknown variable: {}", parts[2])),
            },
        };
        
        let result = match parts[1] {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => {
                if right == 0.0 {
                    return Err("Division by zero".to_string());
                }
                left / right
            },
            _ => return Err(format!("Unknown operator: {}", parts[1])),
        };
        
        // Update instance state
        self.history.push(expression.to_string());
        self.last_result = Some(result);
        
        Ok(result)
    }
    
    pub fn get_history(&self) -> &[String] {
        &self.history
    }
    
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
    
    pub fn get_last_result(&self) -> Option<f64> {
        self.last_result
    }
}

// Thread-safe calculator that encapsulates its state
pub struct ThreadSafeCalculator {
    inner: Mutex<Calculator>,
}

impl ThreadSafeCalculator {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(Calculator::new()),
        }
    }
    
    pub fn set_variable(&self, name: &str, value: f64) -> Result<(), String> {
        let mut calc = self.inner.lock().map_err(|_| "Lock poisoned")?;
        calc.set_variable(name, value);
        Ok(())
    }
    
    pub fn evaluate(&self, expression: &str) -> Result<f64, String> {
        let mut calc = self.inner.lock().map_err(|_| "Lock poisoned")?;
        calc.evaluate(expression)
    }
    
    pub fn get_history(&self) -> Result<Vec<String>, String> {
        let calc = self.inner.lock().map_err(|_| "Lock poisoned")?;
        Ok(calc.get_history().to_vec())
    }
}

fn main() {
    println!("--- BAD APPROACH: Using Global State ---");
    
    set_variable("x", 10.0);
    set_variable("y", 5.0);
    
    match evaluate("x + y") {
        Ok(result) => println!("x + y = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match evaluate("x * 2") {
        Ok(result) => println!("x * 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    println!("History: {:?}", get_history());
    println!("Last result: {:?}", get_last_result());
    
    // Issues with global state:
    // - Hard to test
    // - Thread safety concerns
    // - Difficult to reason about program behavior
    // - Tight coupling between components
    
    println!("\n--- BETTER APPROACH: Explicit State Management ---");
    
    let mut calc = Calculator::new();
    calc.set_variable("x", 10.0);
    calc.set_variable("y", 5.0);
    
    match calc.evaluate("x + y") {
        Ok(result) => println!("x + y = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match calc.evaluate("x * 2") {
        Ok(result) => println!("x * 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    println!("History: {:?}", calc.get_history());
    println!("Last result: {:?}", calc.get_last_result());
    
    println!("\n--- THREAD-SAFE APPROACH ---");
    
    let safe_calc = ThreadSafeCalculator::new();
    safe_calc.set_variable("a", 7.0).unwrap();
    safe_calc.set_variable("b", 3.0).unwrap();
    
    match safe_calc.evaluate("a - b") {
        Ok(result) => println!("a - b = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    println!("History: {:?}", safe_calc.get_history().unwrap());
}