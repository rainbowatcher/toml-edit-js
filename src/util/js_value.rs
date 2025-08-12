use toml_edit::{
    Array, ArrayOfTables, Date, Datetime, Formatted, Item, Offset, Table, TableLike, Time, Value,
    value,
};
use wasm_bindgen::{JsCast as _, JsValue, throw_str};
use web_sys::js_sys::{self, Array as JsArray, Date as JsDate, Object as JsObject};

use crate::util::value::f64_to_value;

#[inline]
pub fn to_value(js_value: &JsValue, inline: bool) -> Option<Value> {
    if let Some(b) = js_value.as_bool() {
        Some(Value::from(b))
    } else if let Some(s) = js_value.as_string() {
        Some(Value::from(s))
    } else if let Some(n) = js_value.as_f64() {
        Some(Value::from(f64_to_value(n)))
    } else if let Ok(date) = js_value.to_owned().dyn_into::<JsDate>() {
        Some(Value::from(to_datetime(date)))
    } else if let Ok(js_array) = js_value.to_owned().dyn_into::<JsArray>() {
        Some(Value::from(to_array(js_array, inline)))
    } else {
        None
    }
}

#[inline]
pub fn to_item(js_value: &JsValue, inline: bool) -> Item {
    if js_value.is_null() || js_value.is_undefined() {
        Item::None
    } else if js_value.is_bigint() {
        throw_str("Bigint is not supported")
    } else if let Some(v) = to_value(js_value, inline) {
        value(v)
    } else if let Ok(js_obj) = js_value.to_owned().dyn_into::<JsObject>() {
        let table = to_table(&js_obj);
        if inline {
            Item::Value(Value::InlineTable(table.into_inline_table()))
        } else {
            Item::Table(table)
        }
    } else {
        Item::Value(Value::String(Formatted::new(js_value.as_string().unwrap_or_default())))
    }
}

#[inline]
pub fn to_table(js_object: &JsObject) -> Table {
    let entries = JsObject::entries(js_object);
    let mut table = Table::new();

    for entry in entries.iter() {
        if let Ok(arr) = entry.dyn_into::<JsArray>() {
            if arr.length() == 2 {
                if let (Some(key), value) = (arr.get(0).as_string(), arr.get(1)) {
                    let item = to_item(&value, false);
                    table.insert(key.as_str(), item);
                }
            }
        }
    }

    table
}

#[inline]
pub fn to_array(js_array: JsArray, inline: bool) -> Array {
    let mut toml_array = Array::new();
    for i in 0..js_array.length() {
        let element = js_array.get(i);
        let item = to_item(&element, inline);
        match item {
            Item::None => (),
            Item::Value(value) => toml_array.push(value),
            Item::Table(table) => toml_array.push(table.into_inline_table()),
            Item::ArrayOfTables(aot) => toml_array.push(aot.into_array()),
        }
    }
    toml_array
}

#[inline]
pub fn to_datetime(js_date: JsDate) -> Datetime {
    // Note: JS `get_utc_month()` is 0-indexed (0-11), while TOML is 1-indexed (1-12).
    // We must add 1 to the month.
    let date = Date {
        year: js_date.get_utc_full_year() as u16,
        month: (js_date.get_utc_month() + 1) as u8,
        day: js_date.get_utc_date() as u8,
    };

    // Note: JS provides milliseconds, while TOML supports nanoseconds.
    // We convert milliseconds to nanoseconds by multiplying by 1,000,000.
    let time = Time {
        hour: js_date.get_utc_hours() as u8,
        minute: js_date.get_utc_minutes() as u8,
        second: js_date.get_utc_seconds() as u8,
        nanosecond: js_date.get_utc_milliseconds() as u32 * 1_000_000,
    };

    // We use UTC components, so the offset is always 'Z' (Zulu time).
    Datetime { date: Some(date), time: Some(time), offset: Some(Offset::Z) }
}

#[inline]
pub fn from_item(item: &Item) -> JsValue {
    match item {
        Item::Table(t) => from_table_like(t),
        Item::ArrayOfTables(aot) => from_array_of_tables(aot),
        Item::Value(v) => from_value(v),
        Item::None => JsValue::NULL,
    }
}

#[inline]
pub fn from_table_like(table: &dyn TableLike) -> JsValue {
    let obj = JsObject::new();
    for (key, item) in table.iter() {
        // RECURSIVE CALL, NO CLONING
        let js_val = from_item(item);
        js_sys::Reflect::set(&obj, &JsValue::from_str(key), &js_val).unwrap();
    }
    obj.into()
}

#[inline]
pub fn from_array_of_tables(aot: &ArrayOfTables) -> JsValue {
    let js_arr = JsArray::new_with_length(aot.len() as u32);
    for (i, table) in aot.iter().enumerate() {
        let js_table = from_table_like(table);
        js_arr.set(i as u32, js_table);
    }
    js_arr.into()
}

#[inline]
pub fn from_array(arr: &Array) -> JsValue {
    match u32::try_from(arr.len()) {
        Ok(len) => {
            let js_arr = JsArray::new_with_length(len);
            for (i, value) in arr.iter().enumerate() {
                let value = JsValue::from(from_value(&value));
                js_arr.set(i as u32, value);
            }
            js_arr.into()
        }
        Err(_) => {
            let js_arr = JsArray::new();
            for value in arr.iter() {
                let value = JsValue::from(from_value(&value));
                js_arr.push(&value);
            }
            js_arr.into()
        }
    }
}

#[inline]
pub fn from_value(value: &Value) -> JsValue {
    match value {
        Value::String(formatted) => JsValue::from_str(formatted.value()),
        Value::Integer(formatted) => JsValue::from_f64(*formatted.value() as f64),
        Value::Float(formatted) => JsValue::from_f64(*formatted.value()),
        Value::Boolean(formatted) => JsValue::from_bool(*formatted.value()),
        Value::Datetime(formatted) => JsValue::from_str(&formatted.value().to_string()),
        Value::Array(arr) => from_array(arr),
        Value::InlineTable(table) => from_table_like(table),
    }
}
