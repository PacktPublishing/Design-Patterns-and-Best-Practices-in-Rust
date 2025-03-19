// If we need to share the calculator between threads, we can wrap it in proper synchronization:
use std::sync::{Arc, Mutex};

struct ThreadSafeCalculator {
    inner: Arc<Mutex<Calculator>>,
}

impl ThreadSafeCalculator {
    fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Calculator::new())),
        }
    }

    fn evaluate(&self, expr: String) -> Result<f64, String> {
        let mut calc = self.inner.lock().map_err(|_| "Lock poisoned")?;
        calc.evaluate(expr)
    }

    // Other methods following the same pattern...
}