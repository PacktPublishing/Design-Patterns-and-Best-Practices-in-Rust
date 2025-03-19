// Ch4: Trying to Defeat the Borrow Checker

// This file demonstrates the anti-pattern of fighting against the borrow checker
// in a calculator implementation, followed by a better approach.

// Define token types for our calculator
#[derive(Debug, Clone)]
enum Token {
    Number(f64),
    ResultReference(usize),
    Operator(char),
}

// BAD APPROACH: Fighting the borrow checker

struct BadCalculator {
    current_value: f64,
    memory: Vec<f64>,
}

impl BadCalculator {
    fn new() -> Self {
        Self {
            current_value: 0.0,
            memory: Vec::new(),
        }
    }

    fn evaluate(&mut self, expression: &str) -> Result<f64, String> {
        // Try to parse as a reference to previous result
        if expression.starts_with("result") {
            if let Some(index) = expression.strip_prefix("result") {
                if let Ok(offset) = index.trim().parse::<usize>() {
                    return self.get_previous_result(offset);
                }
            }
        }

        // Parse and evaluate the expression
        let result = self.parse_and_evaluate(expression)?;
        
        // Store the result and update current value
        self.memory.push(result);
        self.current_value = result;
        
        Ok(result)
    }

    fn get_previous_result(&self, index: usize) -> Result<f64, String> {
        if index == 0 {
            Ok(self.current_value)
        } else {
            let pos = self.memory.len().checked_sub(index)
                .ok_or("Invalid result index")?;
            self.memory.get(pos)
                .copied()
                .ok_or_else(|| "Invalid result index".to_string())
        }
    }

    fn tokenize(&self, expression: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        
        for part in expression.split_whitespace() {
            let token = if let Some(index) = part.strip_prefix("result") {
                if let Ok(offset) = index.trim().parse() {
                    Token::ResultReference(offset)
                } else {
                    return Err("Invalid result reference".to_string());
                }
            } else if let Ok(num) = part.parse() {
                Token::Number(num)
            } else if part.len() == 1 && "+-*/".contains(part) {
                Token::Operator(part.chars().next().unwrap())
            } else {
                return Err(format!("Invalid token: {}", part));
            };
            
            tokens.push(token);
        }
        
        Ok(tokens)
    }

    // This will not compile! Self is already mutably borrowed
    fn parse_and_evaluate(&mut self, expression: &str) -> Result<f64, String> {
        let tokens = self.tokenize(expression)?;
        
        for token in &tokens {
            if let Token::ResultReference(index) = token {
                // But we can't do this - we're already mutably borrowed!
                // let prev = self.get_previous_result(*index)?;
                
                // We'd have to use a hack like this:
                let memory = &self.memory;
                let current = self.current_value;
                let prev = if *index == 0 {
                    current
                } else {
                    let pos = memory.len().checked_sub(*index)
                        .ok_or("Invalid result index")?;
                    *memory.get(pos)
                        .ok_or_else(|| "Invalid result index".to_string())?
                };
                
                // Use prev in calculation...
                let _ = prev;
            }
        }
        
        // Simplified implementation for demonstration
        Ok(42.0)
    }
}

// BETTER APPROACH: Working with the borrow checker

struct Calculator {
    memory: Vec<f64>,
    current_value: f64,
}

impl Calculator {
    fn new() -> Self {
        Self {
            memory: Vec::new(),
            current_value: 0.0,
        }
    }

    fn tokenize(&self, expression: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        
        for part in expression.split_whitespace() {
            let token = if let Some(index) = part.strip_prefix("result") {
                if let Ok(offset) = index.trim().parse() {
                    Token::ResultReference(offset)
                } else {
                    return Err("Invalid result reference".to_string());
                }
            } else if let Ok(num) = part.parse() {
                Token::Number(num)
            } else if part.len() == 1 && "+-*/".contains(part) {
                Token::Operator(part.chars().next().unwrap())
            } else {
                return Err(format!("Invalid token: {}", part));
            };
            
            tokens.push(token);
        }
        
        Ok(tokens)
    }

    fn get_previous_result(&self, index: usize) -> Result<f64, String> {
        if index == 0 {
            Ok(self.current_value)
        } else {
            let pos = self.memory.len().checked_sub(index)
                .ok_or("Invalid result index")?;
            self.memory.get(pos)
                .copied()
                .ok_or_else(|| "Invalid result index".to_string())
        }
    }

    fn evaluate_tokens(&self, tokens: Vec<Token>) -> Result<f64, String> {
        // Simplified implementation for demonstration
        if tokens.len() != 3 {
            return Err("Only simple expressions supported".to_string());
        }

        let left = match &tokens[0] {
            Token::Number(n) => *n,
            Token::ResultReference(idx) => self.get_previous_result(*idx)?,
            _ => return Err("Expected number or result reference".to_string()),
        };

        let op = match &tokens[1] {
            Token::Operator(op) => *op,
            _ => return Err("Expected operator".to_string()),
        };

        let right = match &tokens[2] {
            Token::Number(n) => *n,
            Token::ResultReference(idx) => self.get_previous_result(*idx)?,
            _ => return Err("Expected number or result reference".to_string()),
        };

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
            _ => Err("Unknown operator".to_string()),
        }
    }

    fn evaluate(&mut self, expression: &str) -> Result<f64, String> {
        // First get the tokens
        let tokens = self.tokenize(expression)?;
        
        // Evaluate with all values resolved
        let result = self.evaluate_tokens(tokens)?;
        
        // Store the result
        self.memory.push(result);
        self.current_value = result;
        
        Ok(result)
    }
}

fn main() {
    // Note: The BadCalculator code won't compile as written
    // This is intentional to demonstrate the problem
    println!("In a real program, the bad approach would not compile.");
    println!("This example shows how to structure code to work with the borrow checker.\n");
    
    println!("--- BETTER APPROACH: Working with the Borrow Checker ---");
    let mut calc = Calculator::new();
    
    match calc.evaluate("5 + 7") {
        Ok(result) => println!("5 + 7 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match calc.evaluate("result0 * 2") {
        Ok(result) => println!("result0 * 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match calc.evaluate("result1 - result0") {
        Ok(result) => println!("result1 - result0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
