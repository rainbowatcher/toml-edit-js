use toml_edit::{Document, DocumentMut, Item};
use wasm_bindgen::{JsValue, prelude::wasm_bindgen, throw_str};

use crate::{
    ops::set::set_value,
    options::{EditOptions, IEditOptions, IStringifyOptions, StringifyOptions},
    types::{doc::DocumentWrapper, item::ItemWrapper},
    util::parse_edit_path,
};

#[wasm_bindgen(start)]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn parse(input: &str) -> Result<JsValue, JsValue> {
    match Document::parse(input) {
        Ok(doc) => Ok(JsValue::from(DocumentWrapper(doc))),
        Err(e) => throw_str(e.to_string().as_str()),
    }
}

#[wasm_bindgen]
pub fn stringify(input: JsValue, opts: Option<IStringifyOptions>) -> Result<String, JsValue> {
    let value = ItemWrapper::from(input);
    let str = match value.0 {
        Item::Table(table) => DocumentMut::from(table).to_string(),
        Item::ArrayOfTables(aot) => aot
            .iter()
            .map(|t| DocumentMut::from(t.to_owned()).to_string())
            .collect::<Vec<_>>()
            .join("\n"),
        Item::Value(v) => v.to_string(),
        Item::None => "null".to_owned(),
    };
    let stringify_opts = StringifyOptions::new(opts);
    if stringify_opts.final_newline {
        return Ok(str);
    }
    Ok(str.trim_end().to_string())
}

#[wasm_bindgen]
pub fn edit(
    input: &str,
    path: &str,
    value: JsValue,
    opts: Option<IEditOptions>,
) -> Result<String, JsValue> {
    let mut doc: DocumentMut = match input.parse() {
        Ok(d) => d,
        Err(e) => throw_str(e.to_string().as_str()),
    };

    let edit_opts = EditOptions::new(opts);
    let (path_keys, value_key) = parse_edit_path(path);
    set_value(
        doc.as_item_mut(),
        path_keys.iter().map(|x| &**x).collect(),
        &value_key,
        value,
        &edit_opts,
    );

    if edit_opts.final_newline {
        return Ok(doc.to_string());
    }
    Ok(doc.to_string().trim_end().to_string())
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
        let result = super::edit(input, "foo.bar", JsValue::from_str("qux"), None);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, "[foo]\nbar = \"qux\"\n");
    }
}
