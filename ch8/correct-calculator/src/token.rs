// token.rs - Core token types and factory methods

// Number formats
#[derive(Debug, Clone, PartialEq)]
pub enum NumberFormat {
    Decimal,
    Scientific,
    Engineering,
}

// Basic token types
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Function {
    Sin,
    Cos,
    Tan,
    Sqrt,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Number {
    pub value: f64,
    pub format: NumberFormat,
}

impl Number {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            format: NumberFormat::Decimal,
        }
    }
    
    pub fn with_format(value: f64, format: NumberFormat) -> Self {
        Self { value, format }
    }
    
    pub fn format(&self) -> String {
        match self.format {
            NumberFormat::Decimal => format!("{}", self.value),
            NumberFormat::Scientific => format!("{:e}", self.value),
            NumberFormat::Engineering => {
                let exp = self.value.abs().log10().floor();
                let adj_exp = (exp - exp % 3.0).floor();
                let coeff = self.value / 10_f64.powf(adj_exp);
                format!("{}e{}", coeff, adj_exp)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(Number),
    Operator(Operator),
    Function(Function),
    Variable(String),
    OpenParen,
    CloseParen,
}

// Factory methods for Token
impl Token {
    // Factory method for creating number tokens
    pub fn number(value: f64) -> Self {
        Self::Number(Number::new(value))
    }
    
    // Factory method for scientific notation
    pub fn scientific_number(value: f64) -> Self {
        Self::Number(Number::with_format(value, NumberFormat::Scientific))
    }
    
    // Factory method for operators
    pub fn operator(op: Operator) -> Self {
        Self::Operator(op)
    }
    
    // Factory method for functions
    pub fn function(func: Function) -> Self {
        Self::Function(func)
    }
    
    // Factory method for variables
    pub fn variable(name: impl Into<String>) -> Self {
        Self::Variable(name.into())
    }
    
    // Factory method from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        // Try parsing as a number first
        if let Ok(num) = s.parse::<f64>() {
            if s.contains('e') || s.contains('E') {
                return Ok(Self::Number(Number::with_format(num, NumberFormat::Scientific)));
            }
            return Ok(Self::number(num));
        }

        // Check for operators
        match s {
            "+" => Ok(Self::operator(Operator::Add)),
            "-" => Ok(Self::operator(Operator::Subtract)),
            "*" => Ok(Self::operator(Operator::Multiply)),
            "/" => Ok(Self::operator(Operator::Divide)),
            "^" => Ok(Self::operator(Operator::Power)),
            // Functions
            "sin" => Ok(Self::function(Function::Sin)),
            "cos" => Ok(Self::function(Function::Cos)),
            "tan" => Ok(Self::function(Function::Tan)),
            "sqrt" => Ok(Self::function(Function::Sqrt)),
            // Parentheses
            "(" => Ok(Self::OpenParen),
            ")" => Ok(Self::CloseParen),
            // Must be a variable
            name if name.chars().all(|c| c.is_alphanumeric() || c == '_') => 
                Ok(Self::variable(name)),
            // Invalid token
            _ => Err(format!("Invalid token: {}", s)),
        }
    }
}

impl Operator {
    pub fn precedence(&self) -> u8 {
        match self {
            Operator::Add | Operator::Subtract => 1,
            Operator::Multiply | Operator::Divide => 2,
            Operator::Power => 3,
        }
    }
    
    pub fn symbol(&self) -> &'static str {
        match self {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            Operator::Power => "^",
        }
    }
}
