// chain.rs - Chain of Responsibility pattern implementation

use crate::command::{Command, CommandProcessor, EvaluateCommand, SetVariableCommand, ClearVariablesCommand};
use crate::parser::ExpressionParser;

// Handler interface
pub trait InputHandler {
    fn handle(&self, input: &str, processor: &mut CommandProcessor) -> Result<Option<f64>, String>;
    fn set_next(&mut self, next: Box<dyn InputHandler>) -> &mut Self;
}

// Base implementation for chaining
pub struct BaseHandler {
    next: Option<Box<dyn InputHandler>>,
}

impl BaseHandler {
    pub fn new() -> Self {
        Self { next: None }
    }
}

impl InputHandler for BaseHandler {
    fn handle(&self, input: &str, processor: &mut CommandProcessor) -> Result<Option<f64>, String> {
        if let Some(next) = &self.next {
            next.handle(input, processor)
        } else {
            Err(format!("No handler found for input: {}", input))
        }
    }
    
    fn set_next(&mut self, next: Box<dyn InputHandler>) -> &mut Self {
        self.next = Some(next);
        self
    }
}

// Handles special commands like undo, redo, history
pub struct CommandHandler {
    base: BaseHandler,
}

impl CommandHandler {
    pub fn new() -> Self {
        Self { base: BaseHandler::new() }
    }
}

impl InputHandler for CommandHandler {
    fn handle(&self, input: &str, processor: &mut CommandProcessor) -> Result<Option<f64>, String> {
        let trimmed = input.trim();
        if trimmed.starts_with("/") {
            match &trimmed[1..] {
                "undo" => {
                    processor.undo()?;
                    Ok(None)
                },
                "redo" => {
                    processor.redo()?;
                    Ok(None)
                },
                "history" => {
                    for (i, cmd) in processor.history().iter().enumerate() {
                        println!("{}: {}", i+1, cmd);
                    }
                    Ok(None)
                },
                "clear" => {
                    let command = Box::new(ClearVariablesCommand::new());
                    processor.execute(command)
                },
                "help" => {
                    println!("Calculator commands:");
                    println!("  /undo - Undo last operation");
                    println!("  /redo - Redo last undone operation");
                    println!("  /history - Show command history");
                    println!("  /clear - Clear all variables");
                    println!("  /help - Show this help");
                    println!("  /exit - Exit the calculator");
                    Ok(None)
                },
                _ => self.base.handle(input, processor),
            }
        } else {
            self.base.handle(input, processor)
        }
    }
    
    fn set_next(&mut self, next: Box<dyn InputHandler>) -> &mut Self {
        self.base.set_next(next);
        self
    }
}

// Handles variable assignments (x=5)
pub struct VariableAssignmentHandler {
    base: BaseHandler,
    parser: ExpressionParser,
}

impl VariableAssignmentHandler {
    pub fn new(parser: ExpressionParser) -> Self {
        Self { 
            base: BaseHandler::new(),
            parser,
        }
    }
}

impl InputHandler for VariableAssignmentHandler {
    fn handle(&self, input: &str, processor: &mut CommandProcessor) -> Result<Option<f64>, String> {
        let trimmed = input.trim();
        if let Some((name, value_str)) = trimmed.split_once('=') {
            let name = name.trim();
            let value_str = value_str.trim();
            
            // Check if the name is valid
            if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                return Err(format!("Invalid variable name: {}", name));
            }
            
            // Try to evaluate the right side expression
            let expr = self.parser.parse(value_str)?;
            let calculator = processor.get_calculator();
            let value = expr.evaluate(&calculator.variables)?;
            
            // Set the variable
            let set_command = Box::new(SetVariableCommand::new(name.to_string(), value));
            processor.execute(set_command)?;
            
            Ok(Some(value))
        } else {
            self.base.handle(input, processor)
        }
    }
    
    fn set_next(&mut self, next: Box<dyn InputHandler>) -> &mut Self {
        self.base.set_next(next);
        self
    }
}

// Handles expressions (evaluates them)
pub struct ExpressionHandler {
    base: BaseHandler,
    parser: ExpressionParser,
}

impl ExpressionHandler {
    pub fn new(parser: ExpressionParser) -> Self {
        Self { 
            base: BaseHandler::new(),
            parser,
        }
    }
}

impl InputHandler for ExpressionHandler {
    fn handle(&self, input: &str, processor: &mut CommandProcessor) -> Result<Option<f64>, String> {
        let trimmed = input.trim();
        
        // Parse the expression
        let expr = self.parser.parse(trimmed)?;
        
        // Create an evaluation command
        let command = Box::new(EvaluateCommand::new(trimmed.to_string(), expr));
        
        // Execute the command
        processor.execute(command)
    }
    
    fn set_next(&mut self, next: Box<dyn InputHandler>) -> &mut Self {
        self.base.set_next(next);
        self
    }
}

// Function to create the chain of handlers
pub fn create_input_chain(parser: ExpressionParser) -> Box<dyn InputHandler> {
    let mut command_handler = CommandHandler::new();
    let mut var_handler = VariableAssignmentHandler::new(parser.clone());
    let expr_handler = ExpressionHandler::new(parser);
    
    var_handler.set_next(Box::new(expr_handler));
    command_handler.set_next(Box::new(var_handler));
    
    Box::new(command_handler)
}
