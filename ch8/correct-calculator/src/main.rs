// main.rs - Main entry point for the calculator
// Incorporates all design patterns from Chapters 5-8

// Chapter 5-7 modules
mod token;
mod expression;
mod config;
mod command;
mod chain;
mod strategy;
mod parser;
mod mediator;
mod template;
mod bridge;
mod adapter;

// Chapter 8 modules
mod iterator;
mod state;
mod memento;
mod observer;
mod visitor;

use std::io::{self, Write};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use command::{CommandProcessor, EvaluateCommand, SetVariableCommand, Calculator, ClearVariablesCommand};
use chain::create_input_chain;
use parser::ExpressionParser;
use iterator::HistoryIterator;
use state::{StateCalculator, CalculatorState, StandardMode};
use memento::{CalculatorStateManager, MementoOriginator, CalculatorMemento, SaveStateCommand, RestoreStateCommand, CalculatorStateType, get_calculator_state_type, get_angle_mode, get_number_base};
use observer::{Subject, Observer, ObservableCalculator, DisplayObserver, DependentVariableObserver, LoggerObserver, CalculatorEvent, VariableProvider};
use visitor::{optimize_expression, validate_expression};
use bridge::{Display, ConsoleDisplay};

// Complete calculator that combines all patterns
struct CorrectCalculator {
    // Chapter 7 patterns
    command_processor: CommandProcessor,
    input_chain: Box<dyn chain::InputHandler>,
    
    // Chapter 8 patterns
    state: Box<dyn CalculatorState>,
    state_manager: CalculatorStateManager,
    observable: ObservableCalculator,
    
    // Core data
    variables: HashMap<String, f64>,
    parser: ExpressionParser,
    next_observer_id: usize,
}

impl CorrectCalculator {
    fn new() -> Self {
        let parser = ExpressionParser::new();
        let command_processor = CommandProcessor::new();
        let input_chain = create_input_chain(parser.clone());
        
        let mut calculator = Self {
            command_processor,
            input_chain,
            state: Box::new(StandardMode::new()),
            state_manager: CalculatorStateManager::new(),
            observable: ObservableCalculator::new(),
            variables: HashMap::new(),
            parser,
            next_observer_id: 0,
        };
        
        // Add standard observers
        let display = Arc::new(Mutex::new(ConsoleDisplay));
        calculator.attach_observer(Box::new(DisplayObserver::new(display)));
        calculator.attach_observer(Box::new(LoggerObserver));
        
        calculator
    }
    
    fn attach_observer(&mut self, observer: Box<dyn Observer>) -> usize {
        self.observable.attach(observer)
    }
    
    fn detach_observer(&mut self, observer_id: usize) {
        self.observable.detach(observer_id);
    }
    
    fn notify(&self, event: &CalculatorEvent) {
        self.observable.notify(event);
    }
    
    fn process_input(&mut self, input: &str) -> Result<Option<f64>, String> {
        if input.starts_with("/") {
            self.process_command(&input[1..])
        } else if let Some((name, value_str)) = input.split_once('=') {
            // Variable assignment
            let name = name.trim();
            let value_str = value_str.trim();
            
            // Parse and evaluate the expression
            let expr = self.parser.parse(value_str)?;
            let value = expr.evaluate(&self.variables)?;
            
            // Set the variable
            self.set_variable(name, value);
            
            Ok(Some(value))
        } else {
            // Expression evaluation
            let expr = self.parser.parse(input)?;
            
            // Optimize and validate the expression
            let optimized = optimize_expression(&*expr, &self.variables)?;
            validate_expression(&*optimized)?;
            
            // Evaluate the optimized expression
            let result = optimized.evaluate(&self.variables)?;
            
            // Store the result
            self.command_processor.get_calculator_mut().store_calculation(input.to_string(), result);
            
            // Notify observers
            self.notify(&CalculatorEvent::ResultCalculated(result, input.to_string()));
            
            Ok(Some(result))
        }
    }
    
    fn process_command(&mut self, command: &str) -> Result<Option<f64>, String> {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return Err("Empty command".to_string());
        }
        
        match parts[0] {
            "help" => {
                println!("Available commands:");
                println!("  /help                - Show this help");
                println!("  /exit                - Exit the calculator");
                println!("  /mode [mode]         - Switch mode (standard, scientific, programmer)");
                println!("  /save [name]         - Save current calculator state");
                println!("  /restore [name]      - Restore saved calculator state");
                println!("  /list                - List saved states");
                println!("  /delete [name]       - Delete a saved state");
                println!("  /vars                - List all variables");
                println!("  /clear               - Clear all variables");
                println!("  /history             - Show calculation history");
                println!("  /optimize [expr]     - Show optimized version of expression");
                println!("  /validate [expr]     - Validate an expression");
                Ok(None)
            },
            "mode" => {
                if parts.len() < 2 {
                    return Err("Missing mode argument. Use /mode [standard|scientific|programmer]".to_string());
                }
                
                match parts[1] {
                    "standard" => {
                        self.state = Box::new(StandardMode::new());
                        self.notify(&CalculatorEvent::ModeChanged("Standard".to_string()));
                    },
                    "scientific" => {
                        self.state = Box::new(state::ScientificMode::new());
                        self.notify(&CalculatorEvent::ModeChanged("Scientific".to_string()));
                    },
                    "programmer" => {
                        self.state = Box::new(state::ProgrammerMode::new());
                        self.notify(&CalculatorEvent::ModeChanged("Programmer".to_string()));
                    },
                    _ => return Err(format!("Unknown mode: {}", parts[1])),
                }
                
                println!("Switched to {} mode", self.state.name());
                Ok(None)
            },
            "save" => {
                if parts.len() < 2 {
                    return Err("Missing name argument. Use /save [name]".to_string());
                }
                let name = parts[1];
                
                // Create memento
                let memento = self.create_memento();
                
                // Save the state
                self.state_manager.save_state(name, memento);
                Ok(None)
            },
            "restore" => {
                if parts.len() < 2 {
                    return Err("Missing name argument. Use /restore [name]".to_string());
                }
                let name = parts[1];
                
                // Restore the state
                let memento = self.state_manager.restore_state(name)?;
                self.restore_from_memento(&memento)?;
                
                // Notify observers
                self.notify(&CalculatorEvent::StateRestored);
                
                Ok(None)
            },
            "list" => {
                let states = self.state_manager.list_saved_states();
                if states.is_empty() {
                    println!("No saved states");
                } else {
                    println!("Saved states:");
                    for state in states {
                        println!("  {}", state);
                    }
                }
                Ok(None)
            },
            "delete" => {
                if parts.len() < 2 {
                    return Err("Missing name argument. Use /delete [name]".to_string());
                }
                let name = parts[1];
                
                self.state_manager.delete_state(name)?;
                Ok(None)
            },
            "vars" => {
                if self.variables.is_empty() {
                    println!("No variables defined");
                } else {
                    println!("Variables:");
                    for (name, value) in &self.variables {
                        println!("  {} = {}", name, value);
                    }
                }
                Ok(None)
            },
            "clear" => {
                let command = Box::new(ClearVariablesCommand::new());
                self.command_processor.execute(command)?;
                self.variables.clear();
                println!("All variables cleared");
                Ok(None)
            },
            "history" => {
                let history = self.command_processor.get_calculator().history.clone();
                if history.is_empty() {
                    println!("No calculation history");
                } else {
                    println!("Calculation history:");
                    for (i, calc) in HistoryIterator::new(&history).enumerate() {
                        println!("  {}. {} = {}", i + 1, calc.expression, calc.result);
                    }
                }
                Ok(None)
            },
            "optimize" => {
                if parts.len() < 2 {
                    return Err("Missing expression. Use /optimize [expression]".to_string());
                }
                
                let expr_str = &command[parts[0].len()..].trim();
                let expr = self.parser.parse(expr_str)?;
                let optimized = optimize_expression(&*expr, &self.variables)?;
                
                println!("Original: {}", expr.to_string());
                println!("Optimized: {}", optimized.to_string());
                
                Ok(None)
            },
            "validate" => {
                if parts.len() < 2 {
                    return Err("Missing expression. Use /validate [expression]".to_string());
                }
                
                let expr_str = &command[parts[0].len()..].trim();
                let expr = self.parser.parse(expr_str)?;
                
                match validate_expression(&*expr) {
                    Ok(_) => println!("Expression is valid"),
                    Err(e) => println!("Validation errors: {}", e),
                }
                
                Ok(None)
            },
            _ => Err(format!("Unknown command: {}", parts[0])),
        }
    }
    
    fn set_variable(&mut self, name: &str, value: f64) {
        self.variables.insert(name.to_string(), value);
        
        // Execute SetVariableCommand to enable undo/redo
        let command = Box::new(SetVariableCommand::new(name.to_string(), value));
        let _ = self.command_processor.execute(command);
        
        // Notify observers
        self.notify(&CalculatorEvent::VariableChanged(name.to_string(), value));
    }
    
    fn run(&mut self) {
        println!("Correct Calculator - Chapter 8");
        println!("Incorporating patterns from Chapters 5-8");
        println!("Type expressions to evaluate, variables to set (x = 5),");
        println!("commands (/help, /mode, /save, /restore), or /exit to quit");

        loop {
            print!("{} ", self.state.display_prompt());
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                println!("Error reading input, please try again");
                continue;
            }

            let input = input.trim();
            if input == "/exit" {
                break;
            }
            
            match self.process_input(input) {
                Ok(Some(result)) => println!("= {}", result),
                Ok(None) => {}, // Command executed with no result to display
                Err(error) => {
                    println!("Error: {}", error);
                    self.notify(&CalculatorEvent::Error(error));
                }
            }
        }

        println!("Goodbye!");
    }
}

// Implement MementoOriginator for CorrectCalculator
impl MementoOriginator for CorrectCalculator {
    fn create_memento(&self) -> CalculatorMemento {
        CalculatorMemento {
            variables: self.variables.clone(),
            history: self.command_processor.get_calculator().history.clone(),
            mode: get_calculator_state_type(&*self.state),
            angle_mode: get_angle_mode(&*self.state),
            number_base: get_number_base(&*self.state),
        }
    }
    
    fn restore_from_memento(&mut self, memento: &CalculatorMemento) -> Result<(), String> {
        // Restore variables
        self.variables = memento.variables.clone();
        
        // Restore history
        self.command_processor.get_calculator_mut().history = memento.history.clone();
        
        // Restore state
        self.state = memento::create_state_from_memento(memento);
        
        Ok(())
    }
}

// Implement VariableProvider for Arc<Mutex<CorrectCalculator>>
impl VariableProvider for CorrectCalculator {
    fn get_variable(&self, name: &str) -> Option<f64> {
        self.variables.get(name).copied()
    }
    
    fn set_variable(&mut self, name: &str, value: f64) {
        self.set_variable(name, value);
    }
    
    fn evaluate_expression(&mut self, expr: &str) -> Result<f64, String> {
        let expr_tree = self.parser.parse(expr)?;
        expr_tree.evaluate(&self.variables)
    }
}

// Demonstrate pattern integration
fn main() {
    let mut calculator = CorrectCalculator::new();
    calculator.run();
}

// Example using the State pattern directly
fn _run_with_state() {
    println!("Correct Calculator with State Pattern");
    
    let mut calculator = StateCalculator::new();
    
    loop {
        print!("{}", calculator.display_prompt());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input, please try again");
            continue;
        }

        let input = input.trim();
        if input == "/exit" {
            break;
        }

        match calculator.process_input(input) {
            Ok(Some(result)) => println!("= {}", result),
            Ok(None) => {}, // Command executed with no result to display
            Err(error) => println!("Error: {}", error),
        }
    }
    
    println!("Goodbye!");
}

// Example using the Memento pattern directly
fn _run_with_memento() {
    println!("Correct Calculator with Memento Pattern");
    
    let mut calculator = StateCalculator::new();
    let mut state_manager = CalculatorStateManager::new();
    
    loop {
        print!("{}", calculator.display_prompt());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input, please try again");
            continue;
        }

        let input = input.trim();
        if input == "/exit" {
            break;
        }
        
        if input.starts_with("/save ") {
            let name = input.trim_start_matches("/save ").trim();
            let state_type = get_calculator_state_type(&*calculator.state);
            let angle_mode = get_angle_mode(&*calculator.state);
            let number_base = get_number_base(&*calculator.state);
            
            let memento = CalculatorMemento {
                variables: calculator.variables.clone(),
                history: calculator.results_history.clone().into_iter()
                    .map(|(expr, result)| command::Calculation {
                        expression: expr,
                        result,
                        timestamp: std::time::SystemTime::now(),
                    })
                    .collect(),
                mode: state_type,
                angle_mode,
                number_base,
            };
            
            state_manager.save_state(name, memento);
            continue;
        } else if input.starts_with("/restore ") {
            let name = input.trim_start_matches("/restore ").trim();
            match state_manager.restore_state(name) {
                Ok(memento) => {
                    calculator.variables = memento.variables.clone();
                    calculator.results_history = memento.history.iter()
                        .map(|calc| (calc.expression.clone(), calc.result))
                        .collect();
                    calculator.state = memento::create_state_from_memento(&memento);
                    println!("State '{}' restored", name);
                },
                Err(e) => println!("Error: {}", e),
            }
            continue;
        }

        match calculator.process_input(input) {
            Ok(Some(result)) => println!("= {}", result),
            Ok(None) => {}, // Command executed with no result to display
            Err(error) => println!("Error: {}", error),
        }
    }
    
    println!("Goodbye!");
}

// Example using the Observer pattern directly
fn _run_with_observer() {
    println!("Correct Calculator with Observer Pattern");
    
    let mut calculator = StateCalculator::new();
    let mut observable = ObservableCalculator::new();
    
    // Add observers
    observable.attach(Box::new(LoggerObserver));
    
    loop {
        print!("{}", calculator.display_prompt());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input, please try again");
            continue;
        }

        let input = input.trim();
        if input == "/exit" {
            break;
        }

        match calculator.process_input(input) {
            Ok(Some(result)) => {
                println!("= {}", result);
                observable.notify(&CalculatorEvent::ResultCalculated(result, input.to_string()));
            },
            Ok(None) => {}, // Command executed with no result to display
            Err(error) => {
                println!("Error: {}", error);
                observable.notify(&CalculatorEvent::Error(error));
            },
        }
    }
    
    println!("Goodbye!");
}

// Example using the Visitor pattern directly
fn _run_with_visitor() {
    println!("Correct Calculator with Visitor Pattern");
    
    let parser = ExpressionParser::new();
    let mut variables = HashMap::new();
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input, please try again");
            continue;
        }

        let input = input.trim();
        if input == "exit" {
            break;
        }
        
        if input.starts_with("optimize ") {
            let expr_str = input.trim_start_matches("optimize ").trim();
            match parser.parse(expr_str) {
                Ok(expr) => {
                    match optimize_expression(&*expr, &variables) {
                        Ok(optimized) => {
                            println!("Original: {}", expr.to_string());
                            println!("Optimized: {}", optimized.to_string());
                            
                            match optimized.evaluate(&variables) {
                                Ok(result) => println!("Result: {}", result),
                                Err(e) => println!("Evaluation error: {}", e),
                            }
                        },
                        Err(e) => println!("Optimization error: {}", e),
                    }
                },
                Err(e) => println!("Parsing error: {}", e),
            }
            continue;
        }

        match parser.parse(input) {
            Ok(expr) => {
                match expr.evaluate(&variables) {
                    Ok(result) => println!("= {}", result),
                    Err(e) => println!("Error: {}", e),
                }
            },
            Err(e) => println!("Error: {}", e),
        }
    }
    
    println!("Goodbye!");
}
