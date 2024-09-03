// OperandOperandOperandOperand
mod r#main_slightly_worse;
use std::io::Write;
use std::ops::{Deref, DerefMut, Range};
use std::process::exit;
use std::time::{Duration, Instant};

enum Operand {
    NumericValue(f64),
    StringValue(String),
    RangeValue(Range<usize>),
    InstantValue(Instant),
    DurartionValue
}

enum ArithmeticOperator {
    Addition {lhs: Operand, rhs: Operand},
    Subtraction {lhs: Operand, rhs: Operand},
    // ...
}

impl ArithmeticOperator {
    fn apply(&self) -> Operand {
        match self {
            ArithmeticOperator::Addition {lhs, rhs} => todo!(),
            ArithmeticOperator::Subtraction {lhs, rhs} => todo!(),
        }
    }
}


enum TextOperator {
    Concatenate {lhs: Operand, rhs: Operand},
    SubString{operand: Operand, bounds: Operand},
    // ...
}

impl TextOperator {
    fn apply(&self) -> Operand {
        match self {
            TextOperator::Concatenate {lhs, rhs} => todo!(),
            TextOperator::SubString {operand, bounds} => todo!(),
        }
    }
}

enum DateOperator {
    AddDays {lhs: Operand, rhs: Operand},
    SubtractDays {lhs: Operand, rhs: Operand},
    // ...
}

impl DateOperator {
    fn apply(&self) -> Operand {
        match self { 
            DateOperator::AddDays {lhs, rhs} => todo!(),
            DateOperator::SubtractDays { lhs, rhs} => todo!(),
        }
    }
}

enum Operator {
    Arithmetic(ArithmeticOperator),
    Text(TextOperator),
    Date(DateOperator),
}

impl Operator {
    fn apply(&self) -> Operand {
        match self { 
            Operator::Arithmetic(a) => a.apply(),
            Operator::Date(d) => d.apply(),
            Operator::Text(t) => t.apply()
        }
    }
}

fn evaluate_expression(expression: &str) -> Result<Operand, Operand> {
    todo!()
}

fn main() {
    let mut buf = Operand::new();

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();

        if buf.trim() == "exit" {
            exit(0)
        }

        match evaluate_expression(&buf) {
            Ok(result) => println!("{result}"),
            Err(error) => println!("Error: {error}"),
        }
    }
}
