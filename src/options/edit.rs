//! Edit option definitions and JS input validation.

use wasm_bindgen::{JsCast as _, JsValue, prelude::wasm_bindgen};
use web_sys::js_sys::{Array as JsArray, Object as JsObject};

use crate::{core::error::TomlEditJsError, toml_err};

#[wasm_bindgen(typescript_custom_section)]
const I_EDIT_OPTIONS: &'static str = r#"
interface IEditOptions {
    /**
     * whether add the final newline
     * @default true
     */
    finalNewline?: boolean;

    /**
     * Write data in InlineTable format when the value to be written is a object type and inline is set to true
     * @default true
     */
    inline?: boolean;
};
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IEditOptions")]
    pub type IEditOptions;
}

/// Normalized edit options used by internal Rust operations.
pub struct EditOptions {
    /// Whether to keep a trailing newline in output.
    pub final_newline: bool,
    /// Whether JS objects should be emitted as inline tables when possible.
    pub inline: bool,
}

impl Default for EditOptions {
    fn default() -> Self {
        Self {
            // keep multiline format
            final_newline: true,
            inline: true,
        }
    }
}

impl EditOptions {
    /// Parses and validates user-provided edit options from JS.
    pub fn new(i: Option<IEditOptions>) -> Result<EditOptions, TomlEditJsError<'static>> {
        let mut opt = EditOptions::default();
        if let Some(ieo) = i {
            let js_value: JsValue = ieo.into();
            if js_value.is_array() {
                return toml_err!(TypeError(format!("IEditOptions can not be array")));
            }
            if !js_value.is_object() {
                return toml_err!(TypeError(format!("IEditOptions should be an object")));
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
                            Some(b) => {
                                if !b {
                                    opt.inline = false
                                }
                            }
                            _ => {
                                return toml_err!(TypeError(format!(
                                    "expect inline to be boolean"
                                )));
                            }
                        },
                        _ => return toml_err!(TypeError(format!("unknown property '{key}'"))),
                    }
                }
            }
        }
        Ok(opt)
    }
}
