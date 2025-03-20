// expression.rs - Expression tree implementation (from Chapter 6)

use std::collections::HashMap;
use std::any::Any;
use crate::token::{Operator, Function};

// Expression trait defining common behavior
pub trait Expression {
    fn evaluate(&self, variables: &HashMap<String, f64>) -> Result<f64, String>;
    fn to_string(&self) -> String;
    
    // For debugging and visualization
    fn precedence(&self) -> u8 {
        0 // Leaf nodes have lowest precedence by default
    }
    
    // Allow downcasting for visitor pattern
    fn as_any(&self) -> &dyn Any;
    
    // Default implementation for cloning
    fn clone_box(&self) -> Box<dyn Expression>;
}

// Extension to allow cloning of trait objects
impl Clone for Box<dyn Expression> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

// Leaf node for number values
#[derive(Debug, Clone)]
pub struct NumberExpression {
    pub value: f64,
}

impl NumberExpression {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Expression for NumberExpression {
    fn evaluate(&self, _variables: &HashMap<String, f64>) -> Result<f64, String> {
        Ok(self.value)
    }
    
    fn to_string(&self) -> String {
        format!("{}", self.value)
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

// Leaf node for variables
#[derive(Debug, Clone)]
pub struct VariableExpression {
    pub name: String,
}

impl VariableExpression {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl Expression for VariableExpression {
    fn evaluate(&self, variables: &HashMap<String, f64>) -> Result<f64, String> {
        variables
            .get(&self.name)
            .copied()
            .ok_or_else(|| format!("Undefined variable: {}", self.name))
    }
    
    fn to_string(&self) -> String {
        self.name.clone()
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

// Composite node for binary operations
#[derive(Debug, Clone)]
pub struct BinaryOperation {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub operator: Operator,
}

impl BinaryOperation {
    pub fn new(
        left: Box<dyn Expression>,
        right: Box<dyn Expression>,
        operator: Operator,
    ) -> Self {
        Self { left, right, operator }
    }
    
    fn operator_symbol(&self) -> &'static str {
        match self.operator {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            Operator::Power => "^",
        }
    }
}

impl Expression for BinaryOperation {
    fn evaluate(&self, variables: &HashMap<String, f64>) -> Result<f64, String> {
        let left_val = self.left.evaluate(variables)?;
        let right_val = self.right.evaluate(variables)?;
        
        match self.operator {
            Operator::Add => Ok(left_val + right_val),
            Operator::Subtract => Ok(left_val - right_val),
            Operator::Multiply => Ok(left_val * right_val),
            Operator::Divide => {
                if right_val == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(left_val / right_val)
                }
            },
            Operator::Power => Ok(left_val.powf(right_val)),
        }
    }
    
    fn to_string(&self) -> String {
        let left_str = if self.left.precedence() < self.precedence() {
            format!("({})", self.left.to_string())
        } else {
            self.left.to_string()
        };
        
        let right_str = if self.right.precedence() < self.precedence() {
            format!("({})", self.right.to_string())
        } else {
            self.right.to_string()
        };
        
        format!("{} {} {}", left_str, self.operator_symbol(), right_str)
    }
    
    fn precedence(&self) -> u8 {
        match self.operator {
            Operator::Add | Operator::Subtract => 1,
            Operator::Multiply | Operator::Divide => 2,
            Operator::Power => 3,
        }
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

// Function call expression
#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub function: Function,
    pub argument: Box<dyn Expression>,
}

impl FunctionCall {
    pub fn new(function: Function, argument: Box<dyn Expression>) -> Self {
        Self { function, argument }
    }
}

impl Expression for FunctionCall {
    fn evaluate(&self, variables: &HashMap<String, f64>) -> Result<f64, String> {
        let arg_val = self.argument.evaluate(variables)?;
        
        match self.function {
            Function::Sin => Ok(arg_val.sin()),
            Function::Cos => Ok(arg_val.cos()),
            Function::Tan => {
                if (arg_val - std::f64::consts::PI/2.0).abs() % std::f64::consts::PI < 1e-10 {
                    Err("Tangent undefined at this value".to_string())
                } else {
                    Ok(arg_val.tan())
                }
            },
            Function::Sqrt => {
                if arg_val < 0.0 {
                    Err("Cannot take square root of negative number".to_string())
                } else {
                    Ok(arg_val.sqrt())
                }
            },
        }
    }
    
    fn to_string(&self) -> String {
        let func_name = match self.function {
            Function::Sin => "sin",
            Function::Cos => "cos",
            Function::Tan => "tan",
            Function::Sqrt => "sqrt",
        };
        
        format!("{}({})", func_name, self.argument.to_string())
    }
    
    fn precedence(&self) -> u8 {
        4 // Function calls have highest precedence
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}
