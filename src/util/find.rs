use toml_edit::Item;
use wasm_bindgen::throw_str;

use crate::util::array::parse_array_index;

pub fn find_parent_item<'a>(item: &'a mut Item, path_keys: Vec<&str>) -> &'a mut Item {
    let mut current = item;

    for (idx, key) in path_keys.iter().enumerate() {
        let is_table = current.is_table_like();
        let is_array = current.is_array() || current.is_array_of_tables();
        if is_table || is_array {
            if current.get(key).is_none() {
                if key.starts_with("[") && key.ends_with("]") {
                    if current.is_array() {
                        let i = parse_array_index(key);
                        let arr = current.as_array().unwrap();
                        if i > arr.len() {
                            throw_str(&format!("Index out of boundary: '{i}'"))
                        }
                        current = &mut current[i];
                    } else {
                        let pre_key = path_keys[idx - 1];
                        throw_str(&format!("'{pre_key}' is not a array"))
                    }
                } else {
                    current[key] = toml_edit::table();
                    current = &mut current[key];
                }
            } else {
                current = &mut current[key];
            }
        } else {
            throw_str(&format!("'{key}' should be table or array"));
        }
    }
    current
}
