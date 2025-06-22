use toml_edit::Array;
use wasm_bindgen::JsValue;
use web_sys::js_sys::Array as JsArray;

use crate::types::value::ValueWrapper;

pub struct ArrayWrapper(pub Array);

impl From<ArrayWrapper> for JsValue {
    fn from(arr: ArrayWrapper) -> Self {
        match u32::try_from(arr.0.len()) {
            Ok(len) => {
                let js_arr = JsArray::new_with_length(len);
                for (i, value) in arr.0.iter().enumerate() {
                    let value = JsValue::from(ValueWrapper(value.to_owned()));
                    js_arr.set(i as u32, value);
                }
                js_arr.into()
            }
            Err(_) => {
                let js_arr = JsArray::new();
                for value in arr.0.iter() {
                    let value = JsValue::from(ValueWrapper(value.to_owned()));
                    js_arr.push(&value);
                }
                js_arr.into()
            }
        }
    }
}
