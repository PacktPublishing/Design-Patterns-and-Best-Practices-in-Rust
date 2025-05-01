// strategy.rs - Strategy pattern implementation

use std::collections::HashMap;
use crate::expression::{Expression, NumberExpression, VariableExpression, BinaryOperation, FunctionCall};
use crate::token::{Token, Operator, Function};

// Strategy for evaluating expressions
pub trait EvaluationStrategy {
    fn evaluate(&self, expression: &str, variables: &HashMap<String, f64>) -> Result<f64, String>;
}

// Strategy for formatting results
pub trait PrecisionStrategy {
    fn format(&self, value: f64) -> String;
    fn round(&self, value: f64) -> f64;
}

// Standard precision strategy (fixed decimal places)
pub struct StandardPrecision {
    decimal_places: usize,
}

impl StandardPrecision {
    pub fn new(decimal_places: usize) -> Self {
        Self { decimal_places }
    }
}

impl PrecisionStrategy for StandardPrecision {
    fn format(&self, value: f64) -> String {
        format!("{:.1$}", value, self.decimal_places)
    }
    
    fn round(&self, value: f64) -> f64 {
        let factor = 10.0_f64.powi(self.decimal_places as i32);
        (value * factor).round() / factor
    }
}

// Scientific precision strategy (significant figures)
pub struct ScientificPrecision {
    significant_figures: usize,
}

impl ScientificPrecision {
    pub fn new(significant_figures: usize) -> Self {
        Self { significant_figures }
    }
}

impl PrecisionStrategy for ScientificPrecision {
    fn format(&self, value: f64) -> String {
        // Format in scientific notation with specified significant figures
        if value == 0.0 {
            return "0.0e0".to_string();
        }
        
        let mag = value.abs().log10().floor();
        let mantissa = value / 10.0_f64.powf(mag);
        
        format!("{:.1$}e{2}", mantissa, self.significant_figures - 1, mag as i32)
    }
    
    fn round(&self, value: f64) -> f64 {
        if value == 0.0 {
            return 0.0;
        }
        
        let mag = value.abs().log10().floor();
        let mantissa = value / 10.0_f64.powf(mag);
        
        let factor = 10.0_f64.powi(self.significant_figures as i32 - 1);
        let rounded_mantissa = (mantissa * factor).round() / factor;
        
        rounded_mantissa * 10.0_f64.powf(mag)
    }
}

// Strategy for tokenization
pub trait TokenizationStrategy: Send + Sync {
    fn tokenize(&self, expression: &str) -> Result<Vec<Token>, String>;
}

// Simple tokenization strategy
pub struct SimpleTokenizer;

impl TokenizationStrategy for SimpleTokenizer {
    fn tokenize(&self, expression: &str) -> Result<Vec<Token>, String> {
        // Simple space-delimited tokenization for demonstration
        expression.split_whitespace()
            .map(Token::from_str)
            .collect()
    }
}

// Recursive descent evaluation strategy
pub struct RecursiveDescentStrategy {
    tokenizer: Box<dyn TokenizationStrategy>,
}

impl RecursiveDescentStrategy {
    pub fn new(tokenizer: Box<dyn TokenizationStrategy>) -> Self {
        Self { tokenizer }
    }
    
    fn parse_expression(&self, tokens: &[Token]) -> Result<Box<dyn Expression>, String> {
        // Implement recursive descent parsing
        if tokens.is_empty() {
            return Err("Empty expression".to_string());
        }
        
        self.parse_addition(tokens, 0).map(|(expr, _)| expr)
    }
    
    fn parse_addition(&self, tokens: &[Token], pos: usize) -> Result<(Box<dyn Expression>, usize), String> {
        // Parse term
        let (mut left, mut next_pos) = self.parse_multiplication(tokens, pos)?;
        
        // Parse additional terms
        while next_pos < tokens.len() {
            if let Some(Token::Operator(op)) = tokens.get(next_pos) {
                if op.precedence() == 1 {
                    let (right, new_pos) = self.parse_multiplication(tokens, next_pos + 1)?;
                    left = Box::new(BinaryOperation::new(left, right, op.clone()));
                    next_pos = new_pos;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        Ok((left, next_pos))
    }
    
    fn parse_multiplication(&self, tokens: &[Token], pos: usize) -> Result<(Box<dyn Expression>, usize), String> {
        // Parse factor
        let (mut left, mut next_pos) = self.parse_primary(tokens, pos)?;
        
        // Parse additional factors
        while next_pos < tokens.len() {
            if let Some(Token::Operator(op)) = tokens.get(next_pos) {
                if op.precedence() > 1 {
                    let (right, new_pos) = self.parse_primary(tokens, next_pos + 1)?;
                    left = Box::new(BinaryOperation::new(left, right, op.clone()));
                    next_pos = new_pos;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        Ok((left, next_pos))
    }
    
    fn parse_primary(&self, tokens: &[Token], pos: usize) -> Result<(Box<dyn Expression>, usize), String> {
        match tokens.get(pos) {
            Some(Token::Number(num)) => Ok((Box::new(NumberExpression::new(num.value)), pos + 1)),
            Some(Token::Variable(name)) => Ok((Box::new(VariableExpression::new(name.clone())), pos + 1)),
            Some(Token::OpenParen) => {
                // Parse the subexpression recursively
                let result = self.parse_addition(&tokens[pos+1..], 0)?;
                let (expr, inner_pos) = result;
                let closing_pos = pos + 1 + inner_pos;
                
                // Check for closing parenthesis
                if tokens.get(closing_pos) != Some(&Token::CloseParen) {
                    return Err("Missing closing parenthesis".to_string());
                }
                Ok((expr, closing_pos + 1))
            },
            Some(Token::Function(func)) => {
                // Check for opening parenthesis
                if tokens.get(pos + 1) != Some(&Token::OpenParen) {
                    return Err("Missing opening parenthesis after function".to_string());
                }
                
                // Parse the argument recursively
                let result = self.parse_addition(&tokens[pos+2..], 0)?;
                let (arg, inner_pos) = result;
                let closing_pos = pos + 2 + inner_pos;
                
                // Check for closing parenthesis
                if tokens.get(closing_pos) != Some(&Token::CloseParen) {
                    return Err("Missing closing parenthesis for function".to_string());
                }
                
                Ok((Box::new(FunctionCall::new(func.clone(), arg)), closing_pos + 1))
            },
            _ => Err("Unexpected token".to_string()),
        }
    }
}

impl EvaluationStrategy for RecursiveDescentStrategy {
    fn evaluate(&self, expression: &str, variables: &HashMap<String, f64>) -> Result<f64, String> {
        let tokens = self.tokenizer.tokenize(expression)?;
        let ast = self.parse_expression(&tokens)?;
        ast.evaluate(variables)
    }
}

// Shunting Yard evaluation strategy
pub struct ShuntingYardStrategy {
    tokenizer: Box<dyn TokenizationStrategy>,
}

impl ShuntingYardStrategy {
    pub fn new(tokenizer: Box<dyn TokenizationStrategy>) -> Self {
        Self { tokenizer }
    }
    
    fn parse(&self, tokens: &[Token]) -> Result<Box<dyn Expression>, String> {
        let mut output_queue: Vec<Box<dyn Expression>> = Vec::new();
        let mut operator_stack: Vec<Token> = Vec::new();
        
        for token in tokens {
            match token {
                Token::Number(num) => {
                    output_queue.push(Box::new(NumberExpression::new(num.value)));
                },
                Token::Variable(name) => {
                    output_queue.push(Box::new(VariableExpression::new(name)));
                },
                Token::Operator(op) => {
                    // While there's an operator on the stack with greater precedence
                    loop {
                        if let Some(Token::Operator(top_op)) = operator_stack.last().cloned() {
                            if top_op.precedence() >= op.precedence() {
                                operator_stack.pop();
                                
                                if output_queue.len() < 2 {
                                    return Err("Invalid expression: not enough operands".to_string());
                                }
                                
                                let right = output_queue.pop().unwrap();
                                let left = output_queue.pop().unwrap();
                                
                                output_queue.push(Box::new(BinaryOperation::new(left, right, top_op)));
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    
                    operator_stack.push(Token::Operator(op.clone()));
                },
                Token::Function(func) => {
                    operator_stack.push(Token::Function(func.clone()));
                },
                Token::OpenParen => {
                    operator_stack.push(token.clone());
                },
                Token::CloseParen => {
                    // Pop until matching open paren
                    let mut found_open_paren = false;
                    
                    while let Some(top) = operator_stack.pop() {
                        match top {
                            Token::OpenParen => {
                                found_open_paren = true;
                                
                                // Check if there's a function on the stack
                                if let Some(function_idx) = operator_stack.iter().position(|t| matches!(t, Token::Function(_))) {
                                    if let Token::Function(func) = &operator_stack[function_idx] {
                                        // Remove the function token
                                        let func = func.clone();
                                        operator_stack.remove(function_idx);
                                        
                                        if output_queue.is_empty() {
                                            return Err("Invalid function call: missing argument".to_string());
                                        }
                                        
                                        let arg = output_queue.pop().unwrap();
                                        output_queue.push(Box::new(FunctionCall::new(func, arg)));
                                    }
                                }
                                
                                break;
                            },
                            Token::Operator(op) => {
                                if output_queue.len() < 2 {
                                    return Err("Invalid expression: not enough operands".to_string());
                                }
                                
                                let right = output_queue.pop().unwrap();
                                let left = output_queue.pop().unwrap();
                                
                                output_queue.push(Box::new(BinaryOperation::new(left, right, op)));
                            },
                            _ => return Err(format!("Unexpected token on operator stack: {:?}", top)),
                        }
                    }
                    
                    if !found_open_paren {
                        return Err("Mismatched parentheses".to_string());
                    }
                }
            }
        }
        
        // Pop remaining operators
        while let Some(top) = operator_stack.pop() {
            match top {
                Token::Operator(op) => {
                    if output_queue.len() < 2 {
                        return Err("Invalid expression: not enough operands".to_string());
                    }
                    
                    let right = output_queue.pop().unwrap();
                    let left = output_queue.pop().unwrap();
                    
                    output_queue.push(Box::new(BinaryOperation::new(left, right, op)));
                },
                Token::OpenParen => {
                    return Err("Mismatched parentheses".to_string());
                },
                _ => return Err(format!("Unexpected token on operator stack: {:?}", top)),
            }
        }
        
        // Result should be a single expression
        if output_queue.len() != 1 {
            return Err(format!("Invalid expression: expected 1 result, got {}", output_queue.len()));
        }
        
        Ok(output_queue.pop().unwrap())
    }
}

impl EvaluationStrategy for ShuntingYardStrategy {
    fn evaluate(&self, expression: &str, variables: &HashMap<String, f64>) -> Result<f64, String> {
        let tokens = self.tokenizer.tokenize(expression)?;
        let ast = self.parse(&tokens)?;
        ast.evaluate(variables)
    }
}

// Advanced tokenizer (handles operators without spaces)
pub struct AdvancedTokenizer;

impl TokenizationStrategy for AdvancedTokenizer {
    fn tokenize(&self, expression: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut chars = expression.chars().peekable();
        
        while let Some(c) = chars.next() {
            match c {
                // Skip whitespace
                c if c.is_whitespace() => continue,
                
                // Numbers
                c if c.is_digit(10) || c == '.' => {
                    let mut num_str = c.to_string();
                    
                    while let Some(next_c) = chars.peek() {
                        if next_c.is_digit(10) || *next_c == '.' {
                            num_str.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    
                    match num_str.parse::<f64>() {
                        Ok(value) => tokens.push(Token::number(value)),
                        Err(_) => return Err(format!("Invalid number: {}", num_str)),
                    }
                },
                
                // Variables and functions
                c if c.is_alphabetic() => {
                    let mut name = c.to_string();
                    
                    while let Some(next_c) = chars.peek() {
                        if next_c.is_alphanumeric() || *next_c == '_' {
                            name.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    
                    // Check if it's a function or variable
                    if chars.peek() == Some(&'(') {
                        // Function
                        match name.as_str() {
                            "sin" => tokens.push(Token::Function(Function::Sin)),
                            "cos" => tokens.push(Token::Function(Function::Cos)),
                            "tan" => tokens.push(Token::Function(Function::Tan)),
                            "sqrt" => tokens.push(Token::Function(Function::Sqrt)),
                            _ => return Err(format!("Unknown function: {}", name)),
                        }
                    } else {
                        // Variable
                        tokens.push(Token::variable(name));
                    }
                },
                
                // Operators
                '+' => tokens.push(Token::Operator(Operator::Add)),
                '-' => tokens.push(Token::Operator(Operator::Subtract)),
                '*' => tokens.push(Token::Operator(Operator::Multiply)),
                '/' => tokens.push(Token::Operator(Operator::Divide)),
                '^' => tokens.push(Token::Operator(Operator::Power)),
                
                // Parentheses
                '(' => tokens.push(Token::OpenParen),
                ')' => tokens.push(Token::CloseParen),
                
                _ => return Err(format!("Unknown character: {}", c)),
            }
        }
        
        Ok(tokens)
    }
}

// Context that uses the strategies
pub struct ExpressionEvaluator {
    evaluation_strategy: Box<dyn EvaluationStrategy>,
    precision_strategy: Box<dyn PrecisionStrategy>,
}

impl ExpressionEvaluator {
    pub fn new(
        evaluation_strategy: Box<dyn EvaluationStrategy>,
        precision_strategy: Box<dyn PrecisionStrategy>,
    ) -> Self {
        Self {
            evaluation_strategy,
            precision_strategy,
        }
    }
    
    pub fn evaluate(&self, expression: &str, variables: &HashMap<String, f64>) -> Result<f64, String> {
        self.evaluation_strategy.evaluate(expression, variables)
    }
    
    pub fn format_result(&self, result: f64) -> String {
        self.precision_strategy.format(result)
    }
    
    pub fn set_evaluation_strategy(&mut self, strategy: Box<dyn EvaluationStrategy>) {
        self.evaluation_strategy = strategy;
    }
    
    pub fn set_precision_strategy(&mut self, strategy: Box<dyn PrecisionStrategy>) {
        self.precision_strategy = strategy;
    }
}

// Factory functions for creating evaluators with common configurations
pub fn create_standard_evaluator() -> ExpressionEvaluator {
    ExpressionEvaluator::new(
        Box::new(RecursiveDescentStrategy::new(Box::new(AdvancedTokenizer))),
        Box::new(StandardPrecision::new(4)),
    )
}

pub fn create_scientific_evaluator() -> ExpressionEvaluator {
    ExpressionEvaluator::new(
        Box::new(ShuntingYardStrategy::new(Box::new(AdvancedTokenizer))),
        Box::new(ScientificPrecision::new(6)),
    )
}