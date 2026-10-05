mod evaluator;
use std::env;

fn main() {
    let prog_path = env::args()
        .next()
        .unwrap_or_else(|| "scientific_calc".to_string());

    let prog_name = prog_path
        .split('\\')
        .last()
        .unwrap_or("scientific_calc.exe");

    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("Scientific Calculator v0.1.0");
        println!("Usage: {} \"<expression1>\" \"<expression2>\"", prog_name);
        println!("Example: {} \"2 + 3\" \"_ * 2\"", prog_name);
        println!(
            "Example: {} \"7 / 2\" \"floor(_)\" \"ceil(5.2 + _)\"",
            prog_name
        );
        return;
    }

    // Tracks history across CLI arguments sequentially
    let mut last_result: f64 = 0.0;

    for expr in args {
        // Substitute '_' with the previous round's data
        let processed_expr = expr.replace("_", &last_result.to_string());

        match evaluator::evaluate(&processed_expr) {
            Ok(result) => {
                println!("{} = {}", processed_expr, result);
                last_result = result; // Store history state for the next expression
            }
            Err(err) => {
                eprintln!("Error evaluating '{}': {}", expr, err);
                break; // Stop execution on the first error
            }
        }
    }
}
