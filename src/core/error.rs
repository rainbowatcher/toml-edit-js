//! Error types and helpers shared by parse/stringify/edit flows.

use thiserror::Error;
use wasm_bindgen::JsValue;

/// Domain errors returned by core editing routines.
#[derive(Error, Debug)]
pub enum TomlEditJsError<'a> {
    /// Failed to parse input TOML text.
    #[error("Parse Error: {0}")]
    ParseError(#[from] toml_edit::TomlError),
    /// Invalid key token in a path expression.
    #[error("Key Error: invalid key '{0}'")]
    KeyError(&'a str),
    /// Array index is outside the current bounds.
    #[error("Key Error: index out of boundary '{0}' for '{1}'")]
    IndexOutOfBounds(usize, String),
    /// Empty path key sequence was provided.
    #[error("Key Error: path key is empty")]
    EmptyKey,
    /// Type mismatch for operation or option value.
    #[error("Type error: {0}")]
    TypeError(String),
}

/// Converts domain errors to JS-friendly string exceptions.
impl From<TomlEditJsError<'_>> for JsValue {
    fn from(value: TomlEditJsError) -> Self {
        JsValue::from(value.to_string())
    }
}

#[macro_export]
macro_rules! toml_err {
    (ParseError($e:expr)) => {
        Err($crate::core::error::TomlEditJsError::ParseError($e))
    };
    (KeyError($k:ident)) => {
        Err($crate::core::error::TomlEditJsError::KeyError($k))
    };
    (IndexOutOfBounds($idx:expr, $target:expr)) => {
        Err($crate::core::error::TomlEditJsError::IndexOutOfBounds($idx, $target))
    };
    (EmptyKey) => {
        Err($crate::core::error::TomlEditJsError::EmptyKey)
    };
    (TypeError($msg:expr)) => {
        Err($crate::core::error::TomlEditJsError::TypeError($msg))
    };
}
