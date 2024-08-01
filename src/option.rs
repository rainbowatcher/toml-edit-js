use js_sys::{Array, Object};
use serde::Deserialize;
use wasm_bindgen::{prelude::wasm_bindgen, throw_str, JsValue, UnwrapThrowExt};

#[wasm_bindgen(typescript_custom_section)]
const I_EDIT_OPTIONS: &'static str = r#"
interface IEditOptions {
    finalNewline: boolean;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IEditOptions")]
    pub type IEditOptions;
}

#[derive(Default, Deserialize)]
pub struct EditOptions {
    pub final_newline: bool,
}

impl EditOptions {
    pub fn new(i: IEditOptions) -> EditOptions {
        let js_value: JsValue = i.into();
        let mut opt = EditOptions::default();
        if js_value.is_object() {
            if js_value.is_array() {
                throw_str("IEditOptions can not be an array");
            }
            let obj = Object::from(js_value.clone());
            let entries = Object::entries(&obj);

            for entry in entries.iter() {
                if entry.is_array() {
                    let arr = Array::from(&entry);
                    let key = arr.get(0).as_string().expect_throw("key should be a string");
                    let val = arr.get(1);
                    match key.as_str() {
                        "finalNewline" => {
                            opt.final_newline =
                                val.as_bool().expect_throw("finalNewline should be a boolean")
                        }
                        _ => throw_str(format!("unknown property [{key}]").as_str()),
                    }
                } else {
                    throw_str("entry should be a array");
                }
            }
        } else {
            throw_str("IEditOptions should be an object");
        }
        opt
    }
}
