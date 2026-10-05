use std::f64::consts::{E, PI};

const TAU: f64 = PI * 2.0;

fn get_precedence(op: &str) -> i32 {
    match op {
        "neg" => 5,
        "sin" | "cos" | "tan" | "sqrt" | "ln" | "log10" | "log2" | "exp" | "floor" | "ceil" => 4,
        "^" => 3,
        "*" | "/" => 2,
        "+" | "-" => 1,
        _ => 0,
    }
}

fn is_right_associative(op: &str) -> bool {
    op == "^" || op == "neg"
}

fn get_constant(name: &str) -> Option<f64> {
    match name.to_lowercase().as_str() {
        "pi" => Some(PI),
        "e" => Some(E),
        "tau" => Some(TAU),
        _ => None,
    }
}

/// Manual lightweight tokenizer (Zero dependencies!)
pub fn tokenize(expression: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut chars = expression.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }

        // Parse numbers (integers & decimals)
        if c.is_ascii_digit() || c == '.' {
            let mut num_str = String::new();
            while let Some(&next_c) = chars.peek() {
                if next_c.is_ascii_digit() || next_c == '.' {
                    num_str.push(next_c);
                    chars.next();
                } else {
                    break;
                }
            }
            tokens.push(num_str);
        }
        // Parse identifiers (functions or constants like sin, pi)
        else if c.is_ascii_alphabetic() {
            let mut id_str = String::new();
            while let Some(&next_c) = chars.peek() {
                if next_c.is_ascii_alphanumeric() {
                    id_str.push(next_c);
                    chars.next();
                } else {
                    break;
                }
            }
            tokens.push(id_str);
        }
        // Parse operators and symbols
        else if "+-*/^(),".contains(c) {
            tokens.push(c.to_string());
            chars.next();
        } else {
            return Err(format!("Invalid character: '{}'", c));
        }
    }

    // Handle unary minus conversion
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
                if i + 1 < tokens.len() && tokens[i + 1].parse::<f64>().is_ok() {
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

/// Convert infix tokens to postfix using the Shunting-Yard algorithm
pub fn infix_to_postfix(tokens: Vec<String>) -> Result<Vec<String>, String> {
    // Explicitly type-annotate as Vec<String> to prevent type inference errors
    let mut output: Vec<String> = Vec::new();
    let mut stack: Vec<String> = Vec::new();

    for token in tokens {
        if token.parse::<f64>().is_ok() || get_constant(&token).is_some() {
            output.push(token);
        } else if get_precedence(&token) > 0 {
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

/// Evaluate a postfix token queue
pub fn evaluate_postfix(tokens: Vec<String>) -> Result<f64, String> {
    let mut stack = Vec::new();

    for token in tokens {
        if let Ok(val) = token.parse::<f64>() {
            stack.push(val);
        } else if let Some(val) = get_constant(&token) {
            stack.push(val);
        } else {
            match token.as_str() {
                "sin" | "cos" | "tan" | "sqrt" | "ln" | "log10" | "log2" | "exp" | "floor"
                | "ceil" | "neg" => {
                    let val = stack
                        .pop()
                        .ok_or("Stack underflow / malformed expression")?;
                    let res = match token.as_str() {
                        "sin" => val.sin(),
                        "cos" => val.cos(),
                        "tan" => val.tan(),
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
                        "floor" => val.floor(), // <-- Must be present
                        "ceil" => val.ceil(),   // <-- Must be present
                        "neg" => -val,
                        _ => unreachable!(),
                    };
                    stack.push(res);
                }
                "+" | "-" | "*" | "/" | "^" | "\\" => {
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
                        "^" => a.powf(b),
                        _ => unreachable!(),
                    };
                    stack.push(res);
                }
                _ => return Err(format!("Unknown token: '{}'", token)),
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
