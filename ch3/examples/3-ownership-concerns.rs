struct Calculator {
    history: Vec<CalculationResult>,
    current_expression: Option<String>,
}

impl Calculator {
    fn new() -> Self {
        Self {
            history: Vec::new(),
            current_expression: None,
        }
    }

    // Methods that only need to read use &self
    fn view_history(&self) -> &[CalculationResult] {
        &self.history
    }

    fn get_last_result(&self) -> Option<f64> {
        self.history.last().map(|r| r.result)
    }

    // Methods that need to modify use &mut self
    fn add_to_history(&mut self, expression: String, result: f64) {
        self.history.push(CalculationResult {
            expression,
            result,
        });
    }


    fn evaluate(&mut self, expression: String) -> Result<f64, String> {
        let result = self.calculate_expression(&expression)?;
        self.add_to_history(expression, result);
        Ok(result)
    }
}


// If we need to share access to history, we can create a dedicated type
struct HistoryView<'a> {
    entries: &'a [CalculationResult],

}

impl Calculator {
    fn create_history_view(&self) -> HistoryView<'_> {
        HistoryView {
            entries: &self.history,
        }
    }
}