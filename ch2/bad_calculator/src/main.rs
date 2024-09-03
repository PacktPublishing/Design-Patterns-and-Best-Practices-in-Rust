use std::io::Write;
use std::ops::{Deref, DerefMut};
use std::process::exit;

trait Operand {
    fn evaluate(&self) -> f64;
}

struct Value(f64);

impl Operand for Value {
    fn evaluate(&self) -> f64 {
        self.0
    }
}

trait Operator {
    fn precedence(&self) -> u8;
    fn symbol(&self) -> char;
    fn push_operand(&mut self, operand: Box<dyn Operand>);
    fn pop_operand(&mut self) -> Box<dyn Operand>;
    fn apply(&mut self) -> Box<dyn Operand>;
}

trait UnaryOperator: crate::Operator {
    fn apply_unary(&self, operand: Box<dyn Operand>) -> Box<dyn Operand>;

    fn apply(&mut self) -> Box<dyn Operand> {
        let operand = self.pop_operand();
        self.apply_unary(operand)
    }
}
trait BinaryOperator: Operator {
    fn apply_binary(
        &self,
        operand1: Box<dyn Operand>,
        opperand2: Box<dyn Operand>,
    ) -> Box<dyn Operand>;
}

struct OperandStack(Vec<Box<dyn Operand>>);

impl OperandStack {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn push_operand(&mut self, operand: Box<dyn Operand>) {
        self.0.push(operand);
    }

    fn pop_operand(&mut self) -> Box<dyn Operand> {
        self.0.pop().unwrap()
    }

    fn clear_stack(&mut self) {
        self.0.clear();
    }
}

struct AdditionOperator {
    pub stack: OperandStack,
}

impl crate::AdditionOperator {
    fn new() -> Self {
        Self {
            stack: OperandStack::new(),
        }
    }
}

impl Operator for AdditionOperator {
    fn precedence(&self) -> u8 {
        0
    }

    fn symbol(&self) -> char {
        '+'
    }

    fn push_operand(&mut self, operand: Box<dyn Operand>) {
        self.stack.push_operand(operand);
    }

    fn pop_operand(&mut self) -> Box<dyn Operand> {
        self.stack.pop_operand()
    }

    fn apply(&mut self) -> Box<dyn Operand> {
        let operand2 = self.pop_operand();
        let operand1 = self.pop_operand();
        self.apply_binary(operand1, operand2)
    }
}
impl BinaryOperator for AdditionOperator {
    fn apply_binary(
        &self,
        operand1: Box<dyn Operand>,
        operand2: Box<dyn Operand>,
    ) -> Box<dyn Operand> {
        // Operand 2 was push last, so it is popped first
        let inner_operand2 = operand2.as_ref().evaluate();
        let inner_operand1 = operand1.evaluate();
        let result = inner_operand1 + inner_operand2;
        Box::new(Value(result))
    }
}

impl Deref for AdditionOperator {
    type Target = OperandStack;

    fn deref(&self) -> &Self::Target {
        &self.stack
    }
}

impl DerefMut for AdditionOperator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.stack
    }
}

fn evaluate_expression(expression: &str) -> Result<String, String> {
    /*
    let some_operand= Box::new(Value(0.0));
    let mut addition_operator = AdditionOperator::new();
    addition_operator.push_operand(some_operand);
    addition_operator.clear_stack();
    let popped_operand = addition_operator.pop_operand();
     */
    todo!()
}

fn main() {
    let mut buf = String::new();

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
