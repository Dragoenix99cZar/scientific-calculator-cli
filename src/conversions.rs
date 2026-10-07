#[derive(Debug, Clone, PartialEq)]
pub enum ConversionResult {
    Ascii(char),
    Binary(String),
    Octal(String),
    Hex(String),
}

/// Validates and converts an f64 scalar value into base format representations
pub fn run_conversion(func: &str, val: f64) -> Result<ConversionResult, String> {
    if val < 0.0 || val.fract() != 0.0 {
        return Err(format!(
            "Conversion Error: '{}' inputs must be positive integers",
            func
        ));
    }

    let int_val = val as u64;

    match func {
        "toascii" => {
            if int_val > 255 {
                return Err("ASCII Error: Value must be an integer between 0 and 255".into());
            }
            Ok(ConversionResult::Ascii(int_val as u8 as char))
        }
        "tobin" => Ok(ConversionResult::Binary(format!("0b{:b}", int_val))),
        "tooct" => Ok(ConversionResult::Octal(format!("0o{:o}", int_val))),
        "tohex" => Ok(ConversionResult::Hex(format!("0x{:X}", int_val))),
        _ => Err(format!("Unknown conversion function: '{}'", func)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
