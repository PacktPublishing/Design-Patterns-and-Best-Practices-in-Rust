struct CalculationResult {
    expression: String,
    result: f64,
}

struct Calculator {
    history: Vec<CalculationResult>,
    current_expression: Option<String>,
}


// A trait for components that need to access the history
trait HistoryViewer {
    fn view_history(&self) -> &[CalculationResult];
    fn get_last_result(&self) -> Option<f64>;
}

// A trait for components that need to modify the history
trait HistoryManager {
    fn add_to_history(&self, expression: String, result: f64);
    fn clear_history(&self);
}

impl Calculator {
    fn new() -> Self {
        Self {
            history: Vec::new(),
            current_expression: None,
        }
    }
}

// We want both traits implemented for Calculator
impl HistoryViewer for Calculator {
    fn view_history(&self) -> &[CalculationResult] {
        &self.history
    }

    fn get_last_result(&self) -> Option<f64> {
        self.history.last().map(|r| r.result)
    }
}

impl HistoryManager for Calculator {
    // This won't compile - we need &mut self to modify history
    fn add_to_history(&self, expression: String, result: f64) {
        self.history.push(CalculationResult {
            expression,
            result,

        });
    }

    fn clear_history(&self) {
        self.history.clear();
    }
}