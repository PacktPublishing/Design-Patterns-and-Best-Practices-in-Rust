// Ch4: Unsafe Is (Usually) Not the Answer

// This file demonstrates the anti-pattern of using unsafe code to work around
// borrow checker restrictions, along with a better approach.

// UNSAFE APPROACH: Using pointers to circumvent borrowing rules

struct Expression {
    tokens: Vec<String>,
    result: f64,
}

// A fixed-size cache implemented unsafely
struct UnsafeCache {
    // Vec with initial capacity of 10
    expressions: Vec<Expression>,
}

impl UnsafeCache {
    fn new() -> Self {
        Self {
            expressions: Vec::with_capacity(10),
        }
    }

    fn cache_expression(&mut self, expr: Expression) -> *const Expression {
        self.expressions.push(expr);
        // Get pointer to the expression we just added
        self.expressions.last().unwrap() as *const Expression
    }
}

struct BadCalculator {
    cache: UnsafeCache,
    // Store pointers to $1 and $2
    recent_expr_ptr1: Option<*const Expression>,
    recent_expr_ptr2: Option<*const Expression>,
}

impl BadCalculator {
    fn new() -> Self {
        Self {
            cache: UnsafeCache::new(),
            recent_expr_ptr1: None,
            recent_expr_ptr2: None,
        }
    }

    fn evaluate(&mut self, expression: &str) -> Result<f64, String> {
        // Handle references to previous results
        if expression == "$1" {
            if let Some(ptr) = self.recent_expr_ptr1 {
                // UNSAFE: Pointer will be invalid if Vec reallocated!
                unsafe {
                    return Ok((*ptr).result);
                }
            }
            return Err("No previous expression".to_string());
        }
        if expression == "$2" {
            if let Some(ptr) = self.recent_expr_ptr2 {
                // UNSAFE: Pointer will be invalid if Vec reallocated!
                unsafe {
                    return Ok((*ptr).result);
                }
            }
            return Err("No expression before previous".to_string());
        }

        // Regular expression evaluation
        let tokens = expression.split_whitespace().collect();
        let result = self.evaluate_tokens(&tokens)?;
        
        // Cache the new expression
        let expr = Expression { tokens, result };
        let ptr = self.cache.cache_expression(expr);
        
        // Update recent expression pointers
        self.recent_expr_ptr2 = self.recent_expr_ptr1;
        self.recent_expr_ptr1 = Some(ptr);
        
        Ok(result)
    }
    
    fn evaluate_tokens(&self, tokens: &Vec<String>) -> Result<f64, String> {
        // Simple implementation for demonstration
        if tokens.len() != 3 {
            return Err("Only simple expressions supported".to_string());
        }
        
        let left = tokens[0].parse::<f64>().map_err(|_| "Invalid left operand".to_string())?;
        let right = tokens[2].parse::<f64>().map_err(|_| "Invalid right operand".to_string())?;
        
        match tokens[1].as_str() {
            "+" => Ok(left + right),
            "-" => Ok(left - right),
            "*" => Ok(left * right),
            "/" => {
                if right == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(left / right)
                }
            },
            _ => Err("Unknown operator".to_string()),
        }
    }
}

// This function demonstrates how the unsafe version can lead to undefined behavior
fn demonstrate_unsafe_issue() {
    let mut bad_calc = BadCalculator::new();
    
    // Fill the initial capacity (10 expressions)
    for i in 0..10 {
        bad_calc.evaluate(&format!("{} + {}", i, i)).unwrap();
    }
    
    // Everything has worked fine so far
    println!("Previous result: {}", bad_calc.evaluate("$1").unwrap());
    
    // Add one more expression - Vec must reallocate!
    bad_calc.evaluate("10 + 10").unwrap();
    
    println!("After reallocation, accessing $1 could cause undefined behavior!");
    // UNSAFE: This might crash or produce garbage data
    // bad_calc.evaluate("$1").unwrap();
}

// SAFE APPROACH: Using indices rather than pointers

struct SafeCalculator {
    expressions: Vec<Expression>,
}

impl SafeCalculator {
    fn new() -> Self {
        Self {
            expressions: Vec::new(),
        }
    }

    fn evaluate(&mut self, expression: &str) -> Result<f64, String> {
        if expression == "$1" {
            return self.expressions.last()
                .map(|expr| expr.result)
                .ok_or_else(|| "No previous expression".to_string());
        }
        if expression == "$2" {
            return self.expressions.iter().rev().nth(1)
                .map(|expr| expr.result)
                .ok_or_else(|| "No expression before previous".to_string());
        }

        let tokens = expression.split_whitespace().collect();
        let result = self.evaluate_tokens(&tokens)?;
        
        self.expressions.push(Expression { tokens, result });
        
        Ok(result)
    }
    
    fn evaluate_tokens(&self, tokens: &Vec<String>) -> Result<f64, String> {
        // Simple implementation for demonstration
        if tokens.len() != 3 {
            return Err("Only simple expressions supported".to_string());
        }
        
        let left = tokens[0].parse::<f64>().map_err(|_| "Invalid left operand".to_string())?;
        let right = tokens[2].parse::<f64>().map_err(|_| "Invalid right operand".to_string())?;
        
        match tokens[1].as_str() {
            "+" => Ok(left + right),
            "-" => Ok(left - right),
            "*" => Ok(left * right),
            "/" => {
                if right == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(left / right)
                }
            },
            _ => Err("Unknown operator".to_string()),
        }
    }
}

fn main() {
    println!("--- UNSAFE APPROACH: Using Raw Pointers ---");
    demonstrate_unsafe_issue();
    
    println!("\n--- SAFE APPROACH: Using Indices ---");
    let mut calc = SafeCalculator::new();
    
    match calc.evaluate("5 + 7") {
        Ok(result) => println!("5 + 7 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match calc.evaluate("10 * 3") {
        Ok(result) => println!("10 * 3 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match calc.evaluate("$1") {
        Ok(result) => println!("$1 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match calc.evaluate("$2") {
        Ok(result) => println!("$2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Add more expressions than the original capacity
    for i in 0..15 {
        calc.evaluate(&format!("{} + {}", i, i)).unwrap();
    }
    
    // This still works safely even after reallocation
    match calc.evaluate("$1") {
        Ok(result) => println!("$1 after many additions = {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
