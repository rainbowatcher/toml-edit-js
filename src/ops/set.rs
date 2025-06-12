use toml_edit::Item;
use wasm_bindgen::{JsValue, throw_str};

use crate::{
    options::EditOptions,
    types::item::ItemWrapper,
    util::{detect_array_decoration, find_parent_item, parse_array_index},
};

pub fn set_value(
    obj: &mut Item,
    path_keys: Vec<&str>,
    value_key: &str,
    value: JsValue,
    options: &EditOptions,
) {
    let parent = find_parent_item(obj, path_keys);

    let value_item = ItemWrapper::from_js_value(value, options);

    // handle array
    if value_key.starts_with("[") && value_key.ends_with("]") {
        let i = parse_array_index(value_key);
        if parent.get(i).is_none() {
            let arr = parent.as_array_mut().unwrap();

            match value_item.0 {
                Item::None => (),
                Item::Value(value) => {
                    let (prefix, suffix) = detect_array_decoration(arr);
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
            parent[i] = value_item.0;
        }
    // handle other
    } else if parent.is_table_like() {
        parent[value_key] = value_item.0;
    } else {
        throw_str(&format!("Invalid key: '{value_key}'"))
    }
}
