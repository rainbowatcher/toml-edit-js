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
fn remove_existing_array_item(parent: &mut Item, index: usize) {
    if let Item::Value(Value::Array(arr)) = parent {
        remove_array_item_and_fix_format(arr, index);
    } else if let Item::ArrayOfTables(aot) = parent {
        aot.remove(index);
    }
}

#[inline]
fn remove_array_item_and_fix_format(arr: &mut Array, index: usize) {
    arr.remove(index);
    if arr.len() == 1 {
        if let Some(first) = arr.get_mut(0) {
            let (prefix, _) = get_value_decor(first);
            *first = first.clone().decorated(prefix, "\n");
        }
    }
}

#[inline]
fn replace_existing_array_item(parent: &mut Item, index: usize, value_item: Item) {
    if let Item::Value(existing) = &parent[index] {
        parent[index] = preserve_existing_value_decor(existing, value_item);
    } else {
        parent[index] = value_item;
    }
}

#[inline]
fn preserve_existing_value_decor(existing: &Value, value_item: Item) -> Item {
    let (prefix, suffix) = get_value_decor(existing);
    if let Item::Value(value) = value_item {
        Item::Value(value.decorated(prefix, suffix))
    } else {
        value_item
    }
}

// insert will overwrite the decoration of the original key
// When the table is empty, only write the default decoration
// When the table has values, we need to read the existing decoration and apply it to the newly written value
// When the key to be written exists, only the value should be modified without changing the key's decoration
#[inline]
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
fn insert_new_tablelike_value<'a>(
    table: &mut (dyn TableLike + 'a),
    key: &str,
    value: Value,
    is_inline_table: bool,
) {
    let ctx = collect_tablelike_insert_context(table, key);
    if !is_inline_table {
        insert_non_inline_tablelike_value(table, key, value, &ctx);
        return;
    }
    insert_inline_tablelike_value(table, key, value, &ctx);
}

struct TablelikeInsertContext {
    first_cmp: Ordering,
    last_cmp: Ordering,
    last_key_name: String,
    last_key_prefix: String,
    last_key_suffix: String,
    first_prefix: String,
    first_suffix: String,
    last_prefix: String,
    last_suffix: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InlineInsertPosition {
    First,
    Middle,
    Last,
}

#[inline]
fn collect_tablelike_insert_context(
    table: &mut dyn TableLike,
    key: &str,
) -> TablelikeInsertContext {
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
fn decide_inline_insert_position(first_cmp: Ordering, last_cmp: Ordering) -> InlineInsertPosition {
    match (first_cmp, last_cmp) {
        (Less, Less) => InlineInsertPosition::Last,
        (Less, _) => InlineInsertPosition::Middle,
        _ => InlineInsertPosition::First,
    }
}

#[inline]
fn insert_non_inline_tablelike_value(
    table: &mut dyn TableLike,
    key: &str,
    value: Value,
    ctx: &TablelikeInsertContext,
) {
    table.insert(
        key,
        Item::Value(value.decorated(&ctx.last_prefix, clean_insert_suffix(&ctx.last_suffix))),
    );
}

#[inline]
fn insert_inline_tablelike_value(
    table: &mut dyn TableLike,
    key: &str,
    value: Value,
    ctx: &TablelikeInsertContext,
) {
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
fn insert_inline_tablelike_last(
    table: &mut dyn TableLike,
    key: &str,
    value: Value,
    ctx: &TablelikeInsertContext,
) {
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

    use super::{InlineInsertPosition, decide_inline_insert_position};

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
}
