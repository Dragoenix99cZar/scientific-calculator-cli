use std::f64::consts::{E, PI};

const TAU: f64 = PI * 2.0;

fn get_precedence(op: &str) -> i32 {
    match op {
        "!" => 6,
        "neg" => 5,
        "sin" | "cos" | "tan" | "sin_d" | "cos_d" | "tan_d" | "sqrt" | "ln" | "log10" | "log2"
        | "exp" | "floor" | "ceil" | "abs" => 4,
        "^" => 3,
        "*" | "/" | "%" => 2,
        "+" | "-" => 1,
        _ => 0,
    }
}

fn is_right_associative(op: &str) -> bool {
    op == "^" || op == "neg" || op == "!"
}

fn get_constant(name: &str) -> Option<f64> {
    match name.to_lowercase().as_str() {
        "pi" => Some(PI),
        "e" => Some(E),
        "tau" => Some(TAU),
        _ => None,
    }
}

/// Tokenizer supporting scientific notation (1E3, 1E-3) and digit separators (_)
pub fn tokenize(expression: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut chars = expression.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }

        // Parse numbers (integers, decimals, underscores, scientific notation)
        if c.is_ascii_digit() || c == '.' {
            let mut num_str = String::new();
            while let Some(&next_c) = chars.peek() {
                if next_c.is_ascii_digit()
                    || next_c == '.'
                    || next_c == '_'
                    || next_c == 'e'
                    || next_c == 'E'
                {
                    num_str.push(next_c);
                    chars.next();
                } else if next_c == '+' || next_c == '-' {
                    let last_c = num_str.chars().last();
                    if last_c == Some('e') || last_c == Some('E') {
                        num_str.push(next_c);
                        chars.next();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            tokens.push(num_str);
        } else if c.is_ascii_alphabetic() {
            let mut id_str = String::new();
            while let Some(&next_c) = chars.peek() {
                if next_c.is_ascii_alphanumeric() || next_c == '_' {
                    id_str.push(next_c);
                    chars.next();
                } else {
                    break;
                }
            }
            tokens.push(id_str);
        } else if "+-*/^(),%!".contains(c) {
            tokens.push(c.to_string());
            chars.next();
        } else {
            return Err(format!("Invalid character: '{}'", c));
        }
    }

    // Handle unary minus
    let mut refined = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        let token = &tokens[i];
        if token == "-" {
            let is_unary = refined.is_empty()
                || refined.last().map_or(false, |last: &String| {
                    last == "(" || last == "," || get_precedence(last) > 0
                });
            if is_unary {
                if i + 1 < tokens.len() && tokens[i + 1].replace('_', "").parse::<f64>().is_ok() {
                    refined.push(format!("-{}", tokens[i + 1]));
                    i += 2;
                    continue;
                } else {
                    refined.push("neg".to_string());
                    i += 1;
                    continue;
                }
            }
        }
        refined.push(token.clone());
        i += 1;
    }

    Ok(refined)
}

pub fn infix_to_postfix(tokens: Vec<String>) -> Result<Vec<String>, String> {
    let mut output: Vec<String> = Vec::new();
    let mut stack: Vec<String> = Vec::new();

    for token in tokens {
        let clean_token = token.replace('_', "");
        if clean_token.parse::<f64>().is_ok() || get_constant(&token).is_some() {
            output.push(token);
        } else if get_precedence(&token) > 0 && token != "!" {
            while let Some(top) = stack.last() {
                if get_precedence(top) > get_precedence(&token)
                    || (get_precedence(top) == get_precedence(&token)
                        && !is_right_associative(&token))
                {
                    output.push(stack.pop().unwrap());
                } else {
                    break;
                }
            }
            stack.push(token);
        } else if token == "!" {
            stack.push(token);
        } else if token == "(" {
            stack.push(token);
        } else if token == ")" {
            while let Some(top) = stack.last() {
                if top != "(" {
                    output.push(stack.pop().unwrap());
                } else {
                    break;
                }
            }
            if stack.last() == Some(&"(".to_string()) {
                stack.pop();
            } else {
                return Err("Mismatched parentheses".into());
            }
        } else if token == "," {
            while let Some(top) = stack.last() {
                if top != "(" {
                    output.push(stack.pop().unwrap());
                } else {
                    break;
                }
            }
        } else {
            // Function names or identifiers
            stack.push(token);
        }
    }

    while let Some(op) = stack.pop() {
        if op == "(" || op == ")" {
            return Err("Mismatched parentheses".into());
        }
        output.push(op);
    }

    Ok(output)
}

fn clean_trig(val: f64) -> f64 {
    let rounded = (val * 1e10).round() / 1e10;
    if (rounded - 0.5).abs() < 1e-9 {
        0.5
    } else if (rounded - (-0.5)).abs() < 1e-9 {
        -0.5
    } else if (rounded - 1.0).abs() < 1e-9 {
        1.0
    } else if (rounded - (-1.0)).abs() < 1e-9 {
        -1.0
    } else if rounded.abs() < 1e-9 {
        0.0
    } else {
        rounded
    }
}

pub fn evaluate_postfix(tokens: Vec<String>) -> Result<f64, String> {
    let mut stack = Vec::new();

    for token in tokens {
        let clean_token = token.replace('_', "");
        if let Ok(val) = clean_token.parse::<f64>() {
            stack.push(val);
        } else if let Some(val) = get_constant(&token) {
            stack.push(val);
        } else {
            match token.as_str() {
                "sin" | "cos" | "tan" | "sin_d" | "cos_d" | "tan_d" | "sqrt" | "ln" | "log10"
                | "log2" | "exp" | "floor" | "ceil" | "abs" | "neg" => {
                    let val = stack
                        .pop()
                        .ok_or("Stack underflow / malformed expression")?;
                    let res = match token.as_str() {
                        "sin" => clean_trig(val.sin()),
                        "cos" => clean_trig(val.cos()),
                        "tan" => clean_trig(val.tan()),
                        "sin_d" => clean_trig((val * PI / 180.0).sin()),
                        "cos_d" => clean_trig((val * PI / 180.0).cos()),
                        "tan_d" => clean_trig((val * PI / 180.0).tan()),
                        "sqrt" => {
                            if val < 0.0 {
                                return Err("Math Error: Square root of negative number".into());
                            }
                            val.sqrt()
                        }
                        "ln" => {
                            if val <= 0.0 {
                                return Err("Math Error: Log of non-positive number".into());
                            }
                            val.ln()
                        }
                        "log10" => {
                            if val <= 0.0 {
                                return Err("Math Error: Log of non-positive number".into());
                            }
                            val.log10()
                        }
                        "log2" => {
                            if val <= 0.0 {
                                return Err("Math Error: Log of non-positive number".into());
                            }
                            val.log2()
                        }
                        "exp" => val.exp(),
                        "floor" => val.floor(),
                        "ceil" => val.ceil(),
                        "abs" => val.abs(),
                        "neg" => -val,
                        _ => unreachable!(),
                    };
                    stack.push(res);
                }
                "!" => {
                    let val = stack.pop().ok_or("Stack underflow")?;
                    if val < 0.0 || val.fract() != 0.0 {
                        return Err("Factorial Error: Input must be a non-negative integer".into());
                    }
                    let n = val as u64;
                    let mut res: u64 = 1;
                    for i in 1..=n {
                        res = res.checked_mul(i).ok_or("Factorial overflow")?;
                    }
                    stack.push(res as f64);
                }
                "+" | "-" | "*" | "/" | "^" | "%" => {
                    let b = stack.pop().ok_or("Stack underflow")?;
                    let a = stack.pop().ok_or("Stack underflow")?;
                    let res = match token.as_str() {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        "/" => {
                            if b == 0.0 {
                                return Err("Math Error: Division by zero".into());
                            }
                            a / b
                        }
                        "%" => {
                            if b == 0.0 {
                                return Err("Math Error: Modulo Division by zero".into());
                            }
                            a % b
                        }
                        "^" => a.powf(b),
                        _ => unreachable!(),
                    };
                    stack.push(res);
                }
                _ => return Err(format!("Unknown token or unhandled function: '{}'", token)),
            }
        }
    }

    if stack.len() != 1 {
        return Err("Invalid expression syntax".into());
    }

    stack.pop().ok_or("Empty stack".to_string())
}

pub fn evaluate(expression: &str) -> Result<f64, String> {
    let tokens = tokenize(expression)?;
    let postfix = infix_to_postfix(tokens)?;
    evaluate_postfix(postfix)
}

#[cfg(test)]
mod tests {
    use crate::evaluator::{
        evaluate, evaluate_postfix, get_constant, get_precedence, infix_to_postfix,
        is_right_associative, tokenize,
    };
    use std::f64::consts::{E, PI};

    const EPSILON: f64 = 1e-9;

    fn assert_approx_eq(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < EPSILON,
            "Expected {}, but got {}",
            expected,
            actual
        );
    }

    // ==========================================
    // 1. COMPONENT UNIT TESTS (Internal Functions)
    // ==========================================

    #[test]
    fn test_internal_precedence_and_associativity() {
        // Line & branch coverage for get_precedence
        assert_eq!(get_precedence("neg"), 5);
        assert_eq!(get_precedence("sin"), 4);
        assert_eq!(get_precedence("^"), 3);
        assert_eq!(get_precedence("*"), 2);
        assert_eq!(get_precedence("+"), 1);
        assert_eq!(get_precedence("unknown"), 0);

        // Branch coverage for is_right_associative
        assert!(is_right_associative("^"));
        assert!(is_right_associative("neg"));
        assert!(!is_right_associative("+"));
    }

    #[test]
    fn test_internal_constants() {
        // Coverage for matching cases (ignoring or checking lowercase variant rules)
        assert_approx_eq(get_constant("pi").unwrap(), PI);
        assert_approx_eq(get_constant("PI").unwrap(), PI);
        assert_approx_eq(get_constant("e").unwrap(), E);
        assert_approx_eq(get_constant("tau").unwrap(), PI * 2.0);
        assert!(get_constant("invalid").is_none());
    }

    // ==========================================
    // 2. TOKENIZATION COVERAGE
    // ==========================================

    #[test]
    fn test_tokenize_success_paths() {
        // Whitespace, numbers, decimals, constants, and operators
        let tokens = tokenize("  123.45 + pi * sin(3) , 2^3 ").unwrap();
        assert_eq!(
            tokens,
            vec![
                "123.45", "+", "pi", "*", "sin", "(", "3", ")", ",", "2", "^", "3"
            ]
        );
    }

    #[test]
    fn test_tokenize_errors() {
        // Trigger invalid character branch error
        let err = tokenize("2 @ 3");
        assert!(err.is_err());
        assert_eq!(err.unwrap_err(), "Invalid character: '@'");
    }

    #[test]
    fn test_tokenize_unary_minus_edge_cases() {
        // Leading unary minus attached to parsed float literal
        assert_eq!(tokenize("-5").unwrap(), vec!["-5"]);

        // Leading unary converted to explicit token indicator "neg"
        assert_eq!(
            tokenize("-sin(0)").unwrap(),
            vec!["neg", "sin", "(", "0", ")"]
        );

        // Unary minus immediately following brackets, commas, or precedence gaps
        assert_eq!(tokenize("(-5)").unwrap(), vec!["(", "-5", ")"]);
        assert_eq!(
            tokenize("(-sin(0))").unwrap(),
            vec!["(", "neg", "sin", "(", "0", ")", ")"]
        );
        assert_eq!(
            tokenize("max(2, -3)").unwrap(),
            vec!["max", "(", "2", ",", "-3", ")"]
        );
        assert_eq!(tokenize("2 * -3").unwrap(), vec!["2", "*", "-3"]);
    }

    // ==========================================
    // 3. SHUNTING YARD (INFIX TO POSTFIX) COVERAGE
    // ==========================================

    #[test]
    fn test_infix_to_postfix_handling() {
        // Right associative check: 2 ^ 3 ^ 2 -> [2, 3, 2, ^, ^]
        let tokens = vec!["2".into(), "^".into(), "3".into(), "^".into(), "2".into()];
        assert_eq!(
            infix_to_postfix(tokens).unwrap(),
            vec!["2", "3", "2", "^", "^"]
        );

        // Left associative check: 4 / 2 * 3 -> [4, 2, /, 3, *]
        let tokens = vec!["4".into(), "/".into(), "2".into(), "*".into(), "3".into()];
        assert_eq!(
            infix_to_postfix(tokens).unwrap(),
            vec!["4", "2", "/", "3", "*"]
        );

        // Balanced parenthesis tracking
        let tokens = vec!["(".into(), "2".into(), "+".into(), "3".into(), ")".into()];
        assert_eq!(infix_to_postfix(tokens).unwrap(), vec!["2", "3", "+"]);
    }

    #[test]
    fn test_infix_to_postfix_errors() {
        // Mismatched closing parenthesis
        let tokens = vec!["2".into(), "+".into(), "3".into(), ")".into()];
        assert_eq!(
            infix_to_postfix(tokens).unwrap_err(),
            "Mismatched parentheses"
        );

        // Mismatched unclosed trailing open parenthesis
        let tokens = vec!["(".into(), "2".into(), "+".into(), "3".into()];
        assert_eq!(
            infix_to_postfix(tokens).unwrap_err(),
            "Mismatched parentheses"
        );
    }

    // ==========================================
    // 4. POSTFIX EVALUATION & MATH ACCURACY COVERAGE
    // ==========================================

    #[test]
    fn test_all_supported_math_functions() {
        assert_approx_eq(evaluate("sin(0)").unwrap(), 0.0);
        assert_approx_eq(evaluate("cos(0)").unwrap(), 1.0);
        assert_approx_eq(evaluate("tan(0)").unwrap(), 0.0);
        assert_approx_eq(evaluate("sqrt(16)").unwrap(), 4.0);
        assert_approx_eq(evaluate("ln(e)").unwrap(), 1.0);
        assert_approx_eq(evaluate("log10(100)").unwrap(), 2.0);
        assert_approx_eq(evaluate("log2(8)").unwrap(), 3.0);
        assert_approx_eq(evaluate("exp(1)").unwrap(), E);
        assert_approx_eq(evaluate("floor(5.7)").unwrap(), 5.0);
        assert_approx_eq(evaluate("ceil(5.2)").unwrap(), 6.0);
        assert_approx_eq(evaluate("neg(5)").unwrap(), -5.0);
    }

    #[test]
    fn test_binary_operators() {
        assert_approx_eq(evaluate("5 + 3").unwrap(), 8.0);
        assert_approx_eq(evaluate("5 - 3").unwrap(), 2.0);
        assert_approx_eq(evaluate("5 * 3").unwrap(), 15.0);
        assert_approx_eq(evaluate("6 / 3").unwrap(), 2.0);
        assert_approx_eq(evaluate("2 ^ 3").unwrap(), 8.0);
    }

    #[test]
    fn test_evaluation_errors_and_underflows() {
        // Division by zero branch guard
        assert_eq!(
            evaluate("5 / 0").unwrap_err(),
            "Math Error: Division by zero"
        );

        // Negative numbers wrapped in square roots
        assert_eq!(
            evaluate("sqrt(-1)").unwrap_err(),
            "Math Error: Square root of negative number"
        );

        // Boundary violations for logarithms
        assert_eq!(
            evaluate("ln(0)").unwrap_err(),
            "Math Error: Log of non-positive number"
        );
        assert_eq!(
            evaluate("log10(-5)").unwrap_err(),
            "Math Error: Log of non-positive number"
        );
        assert_eq!(
            evaluate("log2(0)").unwrap_err(),
            "Math Error: Log of non-positive number"
        );

        // Syntactical flaws triggering stack anomalies
        assert_eq!(evaluate("5 +").unwrap_err(), "Stack underflow");
        assert_eq!(
            evaluate("sin()").unwrap_err(),
            "Stack underflow / malformed expression"
        );
        assert_eq!(evaluate("5 6").unwrap_err(), "Invalid expression syntax");

        // Unknown or unexpected tokens safely processed via explicit errors
        let invalid_postfix = vec!["unknown_op".to_string()];
        assert!(evaluate_postfix(invalid_postfix).is_err());
    }

    // ==========================================
    // 5. INTEGRATION STATE (CLI HISTORY STATE)
    // ==========================================

    #[test]
    fn test_sequential_cli_history_simulation() {
        // Mimics main.rs sequencing through successive substitution phases
        let step_1_res = evaluate("7 / 2").unwrap();
        assert_approx_eq(step_1_res, 3.5);

        let step_2_expr = "floor(_)".replace("_", &step_1_res.to_string());
        let step_2_res = evaluate(&step_2_expr).unwrap();
        assert_approx_eq(step_2_res, 3.0);

        let step_3_expr = "ceil(5.2 + _)".replace("_", &step_2_res.to_string());
        let step_3_res = evaluate(&step_3_expr).unwrap();
        assert_approx_eq(step_3_res, 9.0);
    }

    #[test]
    fn test_scientific_notation_and_separators() {
        assert_approx_eq(evaluate("1E3").unwrap(), 1000.0);
        assert_approx_eq(evaluate("1E-3").unwrap(), 0.001);
        assert_approx_eq(evaluate("10.001000").unwrap(), 10.001);
        assert_approx_eq(evaluate("1_000").unwrap(), 1000.0);
        assert_approx_eq(evaluate("1_000_000").unwrap(), 1_000_000.0);
    }

    #[test]
    fn test_absolute_value_and_unary() {
        assert_approx_eq(evaluate("abs(-42.5)").unwrap(), 42.5);
        assert_approx_eq(evaluate("neg(5)").unwrap(), -5.0);
    }

    #[test]
    fn test_factorials() {
        assert_approx_eq(evaluate("0!").unwrap(), 1.0);
        assert_approx_eq(evaluate("5!").unwrap(), 120.0);
        assert_approx_eq(evaluate("20!").unwrap(), 2432902008176640000.0);
    }

    #[test]
    fn test_degree_trig_precision() {
        assert_approx_eq(evaluate("cos_d(60)").unwrap(), 0.5);
        assert_approx_eq(evaluate("sin_d(90)").unwrap(), 1.0);
        assert_approx_eq(evaluate("tan_d(45)").unwrap(), 1.0);
        assert_approx_eq(evaluate("cos_d(90)").unwrap(), 0.0);
    }

    #[test]
    fn test_basic_arithmetic_and_errors() {
        assert_approx_eq(evaluate("5 + 3 * 2").unwrap(), 11.0);
        assert!(evaluate("5 / 0").is_err());
        assert!(evaluate("sqrt(-1)").is_err());
    }
}
