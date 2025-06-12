use toml_edit::ArrayOfTables;
use wasm_bindgen::JsValue;
use web_sys::js_sys::Array as JsArray;

use crate::types::table::TableLikeWrapper;

pub struct ArrayTablesWrapper(pub ArrayOfTables);

impl From<ArrayTablesWrapper> for JsValue {
    fn from(arr: ArrayTablesWrapper) -> Self {
        match u32::try_from(arr.0.len()) {
            Ok(len) => {
                let js_arr = JsArray::new_with_length(len);
                for (i, value) in arr.0.iter().enumerate() {
                    let table = JsValue::from(TableLikeWrapper::from(value.to_owned()));
                    js_arr.set(i as u32, table);
                }
                js_arr.into()
            }
            Err(_) => {
                let js_arr = JsArray::new();
                for value in arr.0.iter() {
                    let table = JsValue::from(TableLikeWrapper::from(value.to_owned()));
                    js_arr.push(&table);
                }
                js_arr.into()
            }
        }
    }
}
