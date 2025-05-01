// iterator.rs - Iterator pattern implementation for collections in the calculator

use std::collections::HashMap;
use crate::command::Calculation;
use crate::expression::{Expression, BinaryOperation, NumberExpression, VariableExpression, FunctionCall};

// History iterator that provides access to past results
pub struct HistoryIterator<'a> {
    history: &'a [Calculation],
    position: usize,
}

impl<'a> HistoryIterator<'a> {
    pub fn new(history: &'a [Calculation]) -> Self {
        Self {
            history,
            position: 0,
        }
    }
}

impl<'a> Iterator for HistoryIterator<'a> {
    type Item = &'a Calculation;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.history.len() {
            let item = &self.history[self.position];
            self.position += 1;
            Some(item)
        } else {
            None
        }
    }
}

// A reverse iterator for the history
pub struct ReverseHistoryIterator<'a> {
    history: &'a [Calculation],
    position: usize,
}

impl<'a> ReverseHistoryIterator<'a> {
    pub fn new(history: &'a [Calculation]) -> Self {
        Self {
            history,
            position: history.len(),
        }
    }
}

impl<'a> Iterator for ReverseHistoryIterator<'a> {
    type Item = &'a Calculation;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.position > 0 {
            self.position -= 1;
            Some(&self.history[self.position])
        } else {
            None
        }
    }
}

// Extension trait for expression tree traversal
pub trait ExpressionExt {
    fn as_binary_op(&self) -> Option<&BinaryOperation> { None }
    fn as_number(&self) -> Option<&NumberExpression> { None }
    fn as_variable(&self) -> Option<&VariableExpression> { None }
    fn as_function(&self) -> Option<&FunctionCall> { None }
    fn is_constant(&self) -> bool { self.as_number().is_some() }
}

impl ExpressionExt for dyn Expression {
    fn as_binary_op(&self) -> Option<&BinaryOperation> { None }
    fn as_number(&self) -> Option<&NumberExpression> { None }
    fn as_variable(&self) -> Option<&VariableExpression> { None }
    fn as_function(&self) -> Option<&FunctionCall> { None }
}

impl ExpressionExt for BinaryOperation {
    fn as_binary_op(&self) -> Option<&BinaryOperation> { Some(self) }
}

impl ExpressionExt for NumberExpression {
    fn as_number(&self) -> Option<&NumberExpression> { Some(self) }
    fn is_constant(&self) -> bool { true }
}

impl ExpressionExt for VariableExpression {
    fn as_variable(&self) -> Option<&VariableExpression> { Some(self) }
}

impl ExpressionExt for FunctionCall {
    fn as_function(&self) -> Option<&FunctionCall> { Some(self) }
}

// Variables map iterator
pub struct VariablesIterator<'a> {
    inner: std::collections::hash_map::Iter<'a, String, f64>,
}

impl<'a> VariablesIterator<'a> {
    pub fn new(variables: &'a HashMap<String, f64>) -> Self {
        Self {
            inner: variables.iter(),
        }
    }
}

impl<'a> Iterator for VariablesIterator<'a> {
    type Item = (&'a String, &'a f64);
    
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

// Non-recursive approach to collecting expressions
pub fn find_constant_nodes(expr: &dyn Expression) -> Vec<Box<dyn Expression>> {
    let mut result = Vec::new();
    collect_nodes_by_type(expr, NodeType::Constant, &mut result);
    result
}

pub fn find_variable_nodes(expr: &dyn Expression) -> Vec<Box<dyn Expression>> {
    let mut result = Vec::new();
    collect_nodes_by_type(expr, NodeType::Variable, &mut result);
    result
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum NodeType {
    Constant,
    Variable,
}

// Helper function to collect nodes by type without using an iterator
fn collect_nodes_by_type(expr: &dyn Expression, node_type: NodeType, result: &mut Vec<Box<dyn Expression>>) {
    if let Some(op) = expr.as_any().downcast_ref::<BinaryOperation>() {
        // Check if the node matches the criteria
        match node_type {
            NodeType::Constant => {
                if op.is_constant() {
                    result.push(op.clone_box());
                }
            },
            NodeType::Variable => {
                if let Some(_) = op.as_variable() {
                    result.push(op.clone_box());
                }
            },
        }
        
        // Process children recursively
        collect_nodes_by_type(&*op.left, node_type, result);
        collect_nodes_by_type(&*op.right, node_type, result);
    } else if let Some(func) = expr.as_any().downcast_ref::<FunctionCall>() {
        // Check if the node matches the criteria
        match node_type {
            NodeType::Constant => {
                if func.is_constant() {
                    result.push(func.clone_box());
                }
            },
            NodeType::Variable => {
                if let Some(_) = func.as_variable() {
                    result.push(func.clone_box());
                }
            },
        }
        
        // Process argument recursively
        collect_nodes_by_type(&*func.argument, node_type, result);
    } else if let Some(num) = expr.as_any().downcast_ref::<NumberExpression>() {
        if node_type == NodeType::Constant {
            result.push(num.clone_box());
        }
    } else if let Some(var) = expr.as_any().downcast_ref::<VariableExpression>() {
        if node_type == NodeType::Variable {
            result.push(var.clone_box());
        }
    }
}