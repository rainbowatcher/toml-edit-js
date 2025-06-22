use once_cell::sync::Lazy;
use toml_edit::{Array, Item, RawString, Table, Value};

pub static DEFAULT_PREFIX: Lazy<RawString> = Lazy::new(|| RawString::from(" "));
pub static DEFAULT_SUFFIX: Lazy<RawString> = Lazy::new(|| RawString::from(""));

pub fn get_array_decor(arr: &Array) -> (RawString, RawString) {
    let second_item = arr.get(1);
    let prefix = second_item.and_then(|i| i.decor().prefix()).unwrap_or(&DEFAULT_PREFIX).to_owned();
    let suffix = second_item.and_then(|i| i.decor().suffix()).unwrap_or(&DEFAULT_SUFFIX).to_owned();
    (prefix, suffix)
}

pub fn get_value_dector(value: &Value) -> (RawString, RawString) {
    let prefix = value.decor().prefix().unwrap_or(&DEFAULT_PREFIX).to_owned();
    let suffix = value.decor().suffix().unwrap_or(&DEFAULT_SUFFIX).to_owned();
    (prefix, suffix)
}

pub fn get_table_dector(table: &Table) -> (RawString, RawString) {
    let prefix = table.decor().prefix().unwrap_or(&DEFAULT_PREFIX).to_owned();
    let suffix = table.decor().suffix().unwrap_or(&DEFAULT_SUFFIX).to_owned();
    (prefix, suffix)
}

pub fn get_item_decor(item: &Item) -> (RawString, RawString) {
    match item {
        Item::None | Item::ArrayOfTables(_) => {
            (DEFAULT_PREFIX.to_owned(), DEFAULT_SUFFIX.to_owned())
        }
        Item::Value(value) => get_value_dector(value),
        Item::Table(table) => get_table_dector(table),
    }
}
