// Correct Calculator - Chapter 5
// Main entry point demonstrating the calculator's features

mod token;
mod factory;
mod builder;
mod config;

use token::{Token, Operator, Function, NumberFormat};
use factory::{TokenFactory, StandardFactory, ScientificFactory};
use builder::ExpressionBuilder;
use config::CalculatorConfig;

fn main() {
    // Demonstrate Factory Methods
    let num_token = Token::number(42.0);
    let op_token = Token::operator(Operator::Add);
    let func_token = Token::function(Function::Sin);
    let var_token = Token::variable("x");
    
    println!("Created tokens: {:?}, {:?}, {:?}, {:?}", 
             num_token, op_token, func_token, var_token);
    
    // Demonstrate Factory from string
    match Token::from_str("3.14") {
        Ok(token) => println!("Parsed number: {:?}", token),
        Err(e) => println!("Error: {}", e),
    }
    
    // Demonstrate Abstract Factory
    let standard_factory = StandardFactory;
    let sci_factory = ScientificFactory;
    
    let standard_num = standard_factory.create_number("123").unwrap();
    let sci_num = sci_factory.create_number("1.23e-4").unwrap();
    
    println!("Standard number: {}", standard_num.format());
    println!("Scientific number: {}", sci_num.format());
    
    // Demonstrate Builder pattern
    let expr = ExpressionBuilder::new()
        .number(2.0)
        .operator(Operator::Add)
        .open_paren()
        .number(3.0)
        .operator(Operator::Multiply)
        .number(4.0)
        .close_paren()
        .build()
        .unwrap();
    
    println!("Built expression: {:?}", expr);
    
    // Demonstrate configuration (alternative to Singleton)
    let default_config = CalculatorConfig::default();
    let sci_config = CalculatorConfig::scientific();
    
    println!("Default config: {:?}", default_config);
    println!("Scientific config: {:?}", sci_config);
}
