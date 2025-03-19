// Ch3: Cloning Everything

// This file demonstrates the anti-pattern of excessive cloning
// in a calculator implementation, followed by a better approach.

use std::collections::HashMap;

// BAD APPROACH: Cloning everything

#[derive(Clone, Debug)]  // The temptation begins...
struct Variable {
    name: String,
    value: f64,
}

#[derive(Clone, Debug)]  // And continues...
enum Token {
    Number(f64),
    Variable(Variable),
    Operator(char),
}

struct BadCalculator {
    variables: HashMap<String, Variable>,
}

impl BadCalculator {
    fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
    
    fn set_variable(&mut self, name: String, value: f64) {
        self.variables.insert(name.clone(), Variable { name, value });
    }

    fn tokenize(&self, expression: &str) -> Vec<Token> {
        let mut tokens = Vec::new();

        for part in expression.split_whitespace() {
            if let Ok(num) = part.parse::<f64>() {
                tokens.push(Token::Number(num));
            } else if let Some(var) = self.variables.get(part) {
                // Just clone it!
                tokens.push(Token::Variable(var.clone()));
            } else if part.len() == 1 && "+-*/".contains(part) {
                tokens.push(Token::Operator(part.chars().next().unwrap()));
            }
        }

        tokens
    }

    fn evaluate(&self, expression: &str) -> Result<f64, String> {
        let tokens = self.tokenize(expression);
        let result = self.evaluate_tokens(tokens)?;
        Ok(result)
    }
    
    fn evaluate_tokens(&self, tokens: Vec<Token>) -> Result<f64, String> {
        let mut working_tokens = tokens.clone();  // Clone the whole vector!

        while working_tokens.len() > 1 {
            // Find next operator
            let op_pos = working_tokens.iter().position(|t| {
                matches!(t, Token::Operator(_))
            }).ok_or("No operator found")?;

            // Get operands (more cloning!)
            let left = working_tokens[op_pos - 1].clone();
            let right = working_tokens[op_pos + 1].clone();

            // Calculate result
            let result = self.apply_operator(left, right)?;

            // Remove old tokens and insert result
            working_tokens.drain(op_pos - 1..=op_pos + 1);
            working_tokens.insert(op_pos - 1, Token::Number(result));
        }

        match &working_tokens[0] {
            Token::Number(n) => Ok(*n),
            _ => Err("Invalid expression result".to_string())
        }
    }
    
    fn apply_operator(&self, left: Token, right: Token) -> Result<f64, String> {
        let left_val = match left {
            Token::Number(n) => n,
            Token::Variable(var) => var.value,
            _ => return Err("Expected number or variable".to_string()),
        };
        
        let right_val = match right {
            Token::Number(n) => n,
            Token::Variable(var) => var.value,
            _ => return Err("Expected number or variable".to_string()),
        };
        
        match working_tokens[op_pos] {
            Token::Operator('+') => Ok(left_val + right_val),
            Token::Operator('-') => Ok(left_val - right_val),
            Token::Operator('*') => Ok(left_val * right_val),
            Token::Operator('/') => {
                if right_val == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(left_val / right_val)
                }
            },
            _ => Err("Invalid operator".to_string()),
        }
    }
}

// BETTER APPROACH: Using references and lifetimes

#[derive(Debug)]
struct BetterVariable {
    name: String,
    value: f64,
}

#[derive(Debug)]
enum BetterToken<'a> {
    Number(f64),
    Variable(&'a BetterVariable),
    Operator(char),
}

struct BetterCalculator {
    variables: HashMap<String, BetterVariable>,
}

impl BetterCalculator {
    fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
    
    fn set_variable(&mut self, name: String, value: f64) {
        self.variables.insert(name.clone(), BetterVariable { name, value });
    }

    fn tokenize<'a>(&'a self, expression: &str) -> Vec<BetterToken<'a>> {
        let mut tokens = Vec::new();

        for part in expression.split_whitespace() {
            if let Ok(num) = part.parse::<f64>() {
                tokens.push(BetterToken::Number(num));
            } else if let Some(var) = self.variables.get(part) {
                // Just use a reference - no cloning needed
                tokens.push(BetterToken::Variable(var));
            } else if part.len() == 1 && "+-*/".contains(part) {
                tokens.push(BetterToken::Operator(part.chars().next().unwrap()));
            }
        }

        tokens
    }

    fn evaluate(&self, expression: &str) -> Result<f64, String> {
        let tokens = self.tokenize(expression);
        self.evaluate_tokens(tokens)
    }
    
    fn evaluate_tokens(&self, mut tokens: Vec<BetterToken>) -> Result<f64, String> {
        while tokens.len() > 1 {
            // Find next operator
            let op_pos = tokens.iter().position(|t| {
                matches!(t, BetterToken::Operator(_))
            }).ok_or("No operator found")?;
            
            // Calculate using references
            let result = match (&tokens[op_pos-1], &tokens[op_pos], &tokens[op_pos+1]) {
                (BetterToken::Number(n1), BetterToken::Operator(op), BetterToken::Number(n2)) => {
                    self.apply_operation(*op, *n1, *n2)?
                },
                (BetterToken::Variable(v1), BetterToken::Operator(op), BetterToken::Number(n2)) => {
                    self.apply_operation(*op, v1.value, *n2)?
                },
                (BetterToken::Number(n1), BetterToken::Operator(op), BetterToken::Variable(v2)) => {
                    self.apply_operation(*op, *n1, v2.value)?
                },
                (BetterToken::Variable(v1), BetterToken::Operator(op), BetterToken::Variable(v2)) => {
                    self.apply_operation(*op, v1.value, v2.value)?
                },
                _ => return Err("Invalid expression syntax".to_string()),
            };

            // Remove old tokens and insert result
            tokens.drain(op_pos - 1..=op_pos + 1);
            tokens.insert(op_pos - 1, BetterToken::Number(result));
        }

        match &tokens[0] {
            BetterToken::Number(n) => Ok(*n),
            _ => Err("Invalid expression result".to_string())
        }
    }
    
    fn apply_operation(&self, op: char, left: f64, right: f64) -> Result<f64, String> {
        match op {
            '+' => Ok(left + right),
            '-' => Ok(left - right),
            '*' => Ok(left * right),
            '/' => {
                if right == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(left / right)
                }
            },
            _ => Err("Invalid operator".to_string()),
        }
    }
}

fn main() {
    println!("--- BAD APPROACH: Cloning Everything ---");
    let mut bad_calc = BadCalculator::new();
    bad_calc.set_variable("x".to_string(), 10.0);
    bad_calc.set_variable("y".to_string(), 5.0);
    
    match bad_calc.evaluate("x + y") {
        Ok(result) => println!("x + y = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    println!("\n--- BETTER APPROACH: Using References ---");
    let mut calc = BetterCalculator::new();
    calc.set_variable("x".to_string(), 10.0);
    calc.set_variable("y".to_string(), 5.0);
    
    match calc.evaluate("x + y") {
        Ok(result) => println!("x + y = {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
