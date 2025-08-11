use toml_edit::{Array, Datetime, Formatted, Item, Value};
use wasm_bindgen::{JsCast, JsValue, throw_str};
use web_sys::js_sys::{Array as JsArray, Date as JsDate, Object as JsObject};

use crate::{
    options::EditOptions,
    types::{array_table::ArrayTablesWrapper, table::TableLikeWrapper, value::ValueWrapper},
};

#[derive(Debug)]
pub struct ItemWrapper(pub Item);

impl ItemWrapper {
    pub fn with_edit_opt(js_value: JsValue, opts: &EditOptions) -> Self {
        let item_wrapper = ItemWrapper::from(js_value);
        let item = if let Item::Table(table) = item_wrapper.0 {
            if opts.inline {
                Item::Value(toml_edit::Value::InlineTable(table.into_inline_table()))
            } else {
                Item::Table(table)
            }
        } else {
            return item_wrapper;
        };
        Self(item)
    }
}

impl From<ItemWrapper> for JsValue {
    fn from(wrapper: ItemWrapper) -> Self {
        match wrapper.0 {
            Item::None => JsValue::NULL,
            Item::Value(value) => JsValue::from(ValueWrapper(value)),
            Item::Table(table) => JsValue::from(TableLikeWrapper::from(table)),
            Item::ArrayOfTables(aot) => JsValue::from(ArrayTablesWrapper(aot)),
        }
    }
}

impl From<JsValue> for ItemWrapper {
    fn from(inn: JsValue) -> Self {
        let item = if inn.is_null() || inn.is_undefined() {
            Item::None
        } else if inn.is_bigint() {
            throw_str("Bigint is not supported")
        } else if let Some(b) = inn.as_bool() {
            Item::Value(Value::Boolean(Formatted::new(b)))
        } else if let Some(s) = inn.as_string() {
            Item::Value(Value::String(Formatted::new(s)))
        } else if let Some(n) = inn.as_f64() {
            Self::from(n).0
        } else if let Ok(date) = inn.to_owned().dyn_into::<JsDate>() {
            Self::from(date).0
        } else if let Ok(js_array) = inn.to_owned().dyn_into::<JsArray>() {
            Self::from(js_array).0
        } else if let Ok(js_obj) = inn.to_owned().dyn_into::<JsObject>() {
            let table = TableLikeWrapper::from(&js_obj);
            Item::Table(table.inner)
        } else {
            Item::Value(Value::String(Formatted::new(inn.as_string().unwrap_or_default())))
        };

        ItemWrapper(item)
    }
}

impl From<f64> for ItemWrapper {
    fn from(value: f64) -> Self {
        if value.fract() != 0.0 || !value.is_finite() {
            return ItemWrapper(Item::Value(Value::Float(Formatted::new(value))));
        }

        if value >= (i64::MIN as f64) && value <= (i64::MAX as f64) {
            ItemWrapper(Item::Value(Value::Integer(Formatted::new(value as i64))))
        } else {
            web_sys::console::warn_1(&format!("Number {value} out of i64 range").into());
            ItemWrapper(Item::Value(Value::Float(Formatted::new(value))))
        }
    }
}

impl From<JsArray> for ItemWrapper {
    fn from(js_array: JsArray) -> Self {
        let mut toml_array = Array::new();
        for i in 0..js_array.length() {
            let element = js_array.get(i);
            let item = ItemWrapper::from(element);
            match item.0 {
                Item::None => (),
                Item::Value(value) => toml_array.push(value),
                Item::Table(table) => toml_array.push(table.into_inline_table()),
                Item::ArrayOfTables(aot) => toml_array.push(aot.into_array()),
            }
        }
        Self(Item::Value(Value::Array(toml_array)))
    }
}

impl From<JsDate> for ItemWrapper {
    fn from(date: JsDate) -> Self {
        let iso_string = date.to_iso_string().as_string().unwrap_or_default();
        iso_string
            .parse::<Datetime>()
            .map(|dt| ItemWrapper(Item::Value(Value::Datetime(Formatted::new(dt)))))
            .unwrap_or_else(|_| ItemWrapper(Item::Value(Value::String(Formatted::new(iso_string)))))
    }
}
