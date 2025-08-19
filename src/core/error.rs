use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Error, Debug)]
pub enum TomlEditJsError<'a> {
    #[error("Parse Error: {0}")]
    ParseError(#[from] toml_edit::TomlError),
    #[error("Key Error: invalid key '{0}'")]
    KeyError(&'a str),
    #[error("Key Error: index out of boundary '{0}' for '{1}'")]
    IndexOutOfBounds(usize, String),
    #[error("Key Error: path key is empty")]
    EmptyKey,
    #[error("Type error: {0}")]
    TypeError(String),
}

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
