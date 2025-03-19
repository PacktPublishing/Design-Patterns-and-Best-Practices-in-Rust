// factory.rs - Abstract Factory implementation

use crate::token::{Token, Number, Operator, Function, NumberFormat};

// Trait for number tokens
pub trait NumberToken {
    fn value(&self) -> f64;
    fn format(&self) -> String;
}

// Trait for operator tokens
pub trait OperatorToken {
    fn precedence(&self) -> u8;
    fn symbol(&self) -> &'static str;
}

// Abstract Factory trait
pub trait TokenFactory {
    type Number: NumberToken;
    type Operator: OperatorToken;

    fn create_number(&self, s: &str) -> Result<Self::Number, String>;
    fn create_operator(&self, s: &str) -> Result<Self::Operator, String>;
}

// Standard calculator implementation
#[derive(Debug, Clone, PartialEq)]
pub struct StandardNumber(pub Number);

impl NumberToken for StandardNumber {
    fn value(&self) -> f64 {
        self.0.value
    }

    fn format(&self) -> String {
        self.0.format()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StandardOperator(pub Operator);

impl OperatorToken for StandardOperator {
    fn precedence(&self) -> u8 {
        match self.0 {
            Operator::Add | Operator::Subtract => 1,
            Operator::Multiply | Operator::Divide => 2,
            Operator::Power => 3,
        }
    }

    fn symbol(&self) -> &'static str {
        match self.0 {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            Operator::Power => "^",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StandardFactory;

impl TokenFactory for StandardFactory {
    type Number = StandardNumber;
    type Operator = StandardOperator;

    fn create_number(&self, s: &str) -> Result<Self::Number, String> {
        match s.parse::<f64>() {
            Ok(value) => Ok(StandardNumber(Number::new(value))),
            Err(_) => Err(format!("Invalid number: {}", s)),
        }
    }

    fn create_operator(&self, s: &str) -> Result<Self::Operator, String> {
        match s {
            "+" => Ok(StandardOperator(Operator::Add)),
            "-" => Ok(StandardOperator(Operator::Subtract)),
            "*" => Ok(StandardOperator(Operator::Multiply)),
            "/" => Ok(StandardOperator(Operator::Divide)),
            "^" => Ok(StandardOperator(Operator::Power)),
            _ => Err(format!("Invalid operator: {}", s)),
        }
    }
}

// Scientific calculator implementation
#[derive(Debug, Clone, PartialEq)]
pub struct ScientificNumber(pub Number);

impl NumberToken for ScientificNumber {
    fn value(&self) -> f64 {
        self.0.value
    }

    fn format(&self) -> String {
        // Scientific calculator prefers scientific notation by default
        match self.0.format {
            NumberFormat::Decimal => format!("{:e}", self.0.value),
            _ => self.0.format(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScientificOperator {
    Basic(Operator),
    Function(Function),
}

impl OperatorToken for ScientificOperator {
    fn precedence(&self) -> u8 {
        match self {
            ScientificOperator::Basic(op) => match op {
                Operator::Add | Operator::Subtract => 1,
                Operator::Multiply | Operator::Divide => 2,
                Operator::Power => 3,
            },
            ScientificOperator::Function(_) => 4,
        }
    }

    fn symbol(&self) -> &'static str {
        match self {
            ScientificOperator::Basic(op) => match op {
                Operator::Add => "+",
                Operator::Subtract => "-",
                Operator::Multiply => "*",
                Operator::Divide => "/",
                Operator::Power => "^",
            },
            ScientificOperator::Function(func) => match func {
                Function::Sin => "sin",
                Function::Cos => "cos",
                Function::Tan => "tan",
                Function::Sqrt => "sqrt",
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ScientificFactory;

impl TokenFactory for ScientificFactory {
    type Number = ScientificNumber;
    type Operator = ScientificOperator;

    fn create_number(&self, s: &str) -> Result<Self::Number, String> {
        // Handle both scientific and standard notation
        match s.parse::<f64>() {
            Ok(value) => {
                let format = if s.contains('e') || s.contains('E') {
                    NumberFormat::Scientific
                } else {
                    NumberFormat::Decimal
                };
                Ok(ScientificNumber(Number::with_format(value, format)))
            }
            Err(_) => Err(format!("Invalid number: {}", s)),
        }
    }

    fn create_operator(&self, s: &str) -> Result<Self::Operator, String> {
        // Scientific calculator supports functions
        match s {
            "+" => Ok(ScientificOperator::Basic(Operator::Add)),
            "-" => Ok(ScientificOperator::Basic(Operator::Subtract)),
            "*" => Ok(ScientificOperator::Basic(Operator::Multiply)),
            "/" => Ok(ScientificOperator::Basic(Operator::Divide)),
            "^" => Ok(ScientificOperator::Basic(Operator::Power)),
            "sin" => Ok(ScientificOperator::Function(Function::Sin)),
            "cos" => Ok(ScientificOperator::Function(Function::Cos)),
            "tan" => Ok(ScientificOperator::Function(Function::Tan)),
            "sqrt" => Ok(ScientificOperator::Function(Function::Sqrt)),
            _ => Err(format!("Invalid operator: {}", s)),
        }
    }
}
