use wasm_bindgen::{JsValue, prelude::wasm_bindgen, throw_str};
use web_sys::js_sys::{Array, Object};

#[wasm_bindgen(typescript_custom_section)]
const I_EDIT_OPTIONS: &'static str = r#"
interface IEditOptions {
    /**
     * whether add the final newline
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

pub struct EditOptions {
    pub final_newline: bool,
    pub inline: bool,
    pub action: Action,
}

impl Default for EditOptions {
    fn default() -> Self {
        Self { final_newline: true, inline: true, action: Action::Set }
    }
}

pub enum Action {
    Append,
    Prepend,
    Set,
    Insert,
}

impl EditOptions {
    pub fn new(i: Option<IEditOptions>) -> EditOptions {
        let mut opt = EditOptions::default();
        if let Some(ieo) = i {
            let js_value: JsValue = ieo.into();
            if js_value.is_object() {
                if js_value.is_array() {
                    throw_str("Type Missmatch, IEditOptions can not be array");
                }

                let entries = Object::entries(&js_value.into());

                for entry in entries.iter() {
                    let arr = Array::from(&entry);
                    let key = arr.get(0).as_string().unwrap();
                    let val = arr.get(1);
                    match key.as_str() {
                        "finalNewline" => match val.as_bool() {
                            Some(b) => {
                                if !b {
                                    opt.final_newline = false
                                }
                            }
                            _ => throw_str("Type Missmatch, expect finalNewline to be boolean"),
                        },
                        "inline" => match val.as_bool() {
                            Some(b) => {
                                if !b {
                                    opt.inline = false
                                }
                            }
                            _ => throw_str("Type Missmatch, expect inline to be boolean"),
                        },
                        _ => throw_str(format!("Unknown property '{key}'").as_str()),
                    }
                }
            } else {
                throw_str("IEditOptions should be an object");
            }
        }
        opt
    }
}
