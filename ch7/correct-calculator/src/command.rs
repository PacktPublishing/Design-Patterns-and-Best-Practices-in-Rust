// command.rs - Command pattern implementation

use std::collections::HashMap;
use crate::expression::Expression;

// Command interface
pub trait Command {
    fn execute(&self, calculator: &mut Calculator) -> Result<Option<f64>, String>;
    fn undo(&self, calculator: &mut Calculator) -> Result<(), String>;
    fn description(&self) -> String;
}

// Calculator struct for command context
pub struct Calculator {
    pub variables: HashMap<String, f64>,
    pub history: Vec<Calculation>,
    pub last_result: Option<f64>,
}

// Represents a complete calculation
#[derive(Debug, Clone)]
pub struct Calculation {
    pub expression: String,
    pub result: f64,
    pub timestamp: std::time::SystemTime,
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

    pub fn clear_variable(&mut self, name: &str) {
        self.variables.remove(name);
    }

    pub fn set_last_result(&mut self, result: f64) {
        self.last_result = Some(result);
    }

    pub fn store_calculation(&mut self, expression: String, result: f64) {
        let calculation = Calculation {
            expression,
            result,
            timestamp: std::time::SystemTime::now(),
        };
        self.history.push(calculation);
        self.last_result = Some(result);
    }
}

// Concrete command for evaluating expressions
pub struct EvaluateCommand {
    expression: String,
    expr_tree: Box<dyn Expression>,
    previous_result: Option<f64>,
}

impl EvaluateCommand {
    pub fn new(expression: String, expr_tree: Box<dyn Expression>) -> Self {
        Self {
            expression,
            expr_tree,
            previous_result: None,
        }
    }
}

impl Command for EvaluateCommand {
    fn execute(&self, calculator: &mut Calculator) -> Result<Option<f64>, String> {
        self.previous_result = calculator.last_result;
        
        let result = self.expr_tree.evaluate(&calculator.variables)?;
        calculator.store_calculation(self.expression.clone(), result);
        
        Ok(Some(result))
    }
    
    fn undo(&self, calculator: &mut Calculator) -> Result<(), String> {
        // Remove the last entry from history
        if !calculator.history.is_empty() {
            calculator.history.pop();
        }
        
        // Restore previous result
        calculator.last_result = self.previous_result;
        
        Ok(())
    }
    
    fn description(&self) -> String {
        format!("Evaluate: {}", self.expression)
    }
}

// Command for setting variables
pub struct SetVariableCommand {
    name: String,
    value: f64,
    previous_value: Option<f64>,
}

impl SetVariableCommand {
    pub fn new(name: String, value: f64) -> Self {
        Self {
            name,
            value,
            previous_value: None,
        }
    }
}

impl Command for SetVariableCommand {
    fn execute(&self, calculator: &mut Calculator) -> Result<Option<f64>, String> {
        self.previous_value = calculator.get_variable(&self.name);
        calculator.set_variable(&self.name, self.value);
        Ok(None)
    }
    
    fn undo(&self, calculator: &mut Calculator) -> Result<(), String> {
        match self.previous_value {
            Some(value) => {
                calculator.set_variable(&self.name, value);
                Ok(())
            },
            None => {
                calculator.clear_variable(&self.name);
                Ok(())
            }
        }
    }
    
    fn description(&self) -> String {
        format!("Set: {} = {}", self.name, self.value)
    }
}

// Clear all variables command
pub struct ClearVariablesCommand {
    previous_variables: Option<HashMap<String, f64>>,
}

impl ClearVariablesCommand {
    pub fn new() -> Self {
        Self {
            previous_variables: None,
        }
    }
}

impl Command for ClearVariablesCommand {
    fn execute(&self, calculator: &mut Calculator) -> Result<Option<f64>, String> {
        self.previous_variables = Some(calculator.variables.clone());
        calculator.variables.clear();
        Ok(None)
    }
    
    fn undo(&self, calculator: &mut Calculator) -> Result<(), String> {
        if let Some(vars) = &self.previous_variables {
            calculator.variables = vars.clone();
            Ok(())
        } else {
            Err("No previous variables state saved".to_string())
        }
    }
    
    fn description(&self) -> String {
        "Clear all variables".to_string()
    }
}

// Command processor that handles and tracks commands
pub struct CommandProcessor {
    calculator: Calculator,
    history: Vec<Box<dyn Command>>,
    undo_stack: Vec<Box<dyn Command>>,
}

impl CommandProcessor {
    pub fn new() -> Self {
        Self {
            calculator: Calculator::new(),
            history: Vec::new(),
            undo_stack: Vec::new(),
        }
    }
    
    pub fn execute(&mut self, command: Box<dyn Command>) -> Result<Option<f64>, String> {
        let result = command.execute(&mut self.calculator)?;
        self.history.push(command);
        self.undo_stack.clear(); // Clear redo stack after new command
        Ok(result)
    }
    
    pub fn undo(&mut self) -> Result<(), String> {
        if let Some(command) = self.history.pop() {
            command.undo(&mut self.calculator)?;
            self.undo_stack.push(command);
            Ok(())
        } else {
            Err("Nothing to undo".to_string())
        }
    }
    
    pub fn redo(&mut self) -> Result<(), String> {
        if let Some(command) = self.undo_stack.pop() {
            command.execute(&mut self.calculator)?;
            self.history.push(command);
            Ok(())
        } else {
            Err("Nothing to redo".to_string())
        }
    }
    
    pub fn history(&self) -> Vec<String> {
        self.history
            .iter()
            .map(|cmd| cmd.description())
            .collect()
    }
    
    pub fn get_calculator(&self) -> &Calculator {
        &self.calculator
    }
    
    pub fn get_calculator_mut(&mut self) -> &mut Calculator {
        &mut self.calculator
    }
}
