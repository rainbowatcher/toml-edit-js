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

#[cfg(test)]
mod tests {
    use super::{
        clean_insert_prefix, clean_insert_suffix, get_array_decor, get_item_decor, get_table_decor,
        get_value_decor, strip_leading_inline_comment,
    };
    use toml_edit::{Array, Formatted, Item, Table, Value, value};

    #[test]
    fn get_array_decor_uses_second_item_when_present() {
        let mut arr = Array::new();
        let mut first = Value::Integer(Formatted::new(1));
        first.decor_mut().set_prefix(" ");
        first.decor_mut().set_suffix("");
        arr.push_formatted(first);

        let mut second = Value::Integer(Formatted::new(2));
        second.decor_mut().set_prefix("\n  ");
        second.decor_mut().set_suffix("\n");
        arr.push_formatted(second);

        let (prefix, suffix) = get_array_decor(&arr);
        assert_eq!(prefix, "\n  ");
        assert_eq!(suffix, "\n");
    }

    #[test]
    fn get_array_decor_falls_back_to_defaults_without_second_item() {
        let mut arr = Array::new();
        arr.push(1);
        let (prefix, suffix) = get_array_decor(&arr);
        assert_eq!(prefix, " ");
        assert_eq!(suffix, "");
    }

    #[test]
    fn get_value_decor_returns_existing_decor() {
        let mut v = Value::Integer(Formatted::new(1));
        v.decor_mut().set_prefix("\n");
        v.decor_mut().set_suffix("\n");
        let (prefix, suffix) = get_value_decor(&v);
        assert_eq!(prefix, "\n");
        assert_eq!(suffix, "\n");
    }

    #[test]
    fn get_table_decor_returns_existing_decor() {
        let mut table = Table::new();
        table.decor_mut().set_prefix(" ");
        table.decor_mut().set_suffix("\n");
        let (prefix, suffix) = get_table_decor(&table);
        assert_eq!(prefix, " ");
        assert_eq!(suffix, "\n");
    }

    #[test]
    fn get_item_decor_returns_value_decor_for_value_item() {
        let mut v = Value::String(Formatted::new("x".to_string()));
        v.decor_mut().set_prefix(" ");
        v.decor_mut().set_suffix("\n");
        let item = Item::Value(v);
        let (prefix, suffix) = get_item_decor(&item);
        assert_eq!(prefix, " ");
        assert_eq!(suffix, "\n");
    }

    #[test]
    fn get_item_decor_returns_default_for_none() {
        let (prefix, suffix) = get_item_decor(&Item::None);
        assert_eq!(prefix, " ");
        assert_eq!(suffix, "");
    }

    #[test]
    fn clean_insert_prefix_keeps_trailing_newline_block() {
        assert_eq!(clean_insert_prefix(" # cmt\n  "), "\n  ");
        assert_eq!(clean_insert_prefix("  "), "  ");
    }

    #[test]
    fn clean_insert_suffix_keeps_only_newline_tail() {
        assert_eq!(clean_insert_suffix(" # cmt\n"), "\n");
        assert_eq!(clean_insert_suffix(""), "");
        assert_eq!(clean_insert_suffix(" # cmt"), "");
    }

    #[test]
    fn strip_leading_inline_comment_removes_comment_prefix() {
        assert_eq!(strip_leading_inline_comment(" # cmt\n  "), "\n  ");
        assert_eq!(strip_leading_inline_comment(" # cmt"), "");
    }

    #[test]
    fn strip_leading_inline_comment_keeps_regular_prefix() {
        assert_eq!(strip_leading_inline_comment("  "), "  ");
    }

    #[test]
    fn get_item_decor_returns_table_decor_for_table_item() {
        let mut table = Table::new();
        table.insert("k", value(1));
        table.decor_mut().set_prefix("\n");
        table.decor_mut().set_suffix("\n");
        let item = Item::Table(table);
        let (prefix, suffix) = get_item_decor(&item);
        assert_eq!(prefix, "\n");
        assert_eq!(suffix, "\n");
    }
}
