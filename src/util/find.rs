use toml_edit::{Item, Table, Value};

use crate::{core::error::TomlEditJsError, toml_err, util::array::parse_array_index};

#[inline]
pub fn find_parent_item<'a>(
    item: &'a mut Item,
    path_keys: &Vec<&str>,
) -> Result<&'a mut Item, TomlEditJsError<'a>> {
    let mut current = item;
    if path_keys.is_empty() {
        return toml_err!(EmptyKey);
    }
    if !current.is_array() && !current.is_array_of_tables() && !current.is_table_like() {
        return toml_err!(TypeError(format!("item root is not a table or array")));
    }
    for (idx, &key) in path_keys.iter().enumerate() {
        let parent_key = if idx > 0 { path_keys[idx - 1] } else { "root" };
        if let Ok(i) = parse_array_index(key) {
            if let Item::Value(Value::Array(arr)) = current {
                if i >= arr.len() {
                    return toml_err!(IndexOutOfBounds(i, path_keys.join(".")));
                }
                current = &mut current[i];
            } else if let Item::ArrayOfTables(aot) = current {
                if i >= aot.len() {
                    return toml_err!(IndexOutOfBounds(i, path_keys.join(".")));
                }
                current = &mut current[i];
            } else {
                return toml_err!(TypeError(format!("item '{parent_key}' is not an array")));
            }
        } else if let Some(table) = current.as_table_mut() {
            current = table.entry(key).or_insert(Item::Table(Table::new()));
        } else {
            return toml_err!(TypeError(format!("item '{parent_key}' is not a table")));
        }
    }
    Ok(current)
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use indoc::indoc;
    use std::str::FromStr;
    use toml_edit::{Array, DocumentMut, Item, Table, Value, value};
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_find_existing_nested_table() {
        let toml_str = indoc! {r#"
            [a]
            b = "hello"
            [a.c]
            d = 42
        "#};
        let mut doc = DocumentMut::from_str(toml_str).unwrap();
        let item = doc.as_item_mut();
        let path = vec!["a", "c"];

        let result = find_parent_item(item, &path);
        assert!(result.is_ok());

        let found_item = result.unwrap();
        assert!(found_item.is_table());
        assert_eq!(found_item["d"].as_integer(), Some(42));
    }

    #[wasm_bindgen_test]
    fn test_create_new_nested_table() {
        let mut item = Item::Table(Table::new());
        let path = vec!["a", "b", "c"];

        let result = find_parent_item(&mut item, &path);
        assert!(result.is_ok());
        let found_item = result.unwrap();
        *found_item = value("Success!");

        assert_eq!(item["a"]["b"]["c"].as_str(), Some("Success!"));
    }

    #[wasm_bindgen_test]
    fn test_create_in_existing_table() {
        let toml_str = "[a]\nkey1 = 'val1'\n";
        let mut doc = DocumentMut::from_str(toml_str).unwrap();
        let item = doc.as_item_mut();
        let path = vec!["a", "b"];

        let result = find_parent_item(item, &path);
        assert!(result.is_ok());

        let found_item = result.unwrap();
        *found_item = value(123);

        let expected_str = "[a]\nkey1 = 'val1'\nb = 123\n";
        assert_eq!(doc.to_string(), expected_str);
    }

    #[wasm_bindgen_test]
    fn test_access_existing_array_element() {
        let mut item = Item::Table(Table::new());
        let mut arr = Array::new();
        arr.push(10);
        arr.push(20);
        item["data"] = Item::Value(arr.into());

        let path = vec!["data", "[1]"];
        let result = find_parent_item(&mut item, &path);
        assert!(result.is_ok());

        let found_item = result.unwrap();
        assert_eq!(found_item.as_integer(), Some(20));

        // 也可以修改它
        *found_item = value(200);
        assert_eq!(item["data"][1].as_integer(), Some(200));
    }

    #[wasm_bindgen_test]
    fn test_access_nested_in_array_of_tables() {
        let toml_str = indoc! {r#"
            [[servers]]
            ip = "127.0.0.1"
            [[servers]]
            ip = "192.168.1.1"
        "#};
        let mut doc = DocumentMut::from_str(toml_str).unwrap();
        let item = doc.as_item_mut();
        let path = vec!["servers", "[1]", "ip"];

        let result = find_parent_item(item, &path);
        assert!(result.is_ok());

        let found_item = result.unwrap();
        assert_eq!(found_item.as_str(), Some("192.168.1.1"));
    }

    // --- Error test ---
    #[wasm_bindgen_test]
    fn test_panic_on_array_index_out_of_bounds() {
        let mut item = Item::Table(Table::new());
        item["data"] = Item::Value(Value::Array(Array::from_iter(vec![1, 2])));

        let path = vec!["data", "[13]"];
        let result = find_parent_item(&mut item, &path);
        assert!(result.is_err(), "Function should return error");
        println!("{:?}", result);
        assert!(result.err().unwrap().to_string().contains("index out of boundary"));
    }

    #[wasm_bindgen_test]
    fn test_error_on_indexing_a_table() {
        let toml_str = "[a]\nb = 1\n";
        let mut doc: DocumentMut = toml_str.parse().unwrap();
        let mut item = doc.as_item_mut();
        let path = vec!["a", "[0]"];
        let result = find_parent_item(&mut item, &path);
        assert!(result.is_err(), "Function should return error");
        assert!(result.err().unwrap().to_string().contains("is not an array"));
    }

    #[wasm_bindgen_test]
    fn test_error_on_keying_an_array() {
        let mut item = Item::Table(Table::new());
        item["data"] = Item::Value(Value::Array(Array::from_iter(vec![1, 2])));
        let path = vec!["data", "key"];
        let result = find_parent_item(&mut item, &path);
        assert!(result.is_err(), "Function should return error");
        assert!(result.err().unwrap().to_string().contains("is not a table"));
    }

    #[wasm_bindgen_test]
    fn test_error_on_keying_a_value() {
        let mut item = Item::Table(Table::new());
        item["config"] = value("enabled");
        let path = vec!["config", "timeout"];
        let result = find_parent_item(&mut item, &path);
        assert!(result.is_err(), "Function should return error");
        assert!(result.err().unwrap().to_string().contains("is not a table"));
    }

    #[wasm_bindgen_test]
    fn test_error_if_root_is_not_table() {
        let mut item = value("I am a string, not a table");
        let path = vec!["a"];
        let result = find_parent_item(&mut item, &path);
        assert!(result.is_err(), "Function should return error");
        assert!(result.err().unwrap().to_string().contains("item root is not a table or array"));
    }
}
