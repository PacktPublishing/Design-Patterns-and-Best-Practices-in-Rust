// mediator.rs - Mediator pattern implementation

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::config::AngleMode;

// Mediator interface
pub trait CalculatorMediator: Send + Sync {
    fn notify(&mut self, sender: &str, event: CalculatorEvent);
    fn get_result(&self) -> Option<f64>;
    fn get_variable(&self, name: &str) -> Option<f64>;
    fn get_all_variables(&self) -> HashMap<String, f64>;
    fn set_variable(&mut self, name: &str, value: f64);
    fn evaluate(&mut self, expression: &str) -> Result<f64, String>;
    fn change_angle_mode(&mut self, mode: AngleMode);
}

// Events that can be sent through the mediator
pub enum CalculatorEvent {
    ResultComputed(f64),
    VariableChanged(String, f64),
    ModeChanged(String),
    DisplayUpdate(String),
    ErrorOccurred(String),
}

// Display component interface
pub trait Display: Send + Sync {
    fn show_result(&mut self, result: f64);
    fn show_message(&mut self, message: &str);
    fn show_error(&mut self, error: &str);
    fn clear(&mut self);
}

// Component that handles evaluation
pub struct EvaluationComponent {
    mediator: Arc<Mutex<dyn CalculatorMediator>>,
    parser: crate::parser::ExpressionParser,
}

impl EvaluationComponent {
    pub fn new(mediator: Arc<Mutex<dyn CalculatorMediator>>) -> Self {
        Self {
            mediator,
            parser: crate::parser::ExpressionParser::new(),
        }
    }
    
    pub fn evaluate(&self, expression: &str) -> Result<f64, String> {
        // Parse expression
        let expr = self.parser.parse(expression)?;
        
        // Get variables from mediator
        let variables = {
            let mediator = self.mediator.lock().unwrap();
            mediator.get_all_variables()
        };
        
        // Evaluate
        let result = expr.evaluate(&variables)?;
        
        // Notify mediator of result
        {
            let mut mediator = self.mediator.lock().unwrap();
            mediator.notify("evaluator", CalculatorEvent::ResultComputed(result));
        }
        
        Ok(result)
    }
}

// Component that manages variables
pub struct VariableStorage {
    mediator: Arc<Mutex<dyn CalculatorMediator>>,
    variables: HashMap<String, f64>,
}

impl VariableStorage {
    pub fn new(mediator: Arc<Mutex<dyn CalculatorMediator>>) -> Self {
        Self {
            mediator,
            variables: HashMap::new(),
        }
    }
    
    pub fn set_variable(&mut self, name: &str, value: f64) {
        self.variables.insert(name.to_string(), value);
        
        // Notify mediator
        let mut mediator = self.mediator.lock().unwrap();
        mediator.notify("variables", CalculatorEvent::VariableChanged(name.to_string(), value));
    }
    
    pub fn get_variable(&self, name: &str) -> Option<f64> {
        self.variables.get(name).copied()
    }
    
    pub fn get_all_variables(&self) -> HashMap<String, f64> {
        self.variables.clone()
    }
    
    pub fn clear(&mut self) {
        self.variables.clear();
    }
}

// Console display component
pub struct ConsoleDisplay {
    mediator: Arc<Mutex<dyn CalculatorMediator>>,
}

impl ConsoleDisplay {
    pub fn new(mediator: Arc<Mutex<dyn CalculatorMediator>>) -> Self {
        Self { mediator }
    }
}

impl Display for ConsoleDisplay {
    fn show_result(&mut self, result: f64) {
        println!("Result: {}", result);
    }
    
    fn show_message(&mut self, message: &str) {
        println!("{}", message);
    }
    
    fn show_error(&mut self, error: &str) {
        println!("Error: {}", error);
    }
    
    fn clear(&mut self) {
        // Clear console (platform-specific)
        // For simplicity, just print some newlines
        println!("\n\n\n\n\n");
    }
}

// Concrete mediator implementation
pub struct CalculatorMediatorImpl {
    evaluator: Option<Arc<EvaluationComponent>>,
    variables: Option<Arc<Mutex<VariableStorage>>>,
    display: Option<Arc<Mutex<dyn Display>>>,
    last_result: Option<f64>,
    angle_mode: AngleMode,
}

impl CalculatorMediatorImpl {
    pub fn new() -> Self {
        Self {
            evaluator: None,
            variables: None,
            display: None,
            last_result: None,
            angle_mode: AngleMode::Radians,
        }
    }
    
    pub fn set_evaluator(&mut self, evaluator: Arc<EvaluationComponent>) {
        self.evaluator = Some(evaluator);
    }
    
    pub fn set_variables(&mut self, variables: Arc<Mutex<VariableStorage>>) {
        self.variables = Some(variables);
    }
    
    pub fn set_display(&mut self, display: Arc<Mutex<dyn Display>>) {
        self.display = Some(display);
    }
}

impl CalculatorMediator for CalculatorMediatorImpl {
    fn notify(&mut self, sender: &str, event: CalculatorEvent) {
        match event {
            CalculatorEvent::ResultComputed(result) => {
                self.last_result = Some(result);
                
                if let Some(display) = &self.display {
                    let mut display = display.lock().unwrap();
                    display.show_result(result);
                }
            },
            CalculatorEvent::VariableChanged(name, value) => {
                if let Some(display) = &self.display {
                    let mut display = display.lock().unwrap();
                    display.show_message(&format!("Variable {} set to {}", name, value));
                }
            },
            CalculatorEvent::ModeChanged(mode) => {
                if let Some(display) = &self.display {
                    let mut display = display.lock().unwrap();
                    display.show_message(&format!("Mode changed to {}", mode));
                }
            },
            CalculatorEvent::DisplayUpdate(message) => {
                if let Some(display) = &self.display {
                    let mut display = display.lock().unwrap();
                    display.show_message(&message);
                }
            },
            CalculatorEvent::ErrorOccurred(error) => {
                if let Some(display) = &self.display {
                    let mut display = display.lock().unwrap();
                    display.show_error(&error);
                }
            },
        }
    }
    
    fn get_result(&self) -> Option<f64> {
        self.last_result
    }
    
    fn get_variable(&self, name: &str) -> Option<f64> {
        if let Some(variables) = &self.variables {
            let variables = variables.lock().unwrap();
            variables.get_variable(name)
        } else {
            None
        }
    }
    
    fn get_all_variables(&self) -> HashMap<String, f64> {
        if let Some(variables) = &self.variables {
            let variables = variables.lock().unwrap();
            variables.get_all_variables()
        } else {
            HashMap::new()
        }
    }
    
    fn set_variable(&mut self, name: &str, value: f64) {
        if let Some(variables) = &self.variables {
            let mut variables = variables.lock().unwrap();
            variables.set_variable(name, value);
        }
    }
    
    fn evaluate(&mut self, expression: &str) -> Result<f64, String> {
        if let Some(evaluator) = &self.evaluator {
            evaluator.evaluate(expression)
        } else {
            Err("Evaluator not initialized".to_string())
        }
    }
    
    fn change_angle_mode(&mut self, mode: AngleMode) {
        self.angle_mode = mode;
        
        let mode_str = match mode {
            AngleMode::Degrees => "Degrees",
            AngleMode::Radians => "Radians",
        };
        
        self.notify("mediator", CalculatorEvent::ModeChanged(mode_str.to_string()));
    }
}

// Helper function to set up mediator system
pub fn create_mediator_system() -> Arc<Mutex<dyn CalculatorMediator>> {
    // Create mediator
    let mediator = Arc::new(Mutex::new(CalculatorMediatorImpl::new()) as Mutex<dyn CalculatorMediator>);
    
    // Create components
    let evaluator = Arc::new(EvaluationComponent::new(mediator.clone()));
    let variables = Arc::new(Mutex::new(VariableStorage::new(mediator.clone())));
    let display = Arc::new(Mutex::new(ConsoleDisplay::new(mediator.clone())) as Mutex<dyn Display>);
    
    // Register components with mediator
    {
        let mut mediator_lock = mediator.lock().unwrap();
        if let Some(mediator_impl) = mediator_lock.downcast_mut::<CalculatorMediatorImpl>() {
            mediator_impl.set_evaluator(evaluator);
            mediator_impl.set_variables(variables);
            mediator_impl.set_display(display);
        }
    }
    
    mediator
}
