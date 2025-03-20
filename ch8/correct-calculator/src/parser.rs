// parser.rs - Parser for expressions

use crate::token::{Token, Operator, Function};
use crate::expression::{Expression, NumberExpression, VariableExpression, BinaryOperation, FunctionCall};

#[derive(Clone)]
pub struct ExpressionParser;

impl ExpressionParser {
    pub fn new() -> Self {
        Self
    }
    
    pub fn parse(&self, expression: &str) -> Result<Box<dyn Expression>, String> {
        // Tokenize
        let tokens = self.tokenize(expression)?;
        
        // Parse using Shunting-yard algorithm
        self.build_expression_tree(tokens)
    }
    
    fn tokenize(&self, input: &str) -> Result<Vec<Token>, String> {
        // This is a simplistic tokenizer for demonstration
        // A real tokenizer would be more sophisticated
        
        // Separate tokens by spaces for simplicity
        let tokens: Result<Vec<Token>, String> = input
            .split_whitespace()
            .map(Token::from_str)
            .collect();
        
        tokens
    }
    
    fn build_expression_tree(&self, tokens: Vec<Token>) -> Result<Box<dyn Expression>, String> {
        // Implementation of the shunting yard algorithm
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
                    let mut found_open_paren = false;
                    
                    while let Some(top) = operator_stack.pop() {
                        match top {
                            Token::OpenParen => {
                                found_open_paren = true;
                                
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
                            },
                            Token::Operator(op) => {
                                if output_queue.len() < 2 {
                                    return Err("Invalid expression: not enough operands".to_string());
                                }
                                
                                let right = output_queue.pop().unwrap();
                                let left = output_queue.pop().unwrap();
                                
                                output_queue.push(Box::new(BinaryOperation::new(left, right, op)));
                            },
                            _ => {
                                return Err(format!("Unexpected token on operator stack: {:?}", top));
                            }
                        }
                    }
                    
                    if !found_open_paren {
                        return Err("Mismatched parentheses".to_string());
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
