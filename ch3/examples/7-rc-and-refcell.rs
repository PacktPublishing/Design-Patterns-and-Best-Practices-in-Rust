#[derive(Debug)]
enum Token {
    Number(f64),
    Variable(String),
    Operator(char),
}

// Results of parsing an expression
struct ParsedExpression {
    tokens: Vec<Token>,
}

// Represents a complete calculation
struct Calculation {
    expression: String,
    tokens: Vec<Token>,
    result: f64,
}

struct Calculator {
    variables: HashMap<String, f64>,
    history: Vec<Calculation>,
}

impl Calculator {
    fn new() -> Self {
        Self {
            variables: HashMap::new(),
            history: Vec::new(),
        }
    }


    // Parse the expression into tokens

    fn parse(&self, expr: &str) -> Result<ParsedExpression, String> {
        let tokens = self.tokenize(expr)?;
        Ok(ParsedExpression { tokens })
    }


    // Evaluate a parsed expression
    fn evaluate_parsed(&mut self, expr: String, parsed: ParsedExpression) -> Result<f64, String> {
        let result = self.evaluate_tokens(parsed.tokens.clone())?;

        // Store the calculation in history
        self.history.push(Calculation {
            expression: expr,
            tokens: parsed.tokens,
            result,
        });

        Ok(result)
    }

    // Convenient wrapper that combines parse and evaluate
    fn evaluate(&mut self, expr: String) -> Result<f64, String> {
        let parsed = self.parse(&expr)?;
        self.evaluate_parsed(expr, parsed)
    }

    // Get a reference to the calculation history
    fn history(&self) -> &[Calculation] {
        &self.history
    }

    // Get the most recent calculation result
    fn last_result(&self) -> Option<f64> {
        self.history.last().map(|calc| calc.result)
    }

    // Set or update a variable value
    fn set_variable(&mut self, name: String, value: f64) {
        self.variables.insert(name, value);
    }


    // Get a variable's value
    fn get_variable(&self, name: &str) -> Option<f64> {
        self.variables.get(name).copied()
    }
}
