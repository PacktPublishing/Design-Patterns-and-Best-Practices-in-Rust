#[derive(Clone)]  // The temptation begins...
struct Variable {
    name: String,
    value: f64,
}

#[derive(Clone)]  // And continues...
enum Token {
    Number(f64),
    Variable(Variable),
    Operator(char),
}

struct Calculator {
    variables: HashMap<String, Variable>,
}

impl Calculator {
    fn tokenize(&self, expression: &str) -> Vec<Token> {
        let mut tokens = Vec::new();

        for part in expression.split_whitespace() {
            if let Some(var) = self.variables.get(part) {
                // Just clone it!
                tokens.push(Token::Variable(var.clone()));
            }
            // ... rest of tokenization
        }

        tokens
    }

    fn evaluate(&self, tokens: Vec<Token>) -> f64 {
        let mut working_tokens = tokens.clone();  // Clone the whole vector!

        while working_tokens.len() > 1 {
            // Find next operator
            let op_pos = working_tokens.iter().position(|t| {
                matches!(t, Token::Operator(_))
            }).unwrap();

            // Get operands (more cloning!)
            let left = working_tokens[op_pos - 1].clone();
            let right = working_tokens[op_pos + 1].clone();

            // Calculate result
            let result = self.apply_operator(left, right);

            // Remove old tokens and insert result
            working_tokens.drain(op_pos - 1..=op_pos + 1);
            working_tokens.insert(op_pos - 1, Token::Number(result));
        }

        match working_tokens[0] {
            Token::Number(n) => n,
            _ => panic!("Invalid expression")
        }
    }
}