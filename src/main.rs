mod conversions;
mod evaluator;

use std::env;

fn parse_args_list(s: &str) -> Result<Vec<f64>, String> {
    if s.is_empty() {
        return Ok(Vec::new());
    }
    let mut args = Vec::new();
    for part in s.split(',') {
        let val = evaluator::evaluate(part.trim())?;
        args.push(val);
    }
    Ok(args)
}

fn print_examples(prog_name: &str) {
    println!("Scientific Calculator v0.2.0");
    println!(
        "Usage: {} \"<expression1>\" \"<expression2>\" ...",
        prog_name
    );
    println!("Flags: -examples (or --examples) to show this guide\n");

    println!("--- 1. Standard Arithmetic & Chaining ---");
    println!("  {} \"2 + 3 * 4\" \"# * 2\" -> 14, 28", prog_name);
    println!(
        "  {} \"7 / 2\" \"floor(#)\" \"ceil(5.2 + #)\" -> 3.5, 3, 9",
        prog_name
    );

    println!("\n--- 2. Scientific Notation & Digit Separators ---");
    println!("  {} \"1E3 + 1E-3\" -> 1000.001", prog_name);
    println!("  {} \"1_000_000 * 2\" -> 2000000", prog_name);

    println!("\n--- 3. Advanced Math & Factorials ---");
    println!("  {} \"abs(-42.5)\" -> 42.5", prog_name);
    println!("  {} \"20!\" -> 2432902008176640000", prog_name);
    println!("  {} \"mod(10, 3)\" -> 1", prog_name);

    println!("\n--- 4. Trigonometry (Radians & Degrees) ---");
    println!("  {} \"sin(pi / 2)\" -> 1", prog_name);
    println!("  {} \"cos_d(60)\" -> 0.5", prog_name);
    println!("  {} \"tan_d(45)\" -> 1", prog_name);

    println!("\n--- 5. Multi-Argument Functions ---");
    println!("  {} \"min(5, 2, 9, 1)\" -> 1", prog_name);
    println!("  {} \"max(5, 2, 9, 1)\" -> 9", prog_name);
    println!("  {} \"gcd(24, 36)\" -> 12", prog_name);
    println!("  {} \"lcm(4, 6)\" -> 12", prog_name);
    println!("  {} \"lerp(0.5, 10, 20)\" -> 15", prog_name);

    println!("\n--- 6. Coordinate & Angle Conversions ---");
    println!("  {} \"topol_r(3, 4)\" -> 5", prog_name);
    println!(
        "  {} \"topol_theta(0, 5)\" -> 1.5707963267948966",
        prog_name
    );
    println!("  {} \"tocart_x(5, 60)\" -> 2.50", prog_name);
    println!("  {} \"tocart_y(5, 90)\" -> 5", prog_name);
    println!("  {} \"torad(180)\" -> 3.141592653589793", prog_name);
    println!("  {} \"todeg(PI) -> 180\"", prog_name);

    println!("\n--- 7. Base & Character Translations ---");
    println!(
        "  {} \"tobin(15)\" \"tooct(10)\" -> 0b1111, 0o12",
        prog_name
    );
    println!(
        "  {} \"tohex(255)\" \"toascii(97)\" -> 0xFF, \'a\'",
        prog_name
    );
}

fn main() {
    let prog_path = env::args()
        .next()
        .unwrap_or_else(|| "scientific_calc".to_string());

    let prog_name = prog_path
        .split('\\')
        .last()
        .unwrap_or("scientific_calc.exe");

    let args: Vec<String> = env::args().skip(1).collect();

    // Show examples if no arguments are provided or if -examples flag is passed
    if args.is_empty() {
        print_examples(prog_name);
        return;
    }

    // Tracks history across CLI arguments sequentially
    let mut last_result: f64 = 0.0;

    for expr in args {
        let processed_expr = expr.replace("#", &last_result.to_string());

        // Check for multi-argument or conversion wrappers like func(arg1, arg2)
        let mut handled = false;

        if let Some(open_idx) = processed_expr.find('(') {
            if processed_expr.ends_with(')') {
                let func_name = &processed_expr[..open_idx];
                let inner = &processed_expr[open_idx + 1..processed_expr.len() - 1];

                // Check if base conversion or multi-arg
                if matches!(func_name, "toascii" | "tobin" | "tooct" | "tohex") {
                    let val = evaluator::evaluate(inner).unwrap_or(0.0);
                    match conversions::run_conversion(func_name, val) {
                        Ok(conv_res) => {
                            match conv_res {
                                conversions::ConversionResult::Ascii(ch) => {
                                    println!("{} = '{}'", processed_expr, ch)
                                }
                                conversions::ConversionResult::Binary(s)
                                | conversions::ConversionResult::Octal(s)
                                | conversions::ConversionResult::Hex(s) => {
                                    println!("{} = {}", processed_expr, s)
                                }
                            }
                            last_result = val;
                        }
                        Err(e) => eprintln!("Error: {}", e),
                    }
                    handled = true;
                } else {
                    // Try parsing as multi-argument function
                    match parse_args_list(inner) {
                        Ok(arg_vals) => {
                            match conversions::run_multi_arg(func_name, &arg_vals) {
                                Ok(res) => {
                                    println!("{} = {}", processed_expr, res);
                                    last_result = res;
                                    handled = true;
                                }
                                Err(_) => {} // Fall through to standard evaluator if not found
                            }
                        }
                        Err(_) => {}
                    }
                }
            }
        }

        if !handled {
            match evaluator::evaluate(&processed_expr) {
                Ok(numeric_res) => {
                    last_result = numeric_res;
                    println!("{} = {}", processed_expr, numeric_res);
                }
                Err(err) => {
                    eprintln!("Error evaluating '{}': {}", expr, err);
                    break;
                }
            }
        }
    }
}
