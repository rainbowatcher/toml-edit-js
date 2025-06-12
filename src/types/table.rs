use toml_edit::{Table, TableLike};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::js_sys::{Array as JsArray, Object as JsObject};

use crate::types::item::ItemWrapper;

pub struct TableLikeWrapper<T: TableLike> {
    pub inner: T,
}

impl<T> From<T> for TableLikeWrapper<T>
where
    T: TableLike,
{
    fn from(value: T) -> Self {
        TableLikeWrapper { inner: value }
    }
}

impl<T> From<TableLikeWrapper<T>> for JsValue
where
    T: TableLike,
{
    fn from(value: TableLikeWrapper<T>) -> Self {
        let entries = JsArray::new();
        for (key, item) in value.inner.iter() {
            let entry = JsArray::new_with_length(2);
            entry.set(0, JsValue::from_str(key));
            entry.set(1, JsValue::from(ItemWrapper(item.to_owned())));
            entries.push(&entry);
        }
        JsObject::from_entries(&entries).unwrap().into()
    }
}

impl From<JsObject> for TableLikeWrapper<Table> {
    fn from(obj: JsObject) -> Self {
        let entries = JsObject::entries(&obj);
        let mut table = Table::new();

        for entry in entries.iter() {
            if let Ok(arr) = entry.dyn_into::<JsArray>() {
                if arr.length() == 2 {
                    if let (Some(key), value) = (arr.get(0).as_string(), arr.get(1)) {
                        let item = ItemWrapper::from(value);
                        table.insert(key.as_str(), item.0);
                    }
                }
            }
        }

        TableLikeWrapper::from(table)
    }
}
