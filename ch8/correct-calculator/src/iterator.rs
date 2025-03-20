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

// Iterator for traversing expression trees (depth-first)
pub struct ExpressionIterator<'a> {
    stack: Vec<&'a dyn Expression>,
}

impl<'a> ExpressionIterator<'a> {
    pub fn new(root: &'a dyn Expression) -> Self {
        let mut stack = Vec::new();
        stack.push(root);
        Self { stack }
    }
}

impl<'a> Iterator for ExpressionIterator<'a> {
    type Item = &'a dyn Expression;
    
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            // Push children onto stack for depth-first traversal
            if let Some(op) = node.as_binary_op() {
                self.stack.push(&*op.right);
                self.stack.push(&*op.left);
            } else if let Some(func) = node.as_function() {
                self.stack.push(&*func.argument);
            }
            Some(node)
        } else {
            None
        }
    }
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

// Helper function to collect constants from an expression
pub fn find_constant_nodes<'a>(expr: &'a dyn Expression) -> Vec<&'a dyn Expression> {
    ExpressionIterator::new(expr)
        .filter(|node| node.is_constant())
        .collect()
}

// Helper function to collect variable nodes from an expression
pub fn find_variable_nodes<'a>(expr: &'a dyn Expression) -> Vec<&'a dyn Expression> {
    ExpressionIterator::new(expr)
        .filter(|node| node.as_variable().is_some())
        .collect()
}
