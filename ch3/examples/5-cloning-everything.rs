struct Variable {
    name: String,
    value: f64,
}

enum Token<'a> {
    Number(f64),
    Variable(&'a Variable),
    Operator(char),
}

impl Calculator {
    fn tokenize<'a>(&'a self, expression: &str) -> Vec<Token<'a>> {
        let mut tokens = Vec::new();

        for part in expression.split_whitespace() {
            if let Some(var) = self.variables.get(part) {
                // Just use a reference
                tokens.push(Token::Variable(var));
            }
            // ... rest of tokenization
        }

        tokens
    }

    fn evaluate(&self, mut tokens: Vec<Token>) -> f64 {
        while tokens.len() > 1 {
            // Find next operator
            let op_pos = tokens.iter().position(|t| {
                matches!(t, Token::Operator(_))
            }).unwrap();

            // Calculate result using references
            let result = match (&tokens[op_pos - 1], &tokens[op_pos + 1]) {
                (Token::Number(n1), Token::Number(n2)) => {
                    self.apply_operator(*n1, *n2)
                }
                (Token::Variable(v1), Token::Number(n2)) => {
                    self.apply_operator(v1.value, *n2)
                }
                // ... other combinations
                _ => panic!("Invalid expression")
            };

            // Remove old tokens and insert result
            tokens.drain(op_pos - 1..=op_pos + 1);
            tokens.insert(op_pos - 1, Token::Number(result));
        }

        match tokens[0] {
            Token::Number(n) => n,
            _ => panic!("Invalid expression")
        }
    }
}