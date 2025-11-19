use std::cmp::Ordering::Less;

use toml_edit::{Item, Key, TableLike, Value};
use wasm_bindgen::JsValue;

use crate::{
    core::error::TomlEditJsError,
    options::EditOptions,
    toml_err,
    util::{
        array::parse_array_index,
        decoration::{get_array_decor, get_item_decor, get_value_decor},
        find::find_parent_item,
        js_value::to_item,
    },
};

#[inline]
pub fn set_value<'a>(
    obj: &'a mut Item,
    path_keys: Vec<&'a str>,
    value_key: &'a str,
    value: &JsValue,
    options: &'a EditOptions,
) -> Result<(), TomlEditJsError<'a>> {
    let parent = find_parent_item(obj, path_keys.clone())?;

    let value_item = to_item(value, options.inline);

    // handle array
    if let Ok(i) = parse_array_index(value_key) {
        if parent.get(i).is_none() {
            if let Item::Value(Value::Array(arr)) = parent {
                match value_item {
                    Item::None => (),
                    Item::Value(value) => {
                        let (prefix, suffix) = get_array_decor(arr);
                        if i > arr.len() {
                            return toml_err!(IndexOutOfBounds(i, path_keys.join(".")));
                        } else {
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
        } else {
            parent[i] = value_item;
        }
    // handle other
    } else if let Some(table) = parent.as_table_like_mut() {
        insert_tablelike(table, value_key, value_item);
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
fn insert_tablelike<'a>(table: &mut (dyn TableLike + 'a), key: &str, value: Item) {
    if table.is_empty() {
        table.insert(key, value);
    } else if let Some((mut pre_key, pre_value)) = table.get_key_value_mut(key) {
        let (prefix, suffix) = get_item_decor(pre_value);
        if let Item::Value(value) = value {
            *pre_value = Item::Value(value.decorated(prefix, suffix));
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
        let (first_prefix, first_suffix) = get_value_decor(first.1);
        let (last_prefix, last_suffix) = get_value_decor(last.1);
        match (
            first.0.first().unwrap().cmp(&&Key::new(key)),
            last.0.first().unwrap().cmp(&&Key::new(key)),
        ) {
            // insert into last
            (Less, Less) => {
                table.insert(key, Item::Value(value.decorated(last_prefix, last_suffix)));
            }
            // insert into middle
            (Less, _) => {
                table.insert(key, Item::Value(value.decorated(last_prefix, first_suffix)));
            }
            // insert into first
            _ => {
                table.insert(key, Item::Value(value.decorated(first_prefix, first_suffix)));
            }
        }
    } else {
        table.insert(key, value);
    }
}
