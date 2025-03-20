// observer.rs - Observer pattern implementation for reactive updates

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::command::Calculation;
use crate::bridge::Display;

// Events that can be observed
#[derive(Clone, Debug)]
pub enum CalculatorEvent {
    VariableChanged(String, f64),
    ResultCalculated(f64, String), // Result and expression
    ModeChanged(String),
    HistoryAdded(Calculation),
    StateRestored,
    Error(String),
}

// Observer interface
pub trait Observer: Send + Sync {
    fn update(&self, event: &CalculatorEvent);
}

// Subject that maintains a list of observers
pub trait Subject {
    fn attach(&mut self, observer: Box<dyn Observer>) -> usize;
    fn detach(&mut self, observer_id: usize);
    fn notify(&self, event: &CalculatorEvent);
}

// Display observer that updates UI when calculator state changes
pub struct DisplayObserver {
    display: Arc<Mutex<dyn Display>>,
}

impl DisplayObserver {
    pub fn new(display: Arc<Mutex<dyn Display>>) -> Self {
        Self { display }
    }
}

impl Observer for DisplayObserver {
    fn update(&self, event: &CalculatorEvent) {
        let mut display = self.display.lock().unwrap();
        match event {
            CalculatorEvent::ResultCalculated(result, expr) => {
                display.show_result(*result);
                display.show_message(&format!("Evaluated: {}", expr));
            },
            CalculatorEvent::VariableChanged(name, value) => {
                display.show_message(&format!("Variable {} = {}", name, value));
            },
            CalculatorEvent::ModeChanged(mode) => {
                display.show_message(&format!("Switched to {} mode", mode));
            },
            CalculatorEvent::Error(message) => {
                display.show_error(message);
            },
            CalculatorEvent::StateRestored => {
                display.show_message("Calculator state restored");
            },
            CalculatorEvent::HistoryAdded(_) => {
                // Do nothing for history additions
            },
        }
    }
}

// Observer for dependent variables
pub struct DependentVariableObserver {
    calculator: Arc<Mutex<dyn VariableProvider>>,
    dependencies: HashMap<String, Vec<(String, String)>>, // Map of variable to tuples of dependent var name and expression
}

// Interface for calculator to provide variable evaluation
pub trait VariableProvider: Send + Sync {
    fn get_variable(&self, name: &str) -> Option<f64>;
    fn set_variable(&mut self, name: &str, value: f64);
    fn evaluate_expression(&mut self, expr: &str) -> Result<f64, String>;
}

impl DependentVariableObserver {
    pub fn new(calculator: Arc<Mutex<dyn VariableProvider>>) -> Self {
        Self {
            calculator,
            dependencies: HashMap::new(),
        }
    }
    
    pub fn add_dependency(&mut self, variable: &str, dependent: &str, expression: &str) {
        let dependencies = self.dependencies
            .entry(variable.to_string())
            .or_insert_with(Vec::new);
        
        dependencies.push((dependent.to_string(), expression.to_string()));
    }
    
    pub fn remove_dependency(&mut self, variable: &str, dependent: &str) {
        if let Some(dependencies) = self.dependencies.get_mut(variable) {
            dependencies.retain(|(dep, _)| dep != dependent);
            
            if dependencies.is_empty() {
                self.dependencies.remove(variable);
            }
        }
    }
}

impl Observer for DependentVariableObserver {
    fn update(&self, event: &CalculatorEvent) {
        if let CalculatorEvent::VariableChanged(name, _) = event {
            // Check if any variables depend on this one
            if let Some(dependents) = self.dependencies.get(name) {
                let mut calc = self.calculator.lock().unwrap();
                for (dependent, expr) in dependents {
                    // Re-evaluate the dependent variable
                    if let Ok(value) = calc.evaluate_expression(expr) {
                        calc.set_variable(dependent, value);
                    }
                }
            }
        } else if let CalculatorEvent::StateRestored = event {
            // Re-evaluate all dependent variables
            let mut calc = self.calculator.lock().unwrap();
            for (_, dependents) in &self.dependencies {
                for (dependent, expr) in dependents {
                    if let Ok(value) = calc.evaluate_expression(expr) {
                        calc.set_variable(dependent, value);
                    }
                }
            }
        }
    }
}

// Logger observer that logs all events
pub struct LoggerObserver;

impl Observer for LoggerObserver {
    fn update(&self, event: &CalculatorEvent) {
        match event {
            CalculatorEvent::VariableChanged(name, value) => {
                println!("[LOG] Variable changed: {} = {}", name, value);
            },
            CalculatorEvent::ResultCalculated(result, expr) => {
                println!("[LOG] Calculation: {} = {}", expr, result);
            },
            CalculatorEvent::ModeChanged(mode) => {
                println!("[LOG] Mode changed to: {}", mode);
            },
            CalculatorEvent::HistoryAdded(calc) => {
                println!("[LOG] History added: {} = {}", calc.expression, calc.result);
            },
            CalculatorEvent::StateRestored => {
                println!("[LOG] State restored");
            },
            CalculatorEvent::Error(message) => {
                println!("[LOG] Error: {}", message);
            },
        }
    }
}

// History observer that tracks calculation history
pub struct HistoryObserver {
    max_entries: usize,
    history: Arc<Mutex<Vec<Calculation>>>,
}

impl HistoryObserver {
    pub fn new(max_entries: usize) -> Self {
        Self {
            max_entries,
            history: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    pub fn get_history(&self) -> Arc<Mutex<Vec<Calculation>>> {
        Arc::clone(&self.history)
    }
}

impl Observer for HistoryObserver {
    fn update(&self, event: &CalculatorEvent) {
        if let CalculatorEvent::HistoryAdded(calc) = event {
            let mut history = self.history.lock().unwrap();
            history.push(calc.clone());
            
            // Trim if exceeds max entries
            if history.len() > self.max_entries {
                history.remove(0);
            }
        } else if let CalculatorEvent::StateRestored = event {
            // Clear history when state is restored
            let mut history = self.history.lock().unwrap();
            history.clear();
        }
    }
}

// Implementation of Subject for a calculator
pub struct ObservableCalculator {
    observers: HashMap<usize, Box<dyn Observer>>,
    next_observer_id: usize,
}

impl ObservableCalculator {
    pub fn new() -> Self {
        Self {
            observers: HashMap::new(),
            next_observer_id: 0,
        }
    }
}

impl Subject for ObservableCalculator {
    fn attach(&mut self, observer: Box<dyn Observer>) -> usize {
        let id = self.next_observer_id;
        self.observers.insert(id, observer);
        self.next_observer_id += 1;
        id
    }
    
    fn detach(&mut self, observer_id: usize) {
        self.observers.remove(&observer_id);
    }
    
    fn notify(&self, event: &CalculatorEvent) {
        for observer in self.observers.values() {
            observer.update(event);
        }
    }
}
