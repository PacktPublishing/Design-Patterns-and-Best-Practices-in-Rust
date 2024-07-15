use std::io::Write;
use std::process::exit;

fn evaluate_expression(expression: &str) -> Result<String, String> {
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
