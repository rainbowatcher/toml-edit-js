//! Core `set` mutation logic for `edit(path, value)`.
//!
//! This module routes path updates across:
//! - normal tables
//! - inline tables
//! - arrays
//! - arrays of tables
//!
//! The implementation preserves local decor to keep comments/whitespace stable.
use std::cmp::Ordering;
use std::cmp::Ordering::Less;

use toml_edit::{Array, Item, Key, TableLike, Value};
use wasm_bindgen::JsValue;

use crate::{
    core::error::TomlEditJsError,
    options::EditOptions,
    toml_err,
    util::{
        array::parse_array_index,
        decoration::{
            clean_insert_prefix, clean_insert_suffix, get_array_decor, get_item_decor,
            get_value_decor, strip_leading_inline_comment,
        },
        find::find_parent_item,
        js_value::to_item,
    },
};

#[inline]
/// Sets a value under a parsed parent path and terminal key/index.
pub fn set_value<'a>(
    obj: &'a mut Item,
    path_keys: &Vec<&'a str>,
    value_key: &'a str,
    value: &JsValue,
    options: &'a EditOptions,
) -> Result<(), TomlEditJsError<'a>> {
    let parent = find_parent_item(obj, &path_keys)?;
    let value_item = to_item(value, options.inline);
    let parent_path = path_keys.join(".");

    if let Ok(index) = parse_array_index(value_key) {
        return handle_array_path(parent, index, value_item, &parent_path);
    }

    handle_tablelike_path(parent, value_key, value_item)
}

#[inline]
/// Handles array index writes/replaces/removals for a resolved parent item.
fn handle_array_path<'a>(
    parent: &mut Item,
    index: usize,
    value_item: Item,
    parent_path: &str,
) -> Result<(), TomlEditJsError<'a>> {
    if parent.get(index).is_none() {
        return insert_array_item(parent, index, value_item, parent_path);
    }

    if let Item::ArrayOfTables(aot) = parent {
        return replace_aot_item(aot, index, value_item, parent_path);
    }

    if value_item.is_none() {
        remove_existing_array_item(parent, index);
        return Ok(());
    }

    replace_existing_array_item(parent, index, value_item);
    Ok(())
}

#[inline]
/// Handles table-like key writes for a resolved parent item.
fn handle_tablelike_path<'a>(
    parent: &mut Item,
    value_key: &'a str,
    value_item: Item,
) -> Result<(), TomlEditJsError<'a>> {
    let is_inline_table = matches!(parent, Item::Value(Value::InlineTable(_)));
    if let Some(table) = parent.as_table_like_mut() {
        insert_tablelike(table, value_key, value_item, is_inline_table);
        Ok(())
    } else {
        toml_err!(KeyError(value_key))
    }
}

#[inline]
/// Inserts into an array-like parent when target index does not exist yet.
fn insert_array_item<'a>(
    parent: &mut Item,
    index: usize,
    value_item: Item,
    parent_path: &str,
) -> Result<(), TomlEditJsError<'a>> {
    if let Item::Value(Value::Array(arr)) = parent {
        return insert_into_value_array(arr, index, value_item, parent_path);
    }

    if let Item::ArrayOfTables(aot) = parent {
        return insert_into_aot(aot, index, value_item, parent_path);
    }

    toml_err!(TypeError(format!("item '{}' is not an array", parent.type_name())))
}

#[inline]
/// Inserts a value/table/aot payload into a TOML array.
fn insert_into_value_array<'a>(
    arr: &mut Array,
    index: usize,
    value_item: Item,
    parent_path: &str,
) -> Result<(), TomlEditJsError<'a>> {
    match value_item {
        Item::None => {
            remove_array_item_and_fix_format(arr, index);
            Ok(())
        }
        Item::Value(value) => {
            if index > arr.len() {
                return toml_err!(IndexOutOfBounds(index, parent_path.to_string()));
            }

            if index == arr.len() && index > 0 {
                if let Some(last) = arr.get_mut(index - 1) {
                    last.decor_mut().set_suffix("");
                }
                let (last_prefix, last_suffix) = get_value_decor(arr.get(index - 1).unwrap());
                let cleaned_suffix = clean_insert_suffix(last_suffix);
                let inserted_suffix = if !cleaned_suffix.is_empty()
                    || last_prefix.contains('\n')
                    || last_suffix.contains('\n')
                {
                    "\n"
                } else {
                    ""
                };
                arr.insert_formatted(
                    index,
                    value.decorated(clean_insert_prefix(last_prefix), inserted_suffix),
                );
                return Ok(());
            }

            let (prefix, suffix) = get_array_decor(arr);
            arr.insert_formatted(index, value.decorated(prefix, suffix));
            Ok(())
        }
        Item::Table(table) => {
            arr.insert(index, table.into_inline_table());
            Ok(())
        }
        Item::ArrayOfTables(aot) => {
            arr.insert(index, aot.into_array());
            Ok(())
        }
    }
}

#[inline]
/// Inserts a table-compatible value into an array-of-tables.
fn insert_into_aot<'a>(
    aot: &mut toml_edit::ArrayOfTables,
    index: usize,
    value_item: Item,
    parent_path: &str,
) -> Result<(), TomlEditJsError<'a>> {
    if index > aot.len() {
        return toml_err!(IndexOutOfBounds(index, parent_path.to_string()));
    }

    match value_item {
        Item::Table(table) => {
            aot.push(table);
            Ok(())
        }
        Item::Value(Value::InlineTable(inline_table)) => {
            aot.push(inline_table.into_table());
            Ok(())
        }
        _ => toml_err!(TypeError(format!(
            "cannot insert {} into array of tables at index {}",
            value_item.type_name(),
            index
        ))),
    }
}

#[inline]
/// Replaces an existing array-of-tables element.
fn replace_aot_item<'a>(
    aot: &mut toml_edit::ArrayOfTables,
    index: usize,
    value_item: Item,
    parent_path: &str,
) -> Result<(), TomlEditJsError<'a>> {
    if let Some(table) = aot.get_mut(index) {
        match value_item {
            Item::Table(value_table) => *table = value_table,
            Item::Value(Value::InlineTable(inline_table)) => *table = inline_table.into_table(),
            _ => {
                return toml_err!(TypeError(format!(
                    "cannot set non-table value into array of tables at index {}",
                    index
                )));
            }
        }
        return Ok(());
    }

    toml_err!(IndexOutOfBounds(index, parent_path.to_string()))
}

#[inline]
/// Removes an existing array element from array or array-of-tables parent.
fn remove_existing_array_item(parent: &mut Item, index: usize) {
    if let Item::Value(Value::Array(arr)) = parent {
        remove_array_item_and_fix_format(arr, index);
    } else if let Item::ArrayOfTables(aot) = parent {
        aot.remove(index);
    }
}

#[inline]
/// Removes one array item and normalizes single-element trailing decor.
fn remove_array_item_and_fix_format(arr: &mut Array, index: usize) {
    arr.remove(index);
    if arr.len() != 1 {
        return;
    }

    if let Some(first) = arr.get_mut(0) {
        let (prefix, _) = get_value_decor(first);
        let suffix = if prefix.contains('\n') { "\n" } else { "" };
        *first = first.clone().decorated(prefix, suffix);
    }
}

#[inline]
/// Replaces an existing array item while preserving existing decor when possible.
fn replace_existing_array_item(parent: &mut Item, index: usize, value_item: Item) {
    if let Item::Value(existing) = &parent[index] {
        parent[index] = preserve_existing_value_decor(existing, value_item);
    } else {
        parent[index] = value_item;
    }
}

#[inline]
/// Applies existing value decor to replacement items.
fn preserve_existing_value_decor(existing: &Value, value_item: Item) -> Item {
    let (prefix, suffix) = get_value_decor(existing);
    if let Item::Value(value) = value_item {
        Item::Value(value.decorated(prefix, suffix))
    } else {
        value_item
    }
}

#[inline]
/// Inserts or updates a key in a table-like item.
///
/// Behavior differs for inline table vs standard table to prevent decor drift.
fn insert_tablelike<'a>(
    table: &mut (dyn TableLike + 'a),
    key: &str,
    value: Item,
    is_inline_table: bool,
) {
    if table.is_empty() {
        table.insert(key, value);
        return;
    }

    if table.get(key).is_some() {
        update_existing_tablelike_key(table, key, value, is_inline_table);
        return;
    }

    insert_new_tablelike_key(table, key, value, is_inline_table);
}

#[inline]
/// Updates an existing table-like key in place.
fn update_existing_tablelike_key<'a>(
    table: &mut (dyn TableLike + 'a),
    key: &str,
    value: Item,
    is_inline_table: bool,
) {
    if let Some((mut pre_key, pre_value)) = table.get_key_value_mut(key) {
        let value_is_none = value.is_none();
        let (prefix, suffix) = get_item_decor(pre_value);
        if let Item::Value(value) = value {
            *pre_value = Item::Value(value.decorated(prefix, suffix));
        } else if value_is_none {
            if is_inline_table {
                remove_inline_table_key_and_clean_comments(table, key);
            } else {
                if let Item::Value(value) = pre_value {
                    value.decor_mut().clear();
                }
                pre_key.leaf_decor_mut().clear();
                pre_key.dotted_decor_mut().clear();
                *pre_value = Item::None;
            }
        } else if value.is_table() && !pre_value.is_table() {
            // remove space before equal sign
            pre_key.leaf_decor_mut().set_suffix("");
            *pre_value = value;
        } else {
            *pre_value = value;
        }
    }
}

#[inline]
/// Removes an inline table key and strips comment fragments from following keys.
fn remove_inline_table_key_and_clean_comments<'a>(table: &mut (dyn TableLike + 'a), key: &str) {
    table.remove(key);
    let keys: Vec<String> = table.iter().map(|(k, _)| k.to_string()).collect();
    for key in keys {
        if let Some(mut key_mut) = table.key_mut(&key) {
            let origin_prefix =
                key_mut.leaf_decor().prefix().and_then(|i| i.as_str()).unwrap_or("").to_string();
            key_mut.leaf_decor_mut().set_prefix(strip_leading_inline_comment(&origin_prefix));
        }
    }
}

#[inline]
/// Inserts a new key for non-existing table-like entry.
fn insert_new_tablelike_key<'a>(
    table: &mut (dyn TableLike + 'a),
    key: &str,
    value: Item,
    is_inline_table: bool,
) {
    if let Item::Value(value) = value {
        insert_new_tablelike_value(table, key, value, is_inline_table);
    } else {
        table.insert(key, value);
    }
}

#[inline]
/// Dispatches insertion strategy for value payloads in table-like items.
fn insert_new_tablelike_value<'a>(
    table: &mut (dyn TableLike + 'a),
    key: &str,
    value: Value,
    is_inline_table: bool,
) {
    // High-level dispatcher only:
    // 1) collect context from existing entries
    // 2) route to inline/non-inline insertion strategy
    let ctx = collect_tablelike_insert_context(table, key);
    if !is_inline_table {
        insert_non_inline_tablelike_value(table, key, value, &ctx);
        return;
    }
    insert_inline_tablelike_value(table, key, value, &ctx);
}

/// Snapshot of decor and ordering context used during table-like insertion.
struct TablelikeInsertContext {
    // Relative position of the new key against first/last existing keys.
    first_cmp: Ordering,
    last_cmp: Ordering,

    // Decor data from the current last key, used by inline-tail insertion.
    last_key_name: String,
    last_key_prefix: String,
    last_key_suffix: String,

    // Value decor anchors reused to preserve local formatting style.
    first_prefix: String,
    first_suffix: String,
    last_prefix: String,
    last_suffix: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Relative insertion position for new inline-table keys.
enum InlineInsertPosition {
    First,
    Middle,
    Last,
}

#[inline]
/// Collects insertion anchors from first/last keys and values.
fn collect_tablelike_insert_context(
    table: &mut dyn TableLike,
    key: &str,
) -> TablelikeInsertContext {
    // Invariant: this function is called only when table is non-empty.
    let values = table.get_values();
    let first = values.first().unwrap();
    let last = values.last().unwrap();
    let (first_cmp, last_cmp) = (
        first.0.first().unwrap().cmp(&&Key::new(key)),
        last.0.first().unwrap().cmp(&&Key::new(key)),
    );
    let (last_key_name, last_key_prefix, last_key_suffix) = table
        .iter()
        .last()
        .map(|(last_key, _)| {
            let key_decor = table.key(last_key).unwrap().leaf_decor();
            (
                last_key.to_string(),
                key_decor.prefix().and_then(|i| i.as_str()).unwrap_or("").to_string(),
                key_decor.suffix().and_then(|i| i.as_str()).unwrap_or(" ").to_string(),
            )
        })
        .unwrap_or_else(|| ("".to_string(), "".to_string(), " ".to_string()));
    let (first_prefix, first_suffix) = get_value_decor(first.1);
    let (last_prefix, last_suffix) = get_value_decor(last.1);

    TablelikeInsertContext {
        first_cmp,
        last_cmp,
        last_key_name,
        last_key_prefix,
        last_key_suffix,
        first_prefix: first_prefix.to_string(),
        first_suffix: first_suffix.to_string(),
        last_prefix: last_prefix.to_string(),
        last_suffix: last_suffix.to_string(),
    }
}

#[inline]
/// Maps key ordering comparisons to inline insertion position.
fn decide_inline_insert_position(first_cmp: Ordering, last_cmp: Ordering) -> InlineInsertPosition {
    // Keep the exact legacy ordering behavior:
    // (Less, Less) => append at tail
    // (Less, _)    => insert in middle
    // otherwise    => insert at head
    match (first_cmp, last_cmp) {
        (Less, Less) => InlineInsertPosition::Last,
        (Less, _) => InlineInsertPosition::Middle,
        _ => InlineInsertPosition::First,
    }
}

#[inline]
/// Inserts into a normal table while preserving tail formatting style.
fn insert_non_inline_tablelike_value(
    table: &mut dyn TableLike,
    key: &str,
    value: Value,
    ctx: &TablelikeInsertContext,
) {
    // Non-inline insertion reuses tail value prefix and a cleaned tail suffix
    // to keep formatting consistent with neighboring entries.
    table.insert(
        key,
        Item::Value(value.decorated(&ctx.last_prefix, clean_insert_suffix(&ctx.last_suffix))),
    );
}

#[inline]
/// Inserts into an inline table with position-sensitive decor strategy.
fn insert_inline_tablelike_value(
    table: &mut dyn TableLike,
    key: &str,
    value: Value,
    ctx: &TablelikeInsertContext,
) {
    // Inline insertion has position-specific decor policies.
    match decide_inline_insert_position(ctx.first_cmp, ctx.last_cmp) {
        InlineInsertPosition::Last => insert_inline_tablelike_last(table, key, value, ctx),
        InlineInsertPosition::Middle => {
            table.insert(key, Item::Value(value.decorated(&ctx.last_prefix, &ctx.first_suffix)));
        }
        InlineInsertPosition::First => {
            table.insert(key, Item::Value(value.decorated(&ctx.first_prefix, &ctx.first_suffix)));
        }
    }
}

#[inline]
/// Appends into inline table tail and normalizes key/value decor.
fn insert_inline_tablelike_last(
    table: &mut dyn TableLike,
    key: &str,
    value: Value,
    ctx: &TablelikeInsertContext,
) {
    // Tail insertion needs four ordered steps to avoid decor drift:
    // 1) clear suffix on previous tail value
    // 2) insert new value with cleaned tail decor
    // 3) copy tail key decor to new key
    // 4) normalize new value decor from old tail value
    if let Some(Item::Value(last_item)) = table.get_mut(&ctx.last_key_name) {
        last_item.decor_mut().set_suffix("");
    }

    table.insert(
        key,
        Item::Value(value.decorated(
            clean_insert_prefix(&ctx.last_prefix),
            clean_insert_suffix(&ctx.last_suffix),
        )),
    );

    if let Some(mut inserted_key) = table.key_mut(key) {
        inserted_key.leaf_decor_mut().set_prefix(clean_insert_prefix(&ctx.last_key_prefix));
        inserted_key.leaf_decor_mut().set_suffix(&ctx.last_key_suffix);
    }

    if let Some(Item::Value(inserted_value)) = table.get_mut(key) {
        inserted_value.decor_mut().set_prefix(clean_insert_prefix(&ctx.last_prefix));
        inserted_value.decor_mut().set_suffix(clean_insert_suffix(&ctx.last_suffix));
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering::{Equal, Greater, Less};

    use super::{
        InlineInsertPosition, decide_inline_insert_position, handle_array_path, handle_tablelike_path,
        insert_into_aot, insert_into_value_array, preserve_existing_value_decor, replace_aot_item,
        remove_existing_array_item, replace_existing_array_item, update_existing_tablelike_key,
    };
    use toml_edit::{Array, ArrayOfTables, Formatted, Item, Table, Value, value};

    #[test]
    fn decide_inline_position_last() {
        assert!(matches!(decide_inline_insert_position(Less, Less), InlineInsertPosition::Last));
    }

    #[test]
    fn decide_inline_position_middle() {
        assert!(matches!(
            decide_inline_insert_position(Less, Greater),
            InlineInsertPosition::Middle
        ));
    }

    #[test]
    fn decide_inline_position_first() {
        assert!(matches!(
            decide_inline_insert_position(Equal, Greater),
            InlineInsertPosition::First
        ));
    }

    #[test]
    fn insert_into_value_array_rejects_out_of_bounds_index() {
        let mut arr = Array::from_iter([Value::Integer(Formatted::new(1))]);
        let result = insert_into_value_array(
            &mut arr,
            3,
            Item::Value(Value::Integer(Formatted::new(2))),
            "foo.items",
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("index out of boundary"));
    }

    #[test]
    fn insert_into_value_array_converts_table_to_inline_table() {
        let mut arr = Array::new();
        let mut table = Table::new();
        table.insert("name", value("tom"));

        let result = insert_into_value_array(&mut arr, 0, Item::Table(table), "foo.items");
        assert!(result.is_ok());
        assert_eq!(arr.len(), 1);
        assert!(matches!(arr.get(0), Some(Value::InlineTable(_))));
    }

    #[test]
    fn insert_into_aot_rejects_non_table_value() {
        let mut aot = ArrayOfTables::new();
        let result = insert_into_aot(
            &mut aot,
            0,
            Item::Value(Value::Integer(Formatted::new(1))),
            "foo.items",
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot insert"));
    }

    #[test]
    fn replace_aot_item_accepts_inline_table_value() {
        let mut aot = ArrayOfTables::new();
        let mut old = Table::new();
        old.insert("name", value("old"));
        aot.push(old);

        let mut new_table = Table::new();
        new_table.insert("name", value("new"));
        let inline = new_table.into_inline_table();

        let result = replace_aot_item(&mut aot, 0, Item::Value(Value::InlineTable(inline)), "foo.aot");
        assert!(result.is_ok());
        assert_eq!(aot.get(0).and_then(|t| t.get("name")).and_then(Item::as_str), Some("new"));
    }

    #[test]
    fn handle_tablelike_path_rejects_non_table_parent() {
        let mut parent = Item::Value(Value::Integer(Formatted::new(1)));
        let result = handle_tablelike_path(&mut parent, "x", value("ok"));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("invalid key"));
    }

    #[test]
    fn preserve_existing_value_decor_applies_to_replacement_value() {
        let mut existing = Value::Integer(Formatted::new(1));
        existing.decor_mut().set_prefix(" ");
        existing.decor_mut().set_suffix("\n");

        let replaced = preserve_existing_value_decor(&existing, value(2));
        match replaced {
            Item::Value(v) => {
                assert_eq!(v.decor().prefix().and_then(|d| d.as_str()), Some(" "));
                assert_eq!(v.decor().suffix().and_then(|d| d.as_str()), Some("\n"));
                assert_eq!(v.as_integer(), Some(2));
            }
            _ => panic!("expected Item::Value"),
        }
    }

    #[test]
    fn handle_array_path_removes_existing_item_on_none() {
        let mut arr = Array::new();
        arr.push(1);
        arr.push(2);
        let mut parent = Item::Value(Value::Array(arr));

        let result = handle_array_path(&mut parent, 0, Item::None, "foo.arr");
        assert!(result.is_ok());
        assert_eq!(parent.as_array().map(Array::len), Some(1));
        assert_eq!(parent[0].as_integer(), Some(2));
    }

    #[test]
    fn insert_into_aot_rejects_out_of_bounds_index() {
        let mut aot = ArrayOfTables::new();
        let result = insert_into_aot(&mut aot, 1, Item::Table(Table::new()), "foo.aot");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("index out of boundary"));
    }

    #[test]
    fn replace_aot_item_rejects_out_of_bounds_index() {
        let mut aot = ArrayOfTables::new();
        let result = replace_aot_item(&mut aot, 0, Item::Table(Table::new()), "foo.aot");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("index out of boundary"));
    }

    #[test]
    fn replace_existing_array_item_updates_plain_value() {
        let mut arr = Array::new();
        arr.push(1);
        let mut parent = Item::Value(Value::Array(arr));
        replace_existing_array_item(&mut parent, 0, value(2));
        assert_eq!(parent[0].as_integer(), Some(2));
    }

    #[test]
    fn remove_existing_array_item_handles_aot() {
        let mut aot = ArrayOfTables::new();
        let mut t1 = Table::new();
        t1.insert("id", value(1));
        let mut t2 = Table::new();
        t2.insert("id", value(2));
        aot.push(t1);
        aot.push(t2);
        let mut parent = Item::ArrayOfTables(aot);

        remove_existing_array_item(&mut parent, 0);

        let aot = parent.as_array_of_tables().unwrap();
        assert_eq!(aot.len(), 1);
        assert_eq!(aot.get(0).and_then(|t| t.get("id")).and_then(Item::as_integer), Some(2));
    }

    #[test]
    fn update_existing_tablelike_key_with_none_marks_item_none_for_non_inline_table() {
        let mut table = Table::new();
        table.insert("name", value("tom"));
        update_existing_tablelike_key(&mut table, "name", Item::None, false);
        assert!(table.get("name").is_none());
    }

    #[test]
    fn handle_tablelike_path_sets_value_for_table() {
        let mut parent = Item::Table(Table::new());
        let result = handle_tablelike_path(&mut parent, "name", value("new"));
        assert!(result.is_ok());
        assert_eq!(parent["name"].as_str(), Some("new"));
    }

}
