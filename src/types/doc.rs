use toml_edit::{Document, Item};
use wasm_bindgen::JsValue;

use crate::types::{array_table::ArrayTablesWrapper, table::TableLikeWrapper, value::ValueWrapper};

pub struct DocumentWrapper<'a>(pub Document<&'a str>);

impl<'a> From<DocumentWrapper<'a>> for JsValue {
    fn from(val: DocumentWrapper) -> Self {
        match val.0.as_item() {
            Item::None => JsValue::NULL,
            Item::Value(value) => JsValue::from(ValueWrapper(value.to_owned())),
            Item::Table(table) => JsValue::from(TableLikeWrapper::from(table.to_owned())),
            Item::ArrayOfTables(aot) => JsValue::from(ArrayTablesWrapper(aot.to_owned())),
        }
    }
}
