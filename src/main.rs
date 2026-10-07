mod conversions;
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
        println!(
            "Usage: {} \"<expression1>\" \"<expression2>\" ...",
            prog_name
        );
        println!("\nExamples (Standard Math & Chaining):");
        println!("  {} \"2 + 3\" \"_ * 2\"", prog_name);
        println!("  {} \"7 / 2\" \"floor(_)\" \"ceil(5.2 + _)\"", prog_name);
        println!("\nExamples (Base Formats & Character Translations):");
        println!("  {} \"tobin(15)\" \"tooct(10)\"", prog_name);
        println!("  {} \"tohex(255)\" \"toascii(97)\"", prog_name);
        println!("  {} \"29523 % 26\" \"64 + _\" \"toascii(_)\"", prog_name);
        return;
    }

    // Tracks history across CLI arguments sequentially
    let mut last_result: f64 = 0.0;

    for expr in args {
        // Substitute '_' with the previous round's data
        let processed_expr = expr.replace("_", &last_result.to_string());

        // Check if this CLI argument is wrapping a translation block
        let mut conversion_type = None;
        let mut math_target = processed_expr.as_str();

        for prefix in &["toascii", "tobin", "tooct", "tohex"] {
            if processed_expr.starts_with(prefix) && processed_expr.ends_with(')') {
                conversion_type = Some(*prefix);
                // Strip out "prefix(" and ")" to capture pure nested inner expression
                let start_idx = prefix.len() + 1;
                let end_idx = processed_expr.len() - 1;
                math_target = &processed_expr[start_idx..end_idx];
                break;
            }
        }

        // 1. Evaluate the mathematical expression block
        match evaluator::evaluate(math_target) {
            Ok(numeric_res) => {
                last_result = numeric_res; // Keep updating the numeric state tracking pipeline

                // 2. Intercept and run conversion if a translation prefix wrapper was present
                if let Some(func_name) = conversion_type {
                    match conversions::run_conversion(func_name, numeric_res) {
                        Ok(conv_res) => match conv_res {
                            conversions::ConversionResult::Ascii(ch) => {
                                println!("{} = '{}'", processed_expr, ch);
                            }
                            conversions::ConversionResult::Binary(s)
                            | conversions::ConversionResult::Octal(s)
                            | conversions::ConversionResult::Hex(s) => {
                                println!("{} = {}", processed_expr, s);
                            }
                        },
                        Err(err) => {
                            eprintln!("Error executing format conversion: {}", err);
                            break;
                        }
                    }
                } else {
                    // Regular clean floating-point math formatting display path
                    println!("{} = {}", processed_expr, numeric_res);
                }
            }
            Err(err) => {
                eprintln!("Error evaluating '{}': {}", expr, err);
                break;
            }
        }
    }
}
