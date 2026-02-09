pub(crate) mod error;

use toml_edit::{Document, DocumentMut, Item, Value};
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

use crate::{
    core::error::TomlEditJsError,
    ops::set::set_value,
    options::{EditOptions, IEditOptions, IStringifyOptions, StringifyOptions},
    util::{
        js_value::{from_item, to_item},
        parse::parse_edit_path,
        string::remove_final_newline,
    },
};

#[wasm_bindgen(start)]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn parse(input: &str) -> Result<JsValue, JsValue> {
    match Document::parse(input) {
        Ok(doc) => Ok(from_item(doc.as_item())),
        Err(e) => Err(TomlEditJsError::ParseError(e).into()),
    }
}

#[wasm_bindgen]
pub fn stringify(input: JsValue, opts: Option<IStringifyOptions>) -> Result<String, JsValue> {
    let opts = StringifyOptions::new(opts)?;

    let value = to_item(&input, opts.inline);
    let mut text = match value {
        Item::Table(table) => DocumentMut::from(table).to_string(),
        Item::ArrayOfTables(aot) => aot.iter().fold(String::new(), |acc, x| acc + &x.to_string()),
        Item::Value(Value::InlineTable(table)) => table.into_table().to_string(),
        Item::Value(v) => v.to_string(),
        Item::None => "null".to_owned(),
    };

    if !opts.final_newline {
        remove_final_newline(&mut text)
    }
    Ok(text)
}

#[wasm_bindgen]
pub fn edit(
    input: &str,
    path: &str,
    value: &JsValue,
    opts: Option<IEditOptions>,
) -> Result<String, JsValue> {
    let mut doc: DocumentMut =
        input.parse().map_err(|e| JsValue::from(TomlEditJsError::ParseError(e)))?;

    let edit_opts = EditOptions::new(opts)?;
    let (path_keys, value_key) = parse_edit_path(path);
    set_value(doc.as_item_mut(), &path_keys, value_key, value, &edit_opts)?;

    let mut result_str = doc.to_string();
    if !edit_opts.final_newline {
        remove_final_newline(&mut result_str);
    }
    Ok(result_str)
}

#[cfg(test)]
#[allow(unused)]
mod tests {

    use indoc::indoc;
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::*;
    use web_sys::js_sys::{Object, Reflect};

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_parse() {
        let input = indoc! { r#"
            [foo]
            bar = "baz"
            "#
        };
        let result = super::parse(input);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(format!("{:?}", result), r#"JsValue(Object({"foo":{"bar":"baz"}}))"#);
    }

    #[wasm_bindgen_test]
    fn test_parse_with_escape_quotes() {
        let input = indoc! { r#"
            [foo]
            bar = "baz\""
            "#
        };
        let result = super::parse(input);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(format!("{:?}", result), r#"JsValue(Object({"foo":{"bar":"baz\""}}))"#);
    }

    #[wasm_bindgen_test]
    fn test_stringify() {
        let root = Object::new();
        let foo = Object::new();
        Reflect::set(&foo, &JsValue::from_str("bar"), &JsValue::from_str("baz")).unwrap();
        Reflect::set(&root, &JsValue::from_str("foo"), &JsValue::from(foo)).unwrap();

        let result = super::stringify(root.into(), None);
        assert!(result.is_ok());
        let result = result.unwrap();
        println!("result: {}", result);
        assert_eq!(result, "[foo]\nbar = \"baz\"\n");
    }

    #[wasm_bindgen_test]
    fn test_edit() {
        let input = indoc! { r#"
            [foo]
            bar = "baz"
            "#
        };
        let result = super::edit(input, "foo.bar", &JsValue::from_str("qux"), None);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, "[foo]\nbar = \"qux\"\n");
    }
}
