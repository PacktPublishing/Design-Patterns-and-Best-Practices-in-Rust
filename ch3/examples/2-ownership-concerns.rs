use std::cell::RefCell;

struct Calculator {
    history: RefCell<Vec<CalculationResult>>,
    current_expression: RefCell<Option<String>>,
}


impl HistoryManager for Calculator {
    // Now this "works" - but at what cost?
    fn add_to_history(&self, expression: String, result: f64) {
        self.history.borrow_mut().push(CalculationResult {
            expression,
            result,
        });
    }

    fn clear_history(&self) {
        self.history.borrow_mut().clear();
    }
}