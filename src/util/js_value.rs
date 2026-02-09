//! Converters between JavaScript values and `toml_edit` data structures.

use toml_edit::{
    Array, ArrayOfTables, Date, Datetime, Formatted, InlineTable, Item, Offset, Table, TableLike,
    Time, Value,
};
use wasm_bindgen::{JsCast, JsValue, throw_str};
use web_sys::js_sys::{Array as JsArray, Date as JsDate, Object as JsObject};

use crate::util::value::from_f64;

#[inline]
/// Converts a JavaScript primitive/object/array to a TOML value when possible.
pub fn to_value(js_value: &JsValue, inline: bool) -> Option<Value> {
    if let Some(b) = js_value.as_bool() {
        Some(Value::Boolean(Formatted::new(b)))
    } else if let Some(s) = js_value.as_string() {
        Some(Value::String(Formatted::new(s)))
    } else if let Some(n) = js_value.as_f64() {
        Some(from_f64(n))
    } else if let Some(js_obj) = js_value.dyn_ref::<JsObject>() {
        if let Some(js_array) = js_obj.dyn_ref::<JsArray>() {
            Some(Value::Array(to_array(js_array)))
        } else if let Some(date) = js_obj.dyn_ref::<JsDate>() {
            Some(Value::Datetime(Formatted::new(to_datetime(date))))
        } else if inline {
            Some(Value::InlineTable(to_inline_table(js_obj)))
        } else {
            None
        }
    } else {
        None
    }
}

#[inline]
/// Converts JavaScript values to TOML items, including table and null handling.
pub fn to_item(js_value: &JsValue, inline: bool) -> Item {
    if js_value.is_bigint() {
        throw_str("Bigint is not supported")
    } else if let Some(v) = to_value(js_value, inline) {
        Item::Value(v)
    } else if let Some(js_obj) = js_value.dyn_ref::<JsObject>() {
        Item::Table(to_table(js_obj, inline))
    } else if js_value.is_null() || js_value.is_undefined() {
        Item::None
    } else {
        Item::Value(Value::String(Formatted::new(js_value.as_string().unwrap_or_default())))
    }
}

#[inline]
/// Converts a JS object into a TOML table recursively.
pub fn to_table(js_object: &JsObject, inline: bool) -> Table {
    let entries = JsObject::entries(js_object);

    entries
        .iter()
        .filter_map(|entry| {
            let arr = entry.dyn_ref::<JsArray>().expect("entry of object should be array");
            if arr.length() == 2 {
                let (key, value) = (arr.get(0), arr.get(1));
                let key_str = key.as_string().unwrap();

                Some((key_str, to_item(&value, inline)))
            } else {
                None
            }
        })
        .collect::<Table>()
}

#[inline]
/// Converts a JS object into a TOML inline table.
pub fn to_inline_table(js_object: &JsObject) -> InlineTable {
    let entries = JsObject::entries(js_object);

    entries
        .iter()
        .filter_map(|entry| match entry.dyn_ref::<JsArray>() {
            Some(arr) if arr.length() == 2 => {
                let (key, value) = (arr.get(0), arr.get(1));
                let key_str = key.as_string().unwrap();
                to_value(&value, true).map(|v| (key_str, v))
            }
            _ => None,
        })
        .collect::<InlineTable>()
}

#[inline]
/// Converts a JS array into a TOML array.
pub fn to_array(js_array: &JsArray) -> Array {
    js_array.iter().filter_map(|i| to_value(&i, true)).collect::<Array>()
}

#[inline]
/// Converts a JS `Date` into a TOML UTC datetime.
pub fn to_datetime(js_date: &JsDate) -> Datetime {
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
        nanosecond: js_date.get_utc_milliseconds() * 1_000_000,
    };

    // We use UTC components, so the offset is always 'Z' (Zulu time).
    Datetime { date: Some(date), time: Some(time), offset: Some(Offset::Z) }
}

#[inline]
/// Converts a TOML item into a JavaScript value tree.
pub fn from_item(item: &Item) -> JsValue {
    match item {
        Item::Table(t) => from_table_like(t),
        Item::ArrayOfTables(aot) => from_array_of_tables(aot),
        Item::Value(v) => from_value(v),
        Item::None => JsValue::NULL,
    }
}

#[inline]
/// Converts any table-like TOML node into a JS object.
pub fn from_table_like(table: &dyn TableLike) -> JsValue {
    let entries = table
        .iter()
        .map(|(key, item)| {
            let entry = JsArray::new_with_length(2);
            entry.set(0, JsValue::from_str(key));
            entry.set(1, from_item(item));
            JsValue::from(entry)
        })
        .collect::<JsArray>();
    JsObject::from_entries(&entries).unwrap().into()
}

#[inline]
/// Converts an array-of-tables into a JS array of objects.
pub fn from_array_of_tables(aot: &ArrayOfTables) -> JsValue {
    aot.iter().map(|tbl| from_table_like(tbl)).collect::<JsArray>().into()
}

#[inline]
/// Converts a TOML array into a JS array.
pub fn from_array(arr: &Array) -> JsValue {
    arr.iter().map(from_value).collect::<JsArray>().into()
}

#[inline]
/// Converts a TOML value into its JS representation.
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
