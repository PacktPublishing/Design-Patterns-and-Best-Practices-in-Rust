// builder.rs - Builder pattern implementation

use crate::token::{Token, Operator};

#[derive(Debug, Clone)]
pub struct Expression {
    tokens: Vec<Token>,
}

pub struct ExpressionBuilder {
    tokens: Vec<Token>,
    paren_count: i32,  // Track parentheses balance
}

impl ExpressionBuilder {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            paren_count: 0,
        }
    }

    // Add a number to the expression
    pub fn number(mut self, value: f64) -> Self {
        self.tokens.push(Token::number(value));
        self
    }

    // Add an operator
    pub fn operator(mut self, op: Operator) -> Self {
        self.tokens.push(Token::operator(op));
        self
    }

    // Add a variable
    pub fn variable(mut self, name: impl Into<String>) -> Self {
        self.tokens.push(Token::variable(name));
        self
    }

    // Open a parenthesis group
    pub fn open_paren(mut self) -> Self {
        self.tokens.push(Token::OpenParen);
        self.paren_count += 1;
        self
    }

    // Close a parenthesis group
    pub fn close_paren(mut self) -> Result<Self, String> {
        if self.paren_count <= 0 {
            return Err("Unmatched closing parenthesis".to_string());
        }
        self.tokens.push(Token::CloseParen);
        self.paren_count -= 1;
        Ok(self)
    }

    // Build the final expression
    pub fn build(self) -> Result<Expression, String> {
        if self.paren_count != 0 {
            return Err("Unmatched parentheses".to_string());
        }
        
        if self.tokens.is_empty() {
            return Err("Empty expression".to_string());
        }
        
        // Validate the expression structure
        self.validate_expression()?;

        Ok(Expression {
            tokens: self.tokens,
        })
    }

    fn validate_expression(&self) -> Result<(), String> {
        // This is a simplistic validation - in a real calculator
        // this would be much more thorough
        
        if self.tokens.is_empty() {
            return Err("Expression cannot be empty".to_string());
        }
        
        // Make sure we don't have consecutive operators
        let mut prev_is_op = false;
        
        for token in &self.tokens {
            match token {
                Token::Operator(_) => {
                    if prev_is_op {
                        return Err("Consecutive operators not allowed".to_string());
                    }
                    prev_is_op = true;
                }
                _ => prev_is_op = false,
            }
        }
        
        Ok(())
    }
}

// Additional builder methods for common expression patterns
impl ExpressionBuilder {
    // Binary operation (like "2 + 3")
    pub fn binary_op(
        self,
        left: f64,
        op: Operator,
        right: f64
    ) -> Self {
        self
            .number(left)
            .operator(op)
            .number(right)
    }
    
    // Function application (like "sin(x)")
    pub fn function_call(
        self,
        func: crate::token::Function,
        arg: impl Into<String>
    ) -> Self {
        self
            .function(func)
            .open_paren()
            .variable(arg)
            .close_paren()
            .unwrap() // Safe because we're matching parens
    }
    
    fn function(mut self, func: crate::token::Function) -> Self {
        self.tokens.push(Token::function(func));
        self
    }
}

// Template methods for common expressions
impl Expression {
    pub fn quadratic() -> ExpressionBuilder {
        ExpressionBuilder::new()
            .number(1.0) // Default a coefficient
            .operator(Operator::Multiply)
            .variable("x")
            .operator(Operator::Power)
            .number(2.0)
            .operator(Operator::Add)
            .number(0.0) // Default b coefficient
            .operator(Operator::Multiply)
            .variable("x")
            .operator(Operator::Add)
            .number(0.0) // Default c coefficient
    }
}
