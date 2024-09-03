use std::io::Write;
use std::process::exit;

enum Operand {
    Value(f64),
}

impl Operand {
    fn evaluate(&self) -> f64 {
        match self {
            Operand::Value(v) => *v,
        }
    }
}

enum Operator {
    Addition { lhs: Operand, rhs: Operand, precedence: u8, symbol: char  },
    Subtraction { lhs: Operand, rhs: Operand, precedence: u8, symbol: char  },
    Multiplication { lhs: Operand, rhs: Operand, precedence: u8, symbol: char  },
    Division { lhs: Operand, rhs: Operand, precedence: u8, symbol: char  },
    Negation { operand: Operand, precedence: u8, symbol: char },
}

impl Operator {
    fn new(operator_type: String, operand1: Operand, operand2: Option<Operand>) -> Self {
        match operator_type.as_str() {
            "add" => Operator::Addition {lhs: operand1, rhs: operand2.unwrap(), precedence:0,
            symbol: '+'},
            "sub" => Operator::Addition {lhs: operand1, rhs: operand2.unwrap(),
            precedence: 0, symbol: '-'},
            "mul" => Operator::Division {lhs: operand1, rhs: operand2.unwrap(),
            precedence: 1, symbol: '*'},
            "div" => Operator::Division {lhs: operand1, rhs: operand2.unwrap(),
            precedence: 1, symbol: '/'},
            "neg" => Operator::Negation {operand: operand1, precedence:2, symbol: '-'},
            _ => panic!("Unknown operator")
        }
    }

    fn apply(&self) -> Operand {
        let inner = match self {
            Operator::Addition { lhs, rhs, .. } => lhs.evaluate() + rhs.evaluate(),
            Operator::Subtraction { lhs, rhs , ..} => lhs.evaluate() - rhs.evaluate(),
            Operator::Multiplication { lhs, rhs, .. } => lhs.evaluate() * rhs.evaluate(),
            Operator::Division { lhs, rhs, .. } => lhs.evaluate() / rhs.evaluate(),
            Operator::Negation { operand, .. } => -operand.evaluate(),
        };
        Operand::Value(inner)
    }

    fn precedence(&self) -> u8 {
        match self {
            Operator::Addition { .. } | Operator::Subtraction { .. } => 0,
            Operator::Multiplication { .. } | Operator::Division { .. } => 1,
            Operator::Negation { .. } => 2,
        }
    }

    fn symbol(&self) -> char {
        match self {
            Operator::Addition { .. } => '+',
            Operator::Subtraction { .. } => '-',
            Operator::Multiplication { .. } => '*',
            Operator::Division { .. } => '/',
            Operator::Negation { .. } => '-',
        }
    }
}
fn evaluate_expression(expression: &str) -> Result<String, String> {
    // let addition = Operator::Addition { lhs: Operand::Value(2.0), rhs: Operand::Value(3.0) };
    // let subtraction = Operator::Subtraction { lhs: Operand::Value(5.0), rhs: Operand::Value(1.0) };
    // let negation = Operator::Negation { operand: Operand::Value(-7.0) };
    //
    // println!("Addition result: {}", addition.apply().evaluate());
    // println!("Subtraction result: {}", subtraction.apply().evaluate());
    // println!("Negation result: {}", negation.apply().evaluate());
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
