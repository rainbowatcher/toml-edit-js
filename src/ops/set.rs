use std::cmp::Ordering::Less;

use toml_edit::{Item, Key, TableLike};
use wasm_bindgen::{JsValue, throw_str};

use crate::{
    options::EditOptions,
    util::{
        array::parse_array_index,
        decoration::{get_array_decor, get_item_decor, get_value_dector},
        find::find_parent_item,
        js_value::to_item,
    },
};

pub fn set_value(
    obj: &mut Item,
    path_keys: Vec<&str>,
    value_key: &str,
    value: JsValue,
    options: &EditOptions,
) {
    let parent = find_parent_item(obj, path_keys);

    let value_item = to_item(&value, options.inline);

    // handle array
    if value_key.starts_with("[") && value_key.ends_with("]") {
        let i = parse_array_index(value_key);
        if parent.get(i).is_none() {
            let arr = parent.as_array_mut().unwrap();

            match value_item {
                Item::None => (),
                Item::Value(value) => {
                    let (prefix, suffix) = get_array_decor(arr);
                    if i > arr.len() {
                        throw_str(&format!("Index out of boundary: '{i}'"))
                    } else {
                        arr.insert_formatted(i, value.decorated(prefix, suffix))
                    }
                }
                Item::Table(table) => arr.insert(i, table.into_inline_table()),
                Item::ArrayOfTables(aot) => arr.insert(i, aot.into_array()),
            }
        } else {
            parent[i] = value_item;
        }
    // handle other
    } else if let Some(table) = parent.as_table_like_mut() {
        insert_tablelike(table, value_key, value_item);
    } else {
        throw_str(&format!("Invalid key: '{value_key}'"))
    }
}

// insert will overwrite the decoration of the original key
// When the table is empty, only write the default decoration
// When the table has values, we need to read the existing decoration and apply it to the newly written value
// When the key to be written exists, only the value should be modified without changing the key's decoration
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
    } else {
        if let Item::Value(value) = value {
            let values = table.get_values();
            let first = values.first().unwrap();
            let last = values.last().unwrap();
            let (first_prefix, first_suffix) = get_value_dector(first.1);
            let (last_prefix, last_suffix) = get_value_dector(last.1);
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
}
