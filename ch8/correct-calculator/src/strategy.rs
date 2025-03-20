// strategy.rs - Strategy pattern implementation

use std::collections::HashMap;
use crate::token::{Token, Operator, Function};
use crate::expression::{Expression, NumberExpression, VariableExpression, BinaryOperation, FunctionCall};

// Strategy interface for expression evaluation
pub trait EvaluationStrategy {
    fn evaluate(&self, expression: &str, variables: &HashMap<String, f64>) -> Result<f64, String>;
}

// Strategy for tokenization
pub trait TokenizationStrategy {
    fn tokenize(&self, input: &str) -> Result<Vec<Token>, String>;
}

// Strategy for numeric precision
pub trait PrecisionStrategy {
    fn format(&self, value: f64) -> String;
    fn round(&self, value: f64) -> f64;
}

// Standard precision implementation
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
        format!("{:.*}", self.decimal_places, value)
    }
    
    fn round(&self, value: f64) -> f64 {
        let factor = 10.0f64.powi(self.decimal_places as i32);
        (value * factor).round() / factor
    }
}

// Scientific precision implementation
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
        // Format with significant figures
        format!("{:.*e}", self.significant_figures - 1, value)
    }
    
    fn round(&self, value: f64) -> f64 {
        // Implementation for significant figure rounding
        if value == 0.0 {
            return 0.0;
        }
        
        let sign = value.signum();
        let abs_value = value.abs();
        let magnitude = abs_value.log10().floor();
        let scale = 10.0f64.powf(magnitude - (self.significant_figures as f64 - 1.0));
        
        sign * ((abs_value / scale).round() * scale)
    }
}

// Standard tokenization strategy
pub struct SimpleTokenizer;

impl TokenizationStrategy for SimpleTokenizer {
    fn tokenize(&self, input: &str) -> Result<Vec<Token>, String> {
        // Simple space-delimited tokenization
        let tokens: Result<Vec<Token>, String> = input
            .split_whitespace()
            .map(Token::from_str)
            .collect();
        
        tokens
    }
}

// Recursive descent parser strategy
pub struct RecursiveDescentStrategy {
    tokenizer: Box<dyn TokenizationStrategy>,
}

impl RecursiveDescentStrategy {
    pub fn new(tokenizer: Box<dyn TokenizationStrategy>) -> Self {
        Self { tokenizer }
    }
    
    // Helper function to parse expressions
    fn parse_expression(&self, tokens: &[Token]) -> Result<Box<dyn Expression>, String> {
        if tokens.is_empty() {
            return Err("Empty expression".to_string());
        }
        
        // This is a simplified recursive descent parser
        // A real one would be more complex with proper grammar rules
        self.parse_addition(tokens)
    }
    
    fn parse_addition(&self, tokens: &[Token]) -> Result<Box<dyn Expression>, String> {
        let mut left = self.parse_multiplication(tokens)?;
        
        // For simplicity, we're not handling the token indices correctly here
        // A real implementation would keep track of the current token index
        for i in 0..tokens.len() {
            if let Token::Operator(op @ (Operator::Add | Operator::Subtract)) = &tokens[i] {
                if i + 1 < tokens.len() {
                    let right = self.parse_multiplication(&tokens[i+1..])?;
                    left = Box::new(BinaryOperation::new(left, right, op.clone()));
                }
            }
        }
        
        Ok(left)
    }
    
    fn parse_multiplication(&self, tokens: &[Token]) -> Result<Box<dyn Expression>, String> {
        let mut left = self.parse_primary(tokens)?;
        
        // Simplified for demonstration
        for i in 0..tokens.len() {
            if let Token::Operator(op @ (Operator::Multiply | Operator::Divide | Operator::Power)) = &tokens[i] {
                if i + 1 < tokens.len() {
                    let right = self.parse_primary(&tokens[i+1..])?;
                    left = Box::new(BinaryOperation::new(left, right, op.clone()));
                }
            }
        }
        
        Ok(left)
    }
    
    fn parse_primary(&self, tokens: &[Token]) -> Result<Box<dyn Expression>, String> {
        if tokens.is_empty() {
            return Err("Unexpected end of expression".to_string());
        }
        
        match &tokens[0] {
            Token::Number(num) => Ok(Box::new(NumberExpression::new(num.value))),
            Token::Variable(name) => Ok(Box::new(VariableExpression::new(name.clone()))),
            Token::Function(func) => {
                if tokens.len() < 3 || tokens[1] != Token::OpenParen || tokens[tokens.len() - 1] != Token::CloseParen {
                    return Err("Invalid function call syntax".to_string());
                }
                let arg_tokens = &tokens[2..tokens.len() - 1];
                let arg = self.parse_expression(arg_tokens)?;
                Ok(Box::new(FunctionCall::new(func.clone(), arg)))
            },
            Token::OpenParen => {
                // Find matching closing paren
                let mut depth = 1;
                let mut close_idx = 0;
                
                for (i, token) in tokens.iter().enumerate().skip(1) {
                    match token {
                        Token::OpenParen => depth += 1,
                        Token::CloseParen => {
                            depth -= 1;
                            if depth == 0 {
                                close_idx = i;
                                break;
                            }
                        },
                        _ => {}
                    }
                }
                
                if depth != 0 {
                    return Err("Mismatched parentheses".to_string());
                }
                
                self.parse_expression(&tokens[1..close_idx])
            },
            _ => Err(format!("Unexpected token: {:?}", tokens[0])),
        }
    }
}

impl EvaluationStrategy for RecursiveDescentStrategy {
    fn evaluate(&self, expression: &str, variables: &HashMap<String, f64>) -> Result<f64, String> {
        let tokens = self.tokenizer.tokenize(expression)?;
        let expr = self.parse_expression(&tokens)?;
        expr.evaluate(variables)
    }
}

// Shunting yard algorithm strategy
pub struct ShuntingYardStrategy {
    tokenizer: Box<dyn TokenizationStrategy>,
}

impl ShuntingYardStrategy {
    pub fn new(tokenizer: Box<dyn TokenizationStrategy>) -> Self {
        Self { tokenizer }
    }
    
    fn build_expression_tree(&self, tokens: Vec<Token>) -> Result<Box<dyn Expression>, String> {
        // This is a simplified implementation of the shunting yard algorithm
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
                    while let Some(Token::Operator(top_op)) = operator_stack.last() {
                        if top_op.precedence() >= op.precedence() {
                            operator_stack.pop();
                            
                            if output_queue.len() < 2 {
                                return Err("Invalid expression: not enough operands".to_string());
                            }
                            
                            let right = output_queue.pop().unwrap();
                            let left = output_queue.pop().unwrap();
                            
                            output_queue.push(Box::new(BinaryOperation::new(left, right, top_op.clone())));
                        } else {
                            break;
                        }
                    }
                    
                    operator_stack.push(Token::Operator(op));
                },
                Token::Function(func) => {
                    operator_stack.push(Token::Function(func));
                },
                Token::OpenParen => {
                    operator_stack.push(token);
                },
                Token::CloseParen => {
                    // Pop until matching open paren
                    while let Some(top) = operator_stack.last() {
                        if let Token::OpenParen = top {
                            operator_stack.pop();
                            
                            // If there's a function on the stack, apply it
                            if let Some(Token::Function(func)) = operator_stack.last() {
                                operator_stack.pop();
                                
                                if output_queue.is_empty() {
                                    return Err("Invalid function call: missing argument".to_string());
                                }
                                
                                let arg = output_queue.pop().unwrap();
                                output_queue.push(Box::new(FunctionCall::new(func.clone(), arg)));
                            }
                            
                            break;
                        } else if let Token::Operator(op) = top {
                            operator_stack.pop();
                            
                            if output_queue.len() < 2 {
                                return Err("Invalid expression: not enough operands".to_string());
                            }
                            
                            let right = output_queue.pop().unwrap();
                            let left = output_queue.pop().unwrap();
                            
                            output_queue.push(Box::new(BinaryOperation::new(left, right, op.clone())));
                        } else {
                            operator_stack.pop();
                        }
                    }
                }
            }
        }
        
        // Process remaining operators
        while let Some(token) = operator_stack.pop() {
            match token {
                Token::Operator(op) => {
                    if output_queue.len() < 2 {
                        return Err("Invalid expression: not enough operands".to_string());
                    }
                    
                    let right = output_queue.pop().unwrap();
                    let left = output_queue.pop().unwrap();
                    
                    output_queue.push(Box::new(BinaryOperation::new(left, right, op)));
                },
                Token::OpenParen | Token::CloseParen => {
                    return Err("Mismatched parentheses".to_string());
                },
                _ => {
                    return Err(format!("Unexpected token on operator stack: {:?}", token));
                }
            }
        }
        
        if output_queue.len() != 1 {
            return Err("Invalid expression: too many values".to_string());
        }
        
        Ok(output_queue.pop().unwrap())
    }
}

impl EvaluationStrategy for ShuntingYardStrategy {
    fn evaluate(&self, expression: &str, variables: &HashMap<String, f64>) -> Result<f64, String> {
        let tokens = self.tokenizer.tokenize(expression)?;
        let expr = self.build_expression_tree(tokens)?;
        expr.evaluate(variables)
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
        let result = self.evaluation_strategy.evaluate(expression, variables)?;
        Ok(self.precision_strategy.round(result))
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

// Factory functions for creating common strategies
pub fn create_standard_evaluator() -> ExpressionEvaluator {
    let tokenizer = Box::new(SimpleTokenizer);
    let evaluation_strategy = Box::new(ShuntingYardStrategy::new(tokenizer));
    let precision_strategy = Box::new(StandardPrecision::new(10));
    
    ExpressionEvaluator::new(evaluation_strategy, precision_strategy)
}

pub fn create_scientific_evaluator() -> ExpressionEvaluator {
    let tokenizer = Box::new(SimpleTokenizer);
    let evaluation_strategy = Box::new(ShuntingYardStrategy::new(tokenizer));
    let precision_strategy = Box::new(ScientificPrecision::new(6));
    
    ExpressionEvaluator::new(evaluation_strategy, precision_strategy)
}
