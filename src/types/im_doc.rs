use toml_edit::{ImDocument, Item};
use wasm_bindgen::JsValue;

use crate::types::{array_table::ArrayTablesWrapper, table::TableLikeWrapper, value::ValueWrapper};

pub struct ImDocumentWrapper(pub ImDocument<String>);

impl From<ImDocumentWrapper> for JsValue {
    fn from(val: ImDocumentWrapper) -> Self {
        match val.0.as_item() {
            Item::None => JsValue::NULL,
            Item::Value(value) => JsValue::from(ValueWrapper(value.to_owned())),
            Item::Table(table) => JsValue::from(TableLikeWrapper::from(table.to_owned())),
            Item::ArrayOfTables(aot) => JsValue::from(ArrayTablesWrapper(aot.to_owned())),
        }
    }
}
