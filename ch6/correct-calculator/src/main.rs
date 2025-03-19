// Correct Calculator - Chapter 6
// Demonstrates structural design patterns

// Import modules from Chapter 5
mod token;
mod factory;
mod config;
mod builder;

// Import new modules for Chapter 6
mod expression;
mod decorator;
mod adapter;
mod facade;
mod bridge;

use std::collections::HashMap;
use token::{Token, Operator, Function};
use expression::{Expression, NumberExpression, VariableExpression, BinaryOperation, FunctionCall};
use decorator::{ConsoleLogger, LoggingExpression, TimingExpression};
use adapter::{StandardScientificOperations, ExternalLibraryAdapter};
use facade::CalculatorFacade;
use bridge::{CalculatorDisplay, ConsoleDisplay, HtmlDisplay, JsonDisplay,
             StandardEvaluator, OptimizingEvaluator, Evaluator};
use config::{CalculatorConfig, AngleMode};

fn main() {
    println!("Correct Calculator - Chapter 6 - Structural Patterns\n");

    // Demonstrate Composite Pattern
    println!("\n== Composite Pattern ==");
    // Build an expression tree for: 2 + 3 * 4
    let multiply = Box::new(BinaryOperation {
        left: Box::new(NumberExpression { value: 3.0 }),
        right: Box::new(NumberExpression { value: 4.0 }),
        operator: Operator::Multiply,
    });
    
    let add = Box::new(BinaryOperation {
        left: Box::new(NumberExpression { value: 2.0 }),
        right: multiply,
        operator: Operator::Add,
    });
    
    // Evaluate the expression
    let variables = HashMap::new();
    println!("Expression: {}", add.to_string());
    match add.evaluate(&variables) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // More complex expression with a function call
    let sin_expr = Box::new(FunctionCall {
        function: Function::Sin,
        argument: Box::new(VariableExpression { name: "x".to_string() }),
    });
    
    let mut var_map = HashMap::new();
    var_map.insert("x".to_string(), std::f64::consts::PI);
    
    println!("\nExpression: {}", sin_expr.to_string());
    match sin_expr.evaluate(&var_map) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Demonstrate Decorator Pattern
    println!("\n== Decorator Pattern ==");
    
    // Create a logging decorated expression
    let logging_expr = LoggingExpression::new(
        add.clone(),
        Box::new(ConsoleLogger),
    );
    
    println!("Evaluating with logging:");
    match logging_expr.evaluate(&variables) {
        Ok(result) => println!("Final result: {}", result),
        Err(e) => println!("Final error: {}", e),
    }
    
    // Create a timing decorated expression
    let timing_expr = TimingExpression::new(
        add.clone(),
    );
    
    println!("\nEvaluating with timing:");
    match timing_expr.evaluate(&variables) {
        Ok(result) => println!("Final result: {}", result),
        Err(e) => println!("Final error: {}", e),
    }
    
    // Demonstrate Adapter Pattern
    println!("\n== Adapter Pattern ==");
    
    // Create a standard scientific operations adapter
    let standard_ops = StandardScientificOperations {
        angle_mode: AngleMode::Radians,
    };
    
    // Create an external library adapter
    let external_ops = ExternalLibraryAdapter::new(AngleMode::Degrees);
    
    // Use both adapters
    println!("Standard sin(π/2): {}", standard_ops.sin(std::f64::consts::PI / 2.0));
    println!("External sin(90°): {}", external_ops.sin(90.0));
    
    // Demonstrate Facade Pattern
    println!("\n== Facade Pattern ==");
    
    // Create a calculator facade
    let mut calculator = CalculatorFacade::new(
        Box::new(standard_ops),
        CalculatorConfig::default(),
    );
    
    // Use the simplified interface
    println!("Using calculator facade:");
    match calculator.evaluate("2 + 3 * 4") {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Use specialized methods
    calculator.set_variable("a", 1.0);
    calculator.set_variable("b", -5.0);
    calculator.set_variable("c", 6.0);
    
    match calculator.calculate_quadratic(1.0, -5.0, 6.0) {
        Ok((x1, x2)) => println!("Quadratic roots: {} and {}", x1, x2),
        Err(e) => println!("Error: {}", e),
    }
    
    // Demonstrate Bridge Pattern
    println!("\n== Bridge Pattern ==");
    
    // Create different implementations
    let console_impl = Box::new(ConsoleDisplay);
    let html_impl = Box::new(HtmlDisplay);
    let json_impl = Box::new(JsonDisplay);
    
    // Create displays with different implementations
    let console_display = CalculatorDisplay::new(console_impl);
    let html_display = CalculatorDisplay::new(html_impl);
    let json_display = CalculatorDisplay::new(json_impl);
    
    // Use the displays
    println!("\nConsole display:");
    console_display.show_result(14.0);
    console_display.show_error("Sample error");
    console_display.show_expression(&*add);
    
    println!("\nHTML display:");
    html_display.show_result(14.0);
    html_display.show_error("Sample error");
    html_display.show_expression(&*add);
    
    println!("\nJSON display:");
    json_display.show_result(14.0);
    json_display.show_error("Sample error");
    json_display.show_expression(&*add);
    
    // Demonstrate the evaluation bridge
    println!("\n== Evaluation Bridge ==");
    
    // Create evaluation strategies
    let standard_eval = Box::new(StandardEvaluator);
    let optimizing_eval = Box::new(OptimizingEvaluator::new());
    
    // Create the evaluator with standard strategy
    let mut evaluator = Evaluator::new(standard_eval);
    
    // Use the evaluator
    println!("Standard evaluation:");
    match evaluator.evaluate(&*add, &variables) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Change the strategy
    evaluator.change_strategy(optimizing_eval);
    
    // Use the evaluator with the new strategy
    println!("\nOptimizing evaluation:");
    match evaluator.evaluate(&*add, &variables) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Evaluate again to demonstrate caching
    println!("\nOptimizing evaluation (second call, should be cached):");
    match evaluator.evaluate(&*add, &variables) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    println!("\nAll structural patterns have been demonstrated!");
}
