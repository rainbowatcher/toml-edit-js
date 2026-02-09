//! Array-related path helpers.

use crate::{core::error::TomlEditJsError, toml_err};

#[inline]
/// Parses a TOML array index token like `"[3]"`.
pub fn parse_array_index(key: &'_ str) -> Result<usize, TomlEditJsError<'_>> {
    match key.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
        Some(inner) => match inner.parse::<usize>() {
            Ok(index) => Ok(index),
            Err(_) => toml_err!(KeyError(key)),
        },
        None => toml_err!(KeyError(key)),
    }
}

#[cfg(test)]
mod tests {
    use super::parse_array_index;

    #[test]
    fn parse_array_index_accepts_valid_index() {
        assert_eq!(parse_array_index("[12]").unwrap(), 12);
    }

    #[test]
    fn parse_array_index_rejects_non_numeric_value() {
        let error = parse_array_index("[abc]").unwrap_err().to_string();
        assert!(error.contains("invalid key"));
    }

    #[test]
    fn parse_array_index_rejects_malformed_brackets() {
        let error = parse_array_index("12").unwrap_err().to_string();
        assert!(error.contains("invalid key"));
    }
}
