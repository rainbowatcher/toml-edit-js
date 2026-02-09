//! Numeric conversion helpers.

use toml_edit::{Formatted, Value};

#[inline]
/// Converts an `f64` from JS into the most suitable TOML numeric value.
///
/// Integer-range finite numbers become `Integer`; others become `Float`.
pub fn from_f64(value: f64) -> Value {
    if value.fract() != 0.0 || !value.is_finite() {
        Value::Float(Formatted::new(value))
    } else if value >= (i64::MIN as f64) && value <= (i64::MAX as f64) {
        Value::Integer(Formatted::new(value as i64))
    } else {
        web_sys::console::warn_1(&format!("Number {value} out of i64 range").into());
        Value::Float(Formatted::new(value))
    }
}

#[cfg(test)]
mod tests {
    use super::from_f64;
    use toml_edit::Value;

    #[test]
    fn from_f64_returns_integer_for_finite_whole_number() {
        let result = from_f64(42.0);
        assert!(matches!(result, Value::Integer(_)));
        assert_eq!(result.as_integer(), Some(42));
    }

    #[test]
    fn from_f64_returns_float_for_fractional_number() {
        let result = from_f64(3.5);
        assert!(matches!(result, Value::Float(_)));
        assert_eq!(result.as_float(), Some(3.5));
    }

    #[test]
    fn from_f64_returns_float_for_non_finite_number() {
        let result = from_f64(f64::INFINITY);
        assert!(matches!(result, Value::Float(_)));
        assert!(result.as_float().unwrap().is_infinite());
    }
}
