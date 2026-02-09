//! Stringify option definitions and JS input validation.

use wasm_bindgen::{JsCast as _, JsValue, prelude::wasm_bindgen, throw_str};
use web_sys::js_sys::{Array as JsArray, Object as JsObject};

use crate::{core::error::TomlEditJsError, toml_err};

#[wasm_bindgen(typescript_custom_section)]
const I_STRINGIFY_OPTIONS: &'static str = r#"
interface IStringifyOptions {
    /**
     * whether add the final newline
     * @default true
     */
    finalNewline?: boolean;

    /**
     * prefer inline table style
     * @default false
     */
    inline?: boolean;
};
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IStringifyOptions")]
    pub type IStringifyOptions;
}

#[derive(Clone)]
/// Normalized stringify options used by serialization routines.
pub struct StringifyOptions {
    /// Whether to keep a trailing newline in output.
    pub final_newline: bool,
    /// Whether to prefer inline tables for object values.
    pub inline: bool,
    // pub indent: u8,
    // pub min_items: u8,
}

impl Default for StringifyOptions {
    fn default() -> Self {
        Self {
            final_newline: true,
            inline: false,
            // indent: 2,
            // min_items: 3
        }
    }
}

impl StringifyOptions {
    /// Parses and validates user-provided stringify options from JS.
    pub fn new(i: Option<IStringifyOptions>) -> Result<StringifyOptions, TomlEditJsError<'static>> {
        let mut opt = StringifyOptions::default();
        if let Some(ieo) = i {
            let js_value: JsValue = ieo.into();
            if js_value.is_object() {
                if js_value.is_array() {
                    throw_str("Type Missmatch, IStringifyOptions can not be array");
                }

                let entries = JsObject::entries(&js_value.into());

                for entry in entries.iter() {
                    if let Some(arr) = entry.dyn_ref::<JsArray>() {
                        let key = arr.get(0).as_string().unwrap();
                        let val = arr.get(1);
                        match key.as_str() {
                            "finalNewline" => match val.as_bool() {
                                Some(b) => {
                                    if !b {
                                        opt.final_newline = false
                                    }
                                }
                                _ => {
                                    return toml_err!(TypeError(format!(
                                        "expect finalNewline to be boolean"
                                    )));
                                }
                            },
                            "inline" => match val.as_bool() {
                                Some(n) => {
                                    if n {
                                        opt.inline = true
                                    }
                                }
                                _ => {
                                    return toml_err!(TypeError(format!(
                                        "expect inline to be boolean"
                                    )));
                                }
                            },
                            // "indent" => match val.as_f64() {
                            //     Some(n) => opt.indent = n as u8,
                            //     _ => throw_str("Type Missmatch, expect indent to be number"),
                            // },
                            // "minItems" => match val.as_f64() {
                            //     Some(n) => opt.min_items = n as u8,
                            //     _ => throw_str("Type Missmatch, expect minItems to be number"),
                            // },
                            _ => return toml_err!(TypeError(format!("unknown property '{key}'"))),
                        }
                    }
                }
            } else {
                throw_str("IEditOptions should be an object");
            }
        }
        Ok(opt)
    }
}
