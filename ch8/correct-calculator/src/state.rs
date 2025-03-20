// state.rs - State pattern implementation for calculator modes

use std::collections::HashMap;
use crate::expression::{Expression, NumberExpression};
use crate::parser::ExpressionParser;
use crate::config::AngleMode;
use crate::adapter::ScientificOperations;

// Enum to represent different number bases for programmer mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NumberBase {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

impl NumberBase {
    pub fn format(&self, value: f64) -> String {
        let value = value as i64; // Convert to integer for non-decimal bases
        match self {
            NumberBase::Binary => format!("0b{:b}", value),
            NumberBase::Octal => format!("0o{:o}", value),
            NumberBase::Decimal => format!("{}", value),
            NumberBase::Hexadecimal => format!("0x{:X}", value),
        }
    }
    
    pub fn parse(&self, text: &str) -> Result<f64, String> {
        match self {
            NumberBase::Binary => {
                if let Some(value) = text.strip_prefix("0b") {
                    i64::from_str_radix(value, 2)
                        .map(|v| v as f64)
                        .map_err(|_| format!("Invalid binary number: {}", text))
                } else {
                    Err(format!("Binary numbers must start with 0b: {}", text))
                }
            },
            NumberBase::Octal => {
                if let Some(value) = text.strip_prefix("0o") {
                    i64::from_str_radix(value, 8)
                        .map(|v| v as f64)
                        .map_err(|_| format!("Invalid octal number: {}", text))
                } else {
                    Err(format!("Octal numbers must start with 0o: {}", text))
                }
            },
            NumberBase::Decimal => {
                text.parse::<f64>()
                    .map_err(|_| format!("Invalid decimal number: {}", text))
            },
            NumberBase::Hexadecimal => {
                if let Some(value) = text.strip_prefix("0x") {
                    i64::from_str_radix(value, 16)
                        .map(|v| v as f64)
                        .map_err(|_| format!("Invalid hexadecimal number: {}", text))
                } else {
                    Err(format!("Hexadecimal numbers must start with 0x: {}", text))
                }
            },
        }
    }
}

// Calculator context for state pattern
pub struct StateCalculator {
    pub state: Box<dyn CalculatorState>,
    pub variables: HashMap<String, f64>,
    pub parser: ExpressionParser,
    pub results_history: Vec<(String, f64)>,
}

impl StateCalculator {
    pub fn new() -> Self {
        Self {
            state: Box::new(StandardMode::new()),
            variables: HashMap::new(),
            parser: ExpressionParser::new(),
            results_history: Vec::new(),
        }
    }
    
    pub fn change_state(&mut self, new_state: Box<dyn CalculatorState>) {
        println!("Switching to {} mode", new_state.name());
        self.state = new_state;
    }
    
    pub fn process_input(&mut self, input: &str) -> Result<Option<f64>, String> {
        self.state.handle_input(input, self)
    }
    
    pub fn store_result(&mut self, input: String, result: f64) {
        self.results_history.push((input, result));
        self.variables.insert("ans".to_string(), result);
    }
    
    pub fn display_prompt(&self) -> String {
        self.state.display_prompt()
    }
}

// State interface
pub trait CalculatorState {
    fn name(&self) -> &str;
    fn handle_input(&self, input: &str, calculator: &mut StateCalculator) -> Result<Option<f64>, String>;
    fn available_operations(&self) -> Vec<&'static str>;
    fn display_prompt(&self) -> String;
}

// Standard calculator mode
pub struct StandardMode {
    pub sci_ops: Box<dyn ScientificOperations>,
}

impl StandardMode {
    pub fn new() -> Self {
        use crate::adapter::StandardScientificOperations;
        Self {
            sci_ops: Box::new(StandardScientificOperations {
                angle_mode: AngleMode::Radians,
            }),
        }
    }
}

impl CalculatorState for StandardMode {
    fn name(&self) -> &str {
        "Standard"
    }
    
    fn handle_input(&self, input: &str, calculator: &mut StateCalculator) -> Result<Option<f64>, String> {
        // Handle basic arithmetic expressions
        if input.starts_with("mode") {
            // Change mode based on command
            let mode = input.trim_start_matches("mode").trim();
            match mode {
                "scientific" => {
                    calculator.change_state(Box::new(ScientificMode::new()));
                    Ok(None)
                },
                "programmer" => {
                    calculator.change_state(Box::new(ProgrammerMode::new()));
                    Ok(None)
                },
                _ => Err(format!("Unknown mode: {}", mode)),
            }
        } else if input.starts_with("help") {
            println!("Available operations: {}", self.available_operations().join(", "));
            println!("Type 'mode scientific' or 'mode programmer' to switch modes");
            Ok(None)
        } else if let Some((var_name, expression)) = input.split_once('=') {
            let var_name = var_name.trim();
            let expression = expression.trim();
            
            // Evaluate the expression and set the variable
            let expr = calculator.parser.parse(expression)?;
            let result = expr.evaluate(&calculator.variables)?;
            calculator.variables.insert(var_name.to_string(), result);
            calculator.store_result(format!("{} = {}", var_name, expression), result);
            Ok(Some(result))
        } else {
            // Normal expression evaluation
            let expr = calculator.parser.parse(input)?;
            let result = expr.evaluate(&calculator.variables)?;
            calculator.store_result(input.to_string(), result);
            Ok(Some(result))
        }
    }
    
    fn available_operations(&self) -> Vec<&'static str> {
        vec!["+", "-", "*", "/", "^"]
    }
    
    fn display_prompt(&self) -> String {
        "[Standard] > ".to_string()
    }
}

// Scientific calculator mode
pub struct ScientificMode {
    pub sci_ops: Box<dyn ScientificOperations>,
    pub angle_mode: AngleMode,
}

impl ScientificMode {
    pub fn new() -> Self {
        use crate::adapter::StandardScientificOperations;
        Self {
            sci_ops: Box::new(StandardScientificOperations {
                angle_mode: AngleMode::Radians,
            }),
            angle_mode: AngleMode::Radians,
        }
    }
}

impl CalculatorState for ScientificMode {
    fn name(&self) -> &str {
        "Scientific"
    }
    
    fn handle_input(&self, input: &str, calculator: &mut StateCalculator) -> Result<Option<f64>, String> {
        // Handle scientific expressions and functions
        if input.starts_with("mode") {
            // Handle mode change
            let mode = input.trim_start_matches("mode").trim();
            match mode {
                "standard" => {
                    calculator.change_state(Box::new(StandardMode::new()));
                    Ok(None)
                },
                "programmer" => {
                    calculator.change_state(Box::new(ProgrammerMode::new()));
                    Ok(None)
                },
                _ => Err(format!("Unknown mode: {}", mode)),
            }
        } else if input == "angle deg" {
            // Change angle mode to degrees
            calculator.change_state(Box::new(ScientificMode {
                sci_ops: Box::new(crate::adapter::StandardScientificOperations {
                    angle_mode: AngleMode::Degrees,
                }),
                angle_mode: AngleMode::Degrees,
            }));
            println!("Angle mode set to degrees");
            Ok(None)
        } else if input == "angle rad" {
            // Change angle mode to radians
            calculator.change_state(Box::new(ScientificMode {
                sci_ops: Box::new(crate::adapter::StandardScientificOperations {
                    angle_mode: AngleMode::Radians,
                }),
                angle_mode: AngleMode::Radians,
            }));
            println!("Angle mode set to radians");
            Ok(None)
        } else if input.starts_with("help") {
            println!("Available operations: {}", self.available_operations().join(", "));
            println!("Type 'mode standard' or 'mode programmer' to switch modes");
            println!("Type 'angle deg' or 'angle rad' to change angle mode");
            Ok(None)
        } else if input.starts_with("sin ") || input.starts_with("cos ") || input.starts_with("tan ") {
            // Handle trigonometric functions
            let (func, arg_str) = input.split_once(' ').unwrap();
            
            // Parse and evaluate the argument
            let expr = calculator.parser.parse(arg_str)?;
            let arg = expr.evaluate(&calculator.variables)?;
            
            let result = match func {
                "sin" => self.sci_ops.sin(arg),
                "cos" => self.sci_ops.cos(arg),
                "tan" => self.sci_ops.tan(arg),
                _ => unreachable!(),
            };
            
            calculator.store_result(input.to_string(), result);
            Ok(Some(result))
        } else if input.starts_with("log") {
            // Handle logarithm with base
            let parts: Vec<&str> = input.splitn(3, ' ').collect();
            if parts.len() != 3 {
                return Err("Usage: log <base> <value>".to_string());
            }
            
            let base_expr = calculator.parser.parse(parts[1])?;
            let value_expr = calculator.parser.parse(parts[2])?;
            
            let base = base_expr.evaluate(&calculator.variables)?;
            let value = value_expr.evaluate(&calculator.variables)?;
            
            let result = self.sci_ops.log(value, base)?;
            calculator.store_result(input.to_string(), result);
            Ok(Some(result))
        } else if let Some((var_name, expression)) = input.split_once('=') {
            // Handle variable assignment
            let var_name = var_name.trim();
            let expression = expression.trim();
            
            let expr = calculator.parser.parse(expression)?;
            let result = expr.evaluate(&calculator.variables)?;
            calculator.variables.insert(var_name.to_string(), result);
            calculator.store_result(format!("{} = {}", var_name, expression), result);
            Ok(Some(result))
        } else {
            // Handle normal expressions with scientific operations
            let expr = calculator.parser.parse(input)?;
            let result = expr.evaluate(&calculator.variables)?;
            calculator.store_result(input.to_string(), result);
            Ok(Some(result))
        }
    }
    
    fn available_operations(&self) -> Vec<&'static str> {
        vec!["+", "-", "*", "/", "^", "sin", "cos", "tan", "log", "ln", "sqrt"]
    }
    
    fn display_prompt(&self) -> String {
        match self.angle_mode {
            AngleMode::Radians => "[Scientific (RAD)] > ".to_string(),
            AngleMode::Degrees => "[Scientific (DEG)] > ".to_string(),
        }
    }
}

// Programmer calculator mode
pub struct ProgrammerMode {
    pub base: NumberBase,
}

impl ProgrammerMode {
    pub fn new() -> Self {
        Self {
            base: NumberBase::Decimal,
        }
    }
    
    // Helper for bitwise operations
    fn execute_bitwise_op(&self, a: f64, b: f64, op: fn(i64, i64) -> i64) -> f64 {
        let a_int = a as i64;
        let b_int = b as i64;
        op(a_int, b_int) as f64
    }
}

impl CalculatorState for ProgrammerMode {
    fn name(&self) -> &str {
        "Programmer"
    }
    
    fn handle_input(&self, input: &str, calculator: &mut StateCalculator) -> Result<Option<f64>, String> {
        // Handle programmer mode commands and operations
        if input.starts_with("mode") {
            // Handle mode change
            let mode = input.trim_start_matches("mode").trim();
            match mode {
                "standard" => {
                    calculator.change_state(Box::new(StandardMode::new()));
                    Ok(None)
                },
                "scientific" => {
                    calculator.change_state(Box::new(ScientificMode::new()));
                    Ok(None)
                },
                _ => Err(format!("Unknown mode: {}", mode)),
            }
        } else if input.starts_with("base") {
            // Change number base
            let base = input.trim_start_matches("base").trim();
            let new_base = match base {
                "bin" | "binary" => NumberBase::Binary,
                "oct" | "octal" => NumberBase::Octal,
                "dec" | "decimal" => NumberBase::Decimal,
                "hex" | "hexadecimal" => NumberBase::Hexadecimal,
                _ => return Err(format!("Unknown base: {}", base)),
            };
            
            calculator.change_state(Box::new(ProgrammerMode { base: new_base }));
            println!("Base set to {:?}", new_base);
            Ok(None)
        } else if input.starts_with("help") {
            println!("Available operations: {}", self.available_operations().join(", "));
            println!("Type 'mode standard' or 'mode scientific' to switch modes");
            println!("Type 'base bin', 'base oct', 'base dec', or 'base hex' to change base");
            println!("Bitwise operations: AND, OR, XOR, NOT, SHL, SHR");
            Ok(None)
        } else if input.starts_with("AND ") || input.starts_with("OR ") || input.starts_with("XOR ") {
            // Handle bitwise binary operations
            let parts: Vec<&str> = input.splitn(3, ' ').collect();
            if parts.len() != 3 {
                return Err(format!("Usage: {} <operand1> <operand2>", parts[0]));
            }
            
            let op = parts[0];
            let a_expr = calculator.parser.parse(parts[1])?;
            let b_expr = calculator.parser.parse(parts[2])?;
            
            let a = a_expr.evaluate(&calculator.variables)?;
            let b = b_expr.evaluate(&calculator.variables)?;
            
            let result = match op {
                "AND" => self.execute_bitwise_op(a, b, |a, b| a & b),
                "OR" => self.execute_bitwise_op(a, b, |a, b| a | b),
                "XOR" => self.execute_bitwise_op(a, b, |a, b| a ^ b),
                _ => unreachable!(),
            };
            
            calculator.store_result(input.to_string(), result);
            println!("{} = {}", input, self.base.format(result));
            Ok(Some(result))
        } else if input.starts_with("NOT ") {
            // Handle bitwise NOT operation
            let expr_str = input.trim_start_matches("NOT ").trim();
            let expr = calculator.parser.parse(expr_str)?;
            let value = expr.evaluate(&calculator.variables)?;
            
            let result = self.execute_bitwise_op(value, 0.0, |a, _| !a);
            calculator.store_result(input.to_string(), result);
            println!("{} = {}", input, self.base.format(result));
            Ok(Some(result))
        } else if input.starts_with("SHL ") || input.starts_with("SHR ") {
            // Handle shift operations
            let parts: Vec<&str> = input.splitn(3, ' ').collect();
            if parts.len() != 3 {
                return Err(format!("Usage: {} <value> <bits>", parts[0]));
            }
            
            let op = parts[0];
            let value_expr = calculator.parser.parse(parts[1])?;
            let bits_expr = calculator.parser.parse(parts[2])?;
            
            let value = value_expr.evaluate(&calculator.variables)?;
            let bits = bits_expr.evaluate(&calculator.variables)? as u32;
            
            let result = match op {
                "SHL" => self.execute_bitwise_op(value, bits as f64, |a, b| a << b as u32),
                "SHR" => self.execute_bitwise_op(value, bits as f64, |a, b| a >> b as u32),
                _ => unreachable!(),
            };
            
            calculator.store_result(input.to_string(), result);
            println!("{} = {}", input, self.base.format(result));
            Ok(Some(result))
        } else if let Some((var_name, expression)) = input.split_once('=') {
            // Handle variable assignment
            let var_name = var_name.trim();
            let expression = expression.trim();
            
            // Try to parse according to current base
            let result = if !expression.contains(|c: char| c.is_ascii_letter() || "+-*/()^".contains(c)) {
                match self.base.parse(expression) {
                    Ok(value) => value,
                    Err(_) => {
                        // Fall back to regular parser if base-specific parsing fails
                        let expr = calculator.parser.parse(expression)?;
                        expr.evaluate(&calculator.variables)?
                    }
                }
            } else {
                // For expressions, use the regular parser
                let expr = calculator.parser.parse(expression)?;
                expr.evaluate(&calculator.variables)?
            };
            
            calculator.variables.insert(var_name.to_string(), result);
            calculator.store_result(format!("{} = {}", var_name, expression), result);
            println!("{} = {}", var_name, self.base.format(result));
            Ok(Some(result))
        } else {
            // Normal expression evaluation
            let expr = calculator.parser.parse(input)?;
            let result = expr.evaluate(&calculator.variables)?;
            calculator.store_result(input.to_string(), result);
            println!("= {}", self.base.format(result));
            Ok(Some(result))
        }
    }
    
    fn available_operations(&self) -> Vec<&'static str> {
        vec!["+", "-", "*", "/", "AND", "OR", "XOR", "NOT", "SHL", "SHR"]
    }
    
    fn display_prompt(&self) -> String {
        match self.base {
            NumberBase::Binary => "[Programmer (BIN)] > ".to_string(),
            NumberBase::Octal => "[Programmer (OCT)] > ".to_string(),
            NumberBase::Decimal => "[Programmer (DEC)] > ".to_string(),
            NumberBase::Hexadecimal => "[Programmer (HEX)] > ".to_string(),
        }
    }
}
