// visitor.rs - Visitor pattern implementation for traversing and transforming expressions

use std::collections::HashMap;
use std::any::Any;
use crate::expression::{Expression, NumberExpression, VariableExpression, BinaryOperation, FunctionCall};
use crate::token::{Operator, Function};

// Visitable interface for expressions
pub trait Visitable: Send + Sync {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) -> Result<(), String>;
    
    // Allow downcasting from trait object
    fn as_any(&self) -> &dyn Any;
}

// Visitor interface for expression operations
pub trait ExpressionVisitor {
    fn visit_number(&mut self, expr: &NumberExpression) -> Result<(), String>;
    fn visit_variable(&mut self, expr: &VariableExpression) -> Result<(), String>;
    fn visit_binary_op(&mut self, expr: &BinaryOperation) -> Result<(), String>;
    fn visit_function_call(&mut self, expr: &FunctionCall) -> Result<(), String>;
}

// Implementation of Visitable for each expression type
impl Visitable for NumberExpression {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) -> Result<(), String> {
        visitor.visit_number(self)
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Visitable for VariableExpression {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) -> Result<(), String> {
        visitor.visit_variable(self)
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Visitable for BinaryOperation {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) -> Result<(), String> {
        // First visit this node
        visitor.visit_binary_op(self)
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Visitable for FunctionCall {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) -> Result<(), String> {
        // Visit this node
        visitor.visit_function_call(self)
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Concrete visitor for optimizing expressions
pub struct OptimizationVisitor {
    variables: HashMap<String, f64>,
    pub optimized_expression: Option<Box<dyn Expression>>,
}

impl OptimizationVisitor {
    pub fn new(variables: HashMap<String, f64>) -> Self {
        Self {
            variables,
            optimized_expression: None,
        }
    }
    
    pub fn optimize(&mut self, expr: &dyn Expression) -> Result<Box<dyn Expression>, String> {
        if let Some(num) = expr.as_any().downcast_ref::<NumberExpression>() {
            self.visit_number(num)?;
        } else if let Some(var) = expr.as_any().downcast_ref::<VariableExpression>() {
            self.visit_variable(var)?;
        } else if let Some(op) = expr.as_any().downcast_ref::<BinaryOperation>() {
            self.visit_binary_op(op)?;
        } else if let Some(func) = expr.as_any().downcast_ref::<FunctionCall>() {
            self.visit_function_call(func)?;
        } else {
            return Ok(expr.clone_box());
        }
        
        match &self.optimized_expression {
            Some(optimized) => Ok(optimized.clone_box()),
            None => Ok(expr.clone_box()),
        }
    }
    
    fn get_constant_value(&self, expr: &dyn Expression) -> Option<f64> {
        if let Some(num_expr) = expr.as_any().downcast_ref::<NumberExpression>() {
            Some(num_expr.value)
        } else {
            None
        }
    }
    
    fn optimize_subexpression(&mut self, expr: &dyn Expression) -> Result<Box<dyn Expression>, String> {
        let saved = self.optimized_expression.take();
        let result = self.optimize(expr)?;
        self.optimized_expression = saved;
        Ok(result)
    }
}

impl ExpressionVisitor for OptimizationVisitor {
    fn visit_number(&mut self, expr: &NumberExpression) -> Result<(), String> {
        // Numbers are already optimized
        self.optimized_expression = Some(Box::new(expr.clone()));
        Ok(())
    }
    
    fn visit_variable(&mut self, expr: &VariableExpression) -> Result<(), String> {
        // If the variable has a known constant value, replace with a number
        if let Some(value) = self.variables.get(&expr.name) {
            self.optimized_expression = Some(Box::new(NumberExpression::new(*value)));
        } else {
            self.optimized_expression = Some(Box::new(expr.clone()));
        }
        Ok(())
    }
    
    fn visit_binary_op(&mut self, expr: &BinaryOperation) -> Result<(), String> {
        // Optimize left and right subexpressions
        let left_opt = self.optimize_subexpression(&*expr.left)?;
        let right_opt = self.optimize_subexpression(&*expr.right)?;
        
        // If both operands are constants, evaluate them
        if let (Some(left_val), Some(right_val)) = (
            self.get_constant_value(&*left_opt), 
            self.get_constant_value(&*right_opt)
        ) {
            let result = match expr.operator {
                Operator::Add => left_val + right_val,
                Operator::Subtract => left_val - right_val,
                Operator::Multiply => left_val * right_val,
                Operator::Divide => {
                    if right_val == 0.0 {
                        return Err("Division by zero in optimization".to_string());
                    }
                    left_val / right_val
                },
                Operator::Power => left_val.powf(right_val),
            };
            
            self.optimized_expression = Some(Box::new(NumberExpression::new(result)));
        } else {
            // Some special cases for further optimization
            match expr.operator {
                Operator::Multiply => {
                    // Multiply by 0 = 0
                    if let Some(0.0) = self.get_constant_value(&*left_opt) {
                        self.optimized_expression = Some(Box::new(NumberExpression::new(0.0)));
                        return Ok(());
                    }
                    if let Some(0.0) = self.get_constant_value(&*right_opt) {
                        self.optimized_expression = Some(Box::new(NumberExpression::new(0.0)));
                        return Ok(());
                    }
                    
                    // Multiply by 1 = other operand
                    if let Some(1.0) = self.get_constant_value(&*left_opt) {
                        self.optimized_expression = Some(right_opt);
                        return Ok(());
                    }
                    if let Some(1.0) = self.get_constant_value(&*right_opt) {
                        self.optimized_expression = Some(left_opt);
                        return Ok(());
                    }
                },
                Operator::Add => {
                    // Add 0 = other operand
                    if let Some(0.0) = self.get_constant_value(&*left_opt) {
                        self.optimized_expression = Some(right_opt);
                        return Ok(());
                    }
                    if let Some(0.0) = self.get_constant_value(&*right_opt) {
                        self.optimized_expression = Some(left_opt);
                        return Ok(());
                    }
                },
                Operator::Subtract => {
                    // Subtract 0 = left operand
                    if let Some(0.0) = self.get_constant_value(&*right_opt) {
                        self.optimized_expression = Some(left_opt);
                        return Ok(());
                    }
                },
                Operator::Divide => {
                    // Divide by 1 = left operand
                    if let Some(1.0) = self.get_constant_value(&*right_opt) {
                        self.optimized_expression = Some(left_opt);
                        return Ok(());
                    }
                    // Divide 0 by anything = 0
                    if let Some(0.0) = self.get_constant_value(&*left_opt) {
                        self.optimized_expression = Some(Box::new(NumberExpression::new(0.0)));
                        return Ok(());
                    }
                },
                Operator::Power => {
                    // Anything^0 = 1
                    if let Some(0.0) = self.get_constant_value(&*right_opt) {
                        self.optimized_expression = Some(Box::new(NumberExpression::new(1.0)));
                        return Ok(());
                    }
                    // Anything^1 = itself
                    if let Some(1.0) = self.get_constant_value(&*right_opt) {
                        self.optimized_expression = Some(left_opt);
                        return Ok(());
                    }
                    // 1^anything = 1
                    if let Some(1.0) = self.get_constant_value(&*left_opt) {
                        self.optimized_expression = Some(Box::new(NumberExpression::new(1.0)));
                        return Ok(());
                    }
                },
            }
            
            // Cannot fully optimize, create a new operation with optimized operands
            self.optimized_expression = Some(Box::new(BinaryOperation::new(
                left_opt,
                right_opt,
                expr.operator.clone(),
            )));
        }
        
        Ok(())
    }
    
    fn visit_function_call(&mut self, expr: &FunctionCall) -> Result<(), String> {
        // Optimize the argument
        let arg_opt = self.optimize_subexpression(&*expr.argument)?;
        
        // If the argument is a constant, evaluate the function
        if let Some(arg_val) = self.get_constant_value(&*arg_opt) {
            let result = match expr.function {
                Function::Sin => arg_val.sin(),
                Function::Cos => arg_val.cos(),
                Function::Tan => {
                    if (arg_val - std::f64::consts::PI/2.0).abs() % std::f64::consts::PI < 1e-10 {
                        return Err("Tangent undefined at this value".to_string());
                    }
                    arg_val.tan()
                },
                Function::Sqrt => {
                    if arg_val < 0.0 {
                        return Err("Cannot take square root of negative number".to_string());
                    }
                    arg_val.sqrt()
                },
            };
            
            self.optimized_expression = Some(Box::new(NumberExpression::new(result)));
        } else {
            // Cannot optimize, create a new function call with optimized argument
            self.optimized_expression = Some(Box::new(FunctionCall::new(
                expr.function.clone(),
                arg_opt,
            )));
        }
        
        Ok(())
    }
}

// Concrete visitor for validating expressions
pub struct ValidationVisitor {
    pub errors: Vec<String>,
}

impl ValidationVisitor {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
        }
    }
    
    pub fn validate(&mut self, expr: &dyn Expression) -> Result<(), String> {
        if let Some(num) = expr.as_any().downcast_ref::<NumberExpression>() {
            self.visit_number(num)?;
        } else if let Some(var) = expr.as_any().downcast_ref::<VariableExpression>() {
            self.visit_variable(var)?;
        } else if let Some(op) = expr.as_any().downcast_ref::<BinaryOperation>() {
            self.visit_binary_op(op)?;
            
            // Validate operands
            self.validate(&*op.left)?;
            self.validate(&*op.right)?;
        } else if let Some(func) = expr.as_any().downcast_ref::<FunctionCall>() {
            self.visit_function_call(func)?;
            
            // Validate argument
            self.validate(&*func.argument)?;
        }
        
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.join("; "))
        }
    }
}

impl ExpressionVisitor for ValidationVisitor {
    fn visit_number(&mut self, _expr: &NumberExpression) -> Result<(), String> {
        // Numbers are always valid
        Ok(())
    }
    
    fn visit_variable(&mut self, _expr: &VariableExpression) -> Result<(), String> {
        // Variables are assumed to be valid (could add name validation here)
        Ok(())
    }
    
    fn visit_binary_op(&mut self, expr: &BinaryOperation) -> Result<(), String> {
        // Check for division by zero in constant expressions
        if let Operator::Divide = expr.operator {
            if let Some(right) = expr.right.as_any().downcast_ref::<NumberExpression>() {
                if right.value == 0.0 {
                    self.errors.push("Division by zero".to_string());
                }
            }
        }
        
        Ok(())
    }
    
    fn visit_function_call(&mut self, expr: &FunctionCall) -> Result<(), String> {
        // Validate function arguments
        match expr.function {
            Function::Sqrt => {
                if let Some(arg) = expr.argument.as_any().downcast_ref::<NumberExpression>() {
                    if arg.value < 0.0 {
                        self.errors.push("Cannot take square root of negative number".to_string());
                    }
                }
            },
            Function::Tan => {
                if let Some(arg) = expr.argument.as_any().downcast_ref::<NumberExpression>() {
                    let value = arg.value;
                    if (value - std::f64::consts::PI/2.0).abs() % std::f64::consts::PI < 1e-10 {
                        self.errors.push("Tangent undefined at this value".to_string());
                    }
                }
            },
            _ => {}
        }
        
        Ok(())
    }
}

// Function to optimize an expression
pub fn optimize_expression(expr: &dyn Expression, variables: &HashMap<String, f64>) -> Result<Box<dyn Expression>, String> {
    let mut visitor = OptimizationVisitor::new(variables.clone());
    visitor.optimize(expr)
}

// Function to validate an expression
pub fn validate_expression(expr: &dyn Expression) -> Result<(), String> {
    let mut visitor = ValidationVisitor::new();
    visitor.validate(expr)
}