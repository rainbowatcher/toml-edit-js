use std::cmp::Ordering::Less;

use toml_edit::{Item, Key, TableLike, Value};
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

    // handle array
    if let Ok(i) = parse_array_index(value_key) {
        if parent.get(i).is_none() {
            if let Item::Value(Value::Array(arr)) = parent {
                match value_item {
                    Item::None => {
                        arr.remove(i);
                        // After removal, ensure proper formatting for remaining elements
                        if arr.len() == 1 {
                            if let Some(first) = arr.get_mut(0) {
                                let (prefix, _) = get_value_decor(first);
                                *first = first.clone().decorated(prefix, "\n");
                            }
                        }
                    }
                    Item::Value(value) => {
                        if i > arr.len() {
                            return toml_err!(IndexOutOfBounds(i, path_keys.join(".")));
                        } else if i == arr.len() && i > 0 {
                            if let Some(last) = arr.get_mut(i - 1) {
                                last.decor_mut().set_suffix("");
                            }
                            let (last_prefix, last_suffix) =
                                get_value_decor(arr.get(i - 1).unwrap());
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
                                i,
                                value.decorated(clean_insert_prefix(last_prefix), inserted_suffix),
                            )
                        } else {
                            let (prefix, suffix) = get_array_decor(arr);
                            arr.insert_formatted(i, value.decorated(prefix, suffix))
                        }
                    }
                    Item::Table(table) => arr.insert(i, table.into_inline_table()),
                    Item::ArrayOfTables(aot) => arr.insert(i, aot.into_array()),
                }
            } else if let Item::ArrayOfTables(aot) = parent {
                if i > aot.len() {
                    return toml_err!(IndexOutOfBounds(i, path_keys.join(".")));
                } else {
                    match value_item {
                        Item::Table(table) => {
                            aot.push(table);
                        }
                        Item::Value(Value::InlineTable(inline_table)) => {
                            aot.push(inline_table.into_table());
                        }
                        _ => toml_err!(TypeError(format!(
                            "cannot insert {} into array of tables at index {}",
                            value_item.type_name(),
                            i
                        )))?,
                    }
                }
            } else {
                return toml_err!(TypeError(format!(
                    "item '{}' is not an array",
                    parent.type_name()
                )));
            }
        } else if let Item::ArrayOfTables(aot) = parent {
            if let Some(table) = aot.get_mut(i) {
                match value_item {
                    Item::Table(value_table) => *table = value_table,
                    Item::Value(Value::InlineTable(inline_table)) => {
                        *table = inline_table.into_table()
                    }
                    _ => {
                        return toml_err!(TypeError(format!(
                            "cannot set non-table value into array of tables at index {}",
                            i
                        )));
                    }
                }
            } else {
                return toml_err!(IndexOutOfBounds(i, path_keys.join(".")));
            }
        } else if value_item.is_none() {
            // Handle removal of existing array element
            if let Item::Value(Value::Array(arr)) = parent {
                arr.remove(i);
                // After removal, ensure proper formatting for remaining elements
                if arr.len() == 1 {
                    if let Some(first) = arr.get_mut(0) {
                        let (prefix, _) = get_value_decor(first);
                        *first = first.clone().decorated(prefix, "\n");
                    }
                }
            } else if let Item::ArrayOfTables(aot) = parent {
                aot.remove(i);
            }
        } else if let Item::Value(existing) = &parent[i] {
            // Preserve decoration when replacing existing array element
            let (prefix, suffix) = get_value_decor(existing);
            if let Item::Value(value) = value_item {
                parent[i] = Item::Value(value.decorated(prefix, suffix));
            } else {
                parent[i] = value_item;
            }
        } else {
            parent[i] = value_item;
        }
    // handle other
    } else if {
        let is_inline_table = matches!(parent, Item::Value(Value::InlineTable(_)));
        if let Some(table) = parent.as_table_like_mut() {
            insert_tablelike(table, value_key, value_item, is_inline_table);
            true
        } else {
            false
        }
    } {
    } else {
        return toml_err!(KeyError(value_key));
    };

    Ok(())
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
    } else if let Some((mut pre_key, pre_value)) = table.get_key_value_mut(key) {
        let value_is_none = value.is_none();
        let (prefix, suffix) = get_item_decor(pre_value);
        if let Item::Value(value) = value {
            *pre_value = Item::Value(value.decorated(prefix, suffix));
        } else if value_is_none {
            if is_inline_table {
                table.remove(key);
                let keys: Vec<String> = table.iter().map(|(k, _)| k.to_string()).collect();
                for key in keys {
                    if let Some(mut key_mut) = table.key_mut(&key) {
                        let origin_prefix = key_mut
                            .leaf_decor()
                            .prefix()
                            .and_then(|i| i.as_str())
                            .unwrap_or("")
                            .to_string();
                        key_mut
                            .leaf_decor_mut()
                            .set_prefix(strip_leading_inline_comment(&origin_prefix));
                    }
                }
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
        };
    } else if let Item::Value(value) = value {
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
        let (first_prefix, first_suffix) = (first_prefix.to_string(), first_suffix.to_string());
        let (last_prefix, last_suffix) = (last_prefix.to_string(), last_suffix.to_string());
        if !is_inline_table {
            table.insert(
                key,
                Item::Value(value.decorated(&last_prefix, clean_insert_suffix(&last_suffix))),
            );
            return;
        }
        match (first_cmp, last_cmp) {
            // insert into last
            (Less, Less) => {
                if is_inline_table {
                    if let Some(Item::Value(last_item)) = table.get_mut(&last_key_name) {
                        last_item.decor_mut().set_suffix("");
                    }
                }
                table.insert(
                    key,
                    Item::Value(value.decorated(
                        clean_insert_prefix(&last_prefix),
                        clean_insert_suffix(&last_suffix),
                    )),
                );
                if is_inline_table {
                    if let Some(mut inserted_key) = table.key_mut(key) {
                        inserted_key
                            .leaf_decor_mut()
                            .set_prefix(clean_insert_prefix(&last_key_prefix));
                        inserted_key.leaf_decor_mut().set_suffix(&last_key_suffix);
                    }
                    if let Some(Item::Value(inserted_value)) = table.get_mut(key) {
                        inserted_value.decor_mut().set_prefix(clean_insert_prefix(&last_prefix));
                        inserted_value.decor_mut().set_suffix(clean_insert_suffix(&last_suffix));
                    }
                }
            }
            // insert into middle
            (Less, _) => {
                table.insert(key, Item::Value(value.decorated(&last_prefix, &first_suffix)));
            }
            // insert into first
            _ => {
                table.insert(key, Item::Value(value.decorated(&first_prefix, &first_suffix)));
            }
        }
    } else {
        table.insert(key, value);
    }
}
