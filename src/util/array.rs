use crate::{core::error::TomlEditJsError, toml_err};

#[inline]
pub fn parse_array_index(key: &'_ str) -> Result<usize, TomlEditJsError<'_>> {
    match key.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
        Some(inner) => match inner.parse::<usize>() {
            Ok(index) => Ok(index),
            Err(_) => toml_err!(KeyError(key)),
        },
        None => toml_err!(KeyError(key)),
    }
}
