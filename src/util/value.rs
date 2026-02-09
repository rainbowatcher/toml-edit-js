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
