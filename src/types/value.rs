use toml_edit::Value;
use wasm_bindgen::JsValue;

use crate::types::{array::ArrayWrapper, table::TableLikeWrapper};

pub struct ValueWrapper(pub Value);

impl From<ValueWrapper> for JsValue {
    fn from(wrapper: ValueWrapper) -> Self {
        match wrapper.0 {
            Value::String(formatted) => JsValue::from_str(formatted.value()),
            Value::Integer(formatted) => JsValue::from_f64(*formatted.value() as f64),
            Value::Float(formatted) => JsValue::from_f64(*formatted.value()),
            Value::Boolean(formatted) => JsValue::from_bool(*formatted.value()),
            Value::Datetime(formatted) => JsValue::from_str(&formatted.value().to_string()),
            Value::Array(arr) => JsValue::from(ArrayWrapper(arr)),
            Value::InlineTable(table) => JsValue::from(TableLikeWrapper::from(table)),
        }
    }
}
