use std::f64::consts::PI;

#[derive(Debug, Clone, PartialEq)]
pub enum ConversionResult {
    Ascii(char),
    Binary(String),
    Octal(String),
    Hex(String),
}

fn calc_gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs()
}

fn calc_lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a / calc_gcd(a, b)) * b.abs()
    }
}

/// Handles multi-argument functions, base conversions, and coordinate transforms
pub fn run_multi_arg(func: &str, args: &[f64]) -> Result<f64, String> {
    match func {
        "min" => {
            if args.is_empty() {
                return Err("min requires at least 1 argument".into());
            }
            Ok(args.iter().cloned().fold(f64::INFINITY, f64::min))
        }
        "max" => {
            if args.is_empty() {
                return Err("max requires at least 1 argument".into());
            }
            Ok(args.iter().cloned().fold(f64::NEG_INFINITY, f64::max))
        }
        "gcd" => {
            if args.is_empty() {
                return Err("gcd requires at least 1 argument".into());
            }
            let mut acc = args[0].round() as i64;
            for &v in &args[1..] {
                acc = calc_gcd(acc, v.round() as i64);
            }
            Ok(acc as f64)
        }
        "lcm" => {
            if args.is_empty() {
                return Err("lcm requires at least 1 argument".into());
            }
            let mut acc = args[0].round() as i64;
            for &v in &args[1..] {
                acc = calc_lcm(acc, v.round() as i64);
            }
            Ok(acc as f64)
        }
        "lerp" => {
            if args.len() != 3 {
                return Err("lerp expects 3 arguments: lerp(value, min_val, max_val)".into());
            }
            Ok(args[1] + args[0] * (args[2] - args[1]))
        }
        "topol_r" => {
            if args.len() != 2 {
                return Err("topol_r expects 2 arguments: topol_r(x, y)".into());
            }
            let x = args[0];
            let y = args[1];
            Ok((x * x + y * y).sqrt())
        }
        "topol_theta" => {
            if args.len() != 2 {
                return Err("topol_theta expects 2 arguments: topol_theta(x, y)".into());
            }
            let x = args[0];
            let y = args[1];
            let rad = y.atan2(x);
            let mut deg = rad * 180.0 / PI;
            if deg < 0.0 {
                deg += 360.0; // Normalize negative angles to 0-360 range
            }
            Ok(deg)
        }
        "tocart_x" => {
            if args.len() != 2 {
                return Err("tocart_x expects 2 arguments: tocart_x(r, angle)".into());
            }
            let r = args[0];
            let angle = args[1] * PI / 180.0;
            Ok(r * angle.cos())
        }
        "tocart_y" => {
            if args.len() != 2 {
                return Err("tocart_y expects 2 arguments: tocart_y(r, angle)".into());
            }
            let r = args[0];
            let angle = args[1] * PI / 180.0;
            Ok(r * angle.sin())
        }
        "torad" => {
            if args.len() != 1 {
                return Err("torad expects 1 argument: torad(degrees)".into());
            }
            // Converts degrees to radians
            Ok(args[0] * PI / 180.0)
        }
        "todeg" => {
            if args.len() != 1 {
                return Err("todeg expects 1 argument: todeg(radians)".into());
            }
            let mut deg = args[0] * 180.0 / PI;
            if deg < 0.0 {
                deg += 360.0; // Normalize negative angles to 0-360 range
            }
            Ok(deg)
        }
        "mod" => {
            if args.len() != 2 {
                return Err("mod expects 2 arguments: mod(a, b)".into());
            }
            if args[1] == 0.0 {
                return Err("Math Error: Modulo by zero".into());
            }
            Ok(args[0] % args[1])
        }
        _ => Err(format!("Unknown multi-argument function: '{}'", func)),
    }
}

pub fn run_conversion(func: &str, val: f64) -> Result<ConversionResult, String> {
    match func {
        "toascii" => {
            if val < 0.0 || val.fract() != 0.0 || val > 255.0 {
                return Err("ASCII Error: Value must be an integer between 0 and 255".into());
            }
            Ok(ConversionResult::Ascii(val as u8 as char))
        }
        "tobin" => {
            if val < 0.0 || val.fract() != 0.0 {
                return Err("Binary Error: Value must be a positive integer".into());
            }
            Ok(ConversionResult::Binary(format!("0b{:b}", val as u64)))
        }
        "tooct" => {
            if val < 0.0 || val.fract() != 0.0 {
                return Err("Octal Error: Value must be a positive integer".into());
            }
            Ok(ConversionResult::Octal(format!("0o{:o}", val as u64)))
        }
        "tohex" => {
            if val < 0.0 || val.fract() != 0.0 {
                return Err("Hex Error: Value must be a positive integer".into());
            }
            Ok(ConversionResult::Hex(format!("0x{:X}", val as u64)))
        }
        _ => Err(format!("Unknown conversion function: '{}'", func)),
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    use crate::conversions::{ConversionResult, run_conversion, run_multi_arg};

    const EPSILON: f64 = 1e-9;

    fn assert_approx_eq(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < EPSILON,
            "Expected {}, but got {}",
            expected,
            actual
        );
    }

    #[test]
    fn test_conversions_success() {
        // Test ASCII character boundary mapping
        assert_eq!(
            run_conversion("toascii", 65.0).unwrap(),
            ConversionResult::Ascii('A')
        );
        assert_eq!(
            run_conversion("toascii", 97.0).unwrap(),
            ConversionResult::Ascii('a')
        );

        // Test base formats conversions
        assert_eq!(
            run_conversion("tobin", 10.0).unwrap(),
            ConversionResult::Binary("0b1010".to_string())
        );
        assert_eq!(
            run_conversion("tooct", 64.0).unwrap(),
            ConversionResult::Octal("0o100".to_string())
        );
        assert_eq!(
            run_conversion("tohex", 255.0).unwrap(),
            ConversionResult::Hex("0xFF".to_string())
        );

        assert_eq!(
            run_conversion("tooct", 10.0).unwrap(),
            ConversionResult::Octal("0o12".to_string())
        );
    }

    #[test]
    fn test_conversions_validation_failures() {
        // Negative numbers check
        assert!(run_conversion("tobin", -5.0).is_err());

        // Floating point decimal check (must be whole integers)
        assert!(run_conversion("tohex", 42.42).is_err());

        // ASCII upper boundary limit checks (Max 255)
        assert!(run_conversion("toascii", 256.0).is_err());

        // Handling completely unknown functions gracefully
        assert!(run_conversion("unknown_func", 10.0).is_err());
    }

    #[test]
    fn test_multi_arg_min_max() {
        let min_res = run_multi_arg("min", &[5.0, 2.0, 9.0, 1.0]).unwrap();
        assert_approx_eq(min_res, 1.0);

        let max_res = run_multi_arg("max", &[5.0, 2.0, 9.0, 1.0]).unwrap();
        assert_approx_eq(max_res, 9.0);
    }

    #[test]
    fn test_multi_arg_gcd_lcm() {
        let gcd_res = run_multi_arg("gcd", &[24.0, 36.0]).unwrap();
        assert_approx_eq(gcd_res, 12.0);

        let lcm_res = run_multi_arg("lcm", &[4.0, 6.0]).unwrap();
        assert_approx_eq(lcm_res, 12.0);
    }

    #[test]
    fn test_multi_arg_lerp_and_mod() {
        let lerp_res = run_multi_arg("lerp", &[0.5, 10.0, 20.0]).unwrap();
        assert_approx_eq(lerp_res, 15.0);

        let mod_res = run_multi_arg("mod", &[10.0, 3.0]).unwrap();
        assert_approx_eq(mod_res, 1.0);
    }

    #[test]
    fn test_coordinate_transforms() {
        let topol_res = run_multi_arg("topol", &[3.0, 4.0]).unwrap();
        assert_approx_eq(topol_res, 5.0);

        let torect_res = run_multi_arg("torect", &[5.0, 0.0]).unwrap();
        assert_approx_eq(torect_res, 5.0);
    }

    #[test]
    fn test_base_and_ascii_conversions() {
        assert_eq!(
            run_conversion("toascii", 65.0).unwrap(),
            ConversionResult::Ascii('A')
        );
        assert_eq!(
            run_conversion("tobin", 10.0).unwrap(),
            ConversionResult::Binary("0b1010".to_string())
        );
        assert_eq!(
            run_conversion("tooct", 64.0).unwrap(),
            ConversionResult::Octal("0o100".to_string())
        );
        assert_eq!(
            run_conversion("tohex", 255.0).unwrap(),
            ConversionResult::Hex("0xFF".to_string())
        );

        // Validation check bounds
        assert!(run_conversion("toascii", 300.0).is_err());
        assert!(run_conversion("tobin", -2.0).is_err());
    }
}
