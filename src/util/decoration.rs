use once_cell::sync::Lazy;
use toml_edit::{Array, Item, Table, Value};

pub static DEFAULT_PREFIX: Lazy<&str> = Lazy::new(|| " ");
pub static DEFAULT_SUFFIX: Lazy<&str> = Lazy::new(|| "");

#[inline]
pub fn get_array_decor(arr: &Array) -> (&str, &str) {
    let second_item = arr.get(1);
    let prefix = second_item
        .and_then(|i| i.decor().prefix().and_then(|i| i.as_str()))
        .unwrap_or(&DEFAULT_PREFIX);
    let suffix = second_item
        .and_then(|i| i.decor().suffix().and_then(|i| i.as_str()))
        .unwrap_or(&DEFAULT_SUFFIX);
    (prefix, suffix)
}

#[inline]
pub fn get_value_decor(value: &Value) -> (&str, &str) {
    let prefix = value.decor().prefix().and_then(|i| i.as_str()).unwrap_or(&DEFAULT_PREFIX);
    let suffix = value.decor().suffix().and_then(|i| i.as_str()).unwrap_or(&DEFAULT_SUFFIX);
    (prefix, suffix)
}

#[inline]
pub fn get_table_decor(table: &Table) -> (&str, &str) {
    let prefix = table.decor().prefix().and_then(|i| i.as_str()).unwrap_or(&DEFAULT_PREFIX);
    let suffix = table.decor().suffix().and_then(|i| i.as_str()).unwrap_or(&DEFAULT_SUFFIX);
    (prefix, suffix)
}

#[inline]
pub fn get_item_decor(item: &Item) -> (&str, &str) {
    match item {
        Item::None | Item::ArrayOfTables(_) => (&DEFAULT_PREFIX, &DEFAULT_SUFFIX),
        Item::Value(value) => get_value_decor(value),
        Item::Table(table) => get_table_decor(table),
    }
}
