fn main() -> Result<(), String> {
    let mut calc = Calculator::new();

    calc.set_variable("pi".to_string(), 3.14159);

    let result1 = calc.evaluate("2 * pi".to_string())?;
    let result2 = calc.evaluate("result + 1".to_string())?;

    for calculation in calc.history() {
        println!("{} = {}", calculation.expression, calculation.result);
    }

    let thread_safe = ThreadSafeCalculator::new();

    let threads: Vec<_> = (0..3).map(|i| {
        let calc = thread_safe.clone();

        std::thread::spawn(move || {
            calc.evaluate(format!("{} + 1", i))
        })
    }).collect();

    Ok(())
}