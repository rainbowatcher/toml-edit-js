//! Decor extraction and normalization helpers.
//!
//! `toml_edit` keeps whitespace/comments in decor fields. These helpers centralize
//! how we reuse decor while editing to preserve style and avoid comment drift.

use once_cell::sync::Lazy;
use toml_edit::{Array, Item, Table, Value};

/// Default leading spacing for inserted values.
pub static DEFAULT_PREFIX: Lazy<&str> = Lazy::new(|| " ");
/// Default trailing decor for inserted values.
pub static DEFAULT_SUFFIX: Lazy<&str> = Lazy::new(|| "");

#[inline]
/// Gets representative decor for array insertions from the second element.
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
/// Gets value decor with fallback defaults.
pub fn get_value_decor(value: &Value) -> (&str, &str) {
    let prefix = value.decor().prefix().and_then(|i| i.as_str()).unwrap_or(&DEFAULT_PREFIX);
    let suffix = value.decor().suffix().and_then(|i| i.as_str()).unwrap_or(&DEFAULT_SUFFIX);
    (prefix, suffix)
}

#[inline]
/// Gets table decor with fallback defaults.
pub fn get_table_decor(table: &Table) -> (&str, &str) {
    let prefix = table.decor().prefix().and_then(|i| i.as_str()).unwrap_or(&DEFAULT_PREFIX);
    let suffix = table.decor().suffix().and_then(|i| i.as_str()).unwrap_or(&DEFAULT_SUFFIX);
    (prefix, suffix)
}

#[inline]
/// Gets decor from any item variant.
pub fn get_item_decor(item: &Item) -> (&str, &str) {
    match item {
        Item::None | Item::ArrayOfTables(_) => (&DEFAULT_PREFIX, &DEFAULT_SUFFIX),
        Item::Value(value) => get_value_decor(value),
        Item::Table(table) => get_table_decor(table),
    }
}

#[inline]
/// Keeps only the trailing newline block for insertion prefix reuse.
pub fn clean_insert_prefix(prefix: &str) -> &str {
    match prefix.rfind('\n') {
        Some(idx) => &prefix[idx..],
        None => prefix,
    }
}

#[inline]
/// Drops inline suffix comments and keeps only newline suffix blocks.
pub fn clean_insert_suffix(suffix: &str) -> &str {
    match suffix.find('\n') {
        Some(idx) => &suffix[idx..],
        None => "",
    }
}

#[inline]
/// Removes a leading inline comment fragment from key prefix decor.
pub fn strip_leading_inline_comment(prefix: &str) -> &str {
    if prefix.starts_with(" #") {
        match prefix.find('\n') {
            Some(idx) => &prefix[idx..],
            None => "",
        }
    } else {
        prefix
    }
}
