use std::convert::Into;

use js_sys::{Array as JsArr, Number, Object as JsObj};
use serde::{
  ser::{SerializeMap, SerializeSeq},
  Serialize,
};
use serde_wasm_bindgen::Serializer;
use toml_edit::{DocumentMut, Formatted, Item};
use wasm_bindgen::{prelude::wasm_bindgen, throw_str, JsValue};

struct TomlValue(toml::Value);

impl Serialize for TomlValue {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    match &self.0 {
      toml::Value::String(s) => serializer.serialize_str(s.as_str()),
      toml::Value::Integer(i) => serializer.serialize_i64(*i),
      toml::Value::Float(f) => serializer.serialize_f64(*f),
      toml::Value::Boolean(b) => serializer.serialize_bool(*b),
      toml::Value::Datetime(dt) => serializer.serialize_str(&dt.to_string()),
      toml::Value::Array(arr) => {
        let mut seq = serializer.serialize_seq(Some(arr.len()))?;
        for value in arr {
          seq.serialize_element(&TomlValue(value.to_owned()))?;
        }
        seq.end()
      }
      toml::Value::Table(table) => {
        let mut map = serializer.serialize_map(Some(table.len()))?;
        for (k, v) in table {
          map.serialize_entry(&k, &TomlValue(v.to_owned()))?;
        }
        map.end()
      }
    }
  }
}

struct JsValueWrapper(JsValue);

impl From<JsValueWrapper> for toml_edit::Value {
  fn from(val: JsValueWrapper) -> Self {
    let inn = val.0;
    match inn.js_typeof().as_string().unwrap().as_str() {
      "number" => {
        if Number::is_nan(&inn) {
          toml_edit::value(Number::NAN).as_value().unwrap().clone()
        } else if !Number::is_finite(&inn) {
          let n = if inn.as_f64().unwrap() < 0.0 {
            Number::NEGATIVE_INFINITY
          } else {
            Number::POSITIVE_INFINITY
          };
          toml_edit::value(n).as_value().unwrap().clone()
        } else if Number::is_safe_integer(&inn) {
          toml_edit::Value::Integer(Formatted::new(inn.as_f64().unwrap() as i64))
        } else {
          toml_edit::Value::Float(Formatted::new(inn.as_f64().unwrap()))
        }
      }
      "object" => {
        if inn.is_null() || inn.is_undefined() {
          throw_str("null and undefined is not supported");
        } else if inn.is_array() {
          let mut toml_arr = toml_edit::Array::new();
          for v in JsArr::from(inn.as_ref()) {
            toml_arr.push(JsValueWrapper(v));
          }
          toml_edit::Value::Array(toml_arr)
        } else {
          let mut toml_table = toml_edit::Table::new();
          let obj = JsObj::from(inn);
          for el in JsObj::entries(&obj) {
            let js_arr = JsArr::from(el.as_ref());
            let key = js_arr.get(0).as_string().unwrap();
            let val = js_arr.get(1);
            toml_table.insert(key.as_str(), Item::Value(JsValueWrapper(val).into()));
          }
          toml_edit::Value::InlineTable(toml_table.into_inline_table())
        }
      }
      "boolean" => toml_edit::Value::Boolean(Formatted::new(inn.as_bool().unwrap())),
      "string" => toml_edit::Value::String(Formatted::new(inn.as_string().unwrap())),
      _ => {
        if inn.is_bigint() {
          throw_str("bigint is not supported")
        } else {
          throw_str("not implemented")
        }
      }
    }
  }
}

#[wasm_bindgen]
pub fn str_input(input: &str) -> String {
  input.to_string()
}

#[wasm_bindgen]
pub fn parse(input: &str) -> Result<JsValue, JsValue> {
  match input.parse::<toml::Value>() {
    Ok(value) => Ok(TomlValue(value).serialize(&Serializer::json_compatible())?),
    Err(e) => throw_str(e.to_string().as_str()),
  }
}

#[wasm_bindgen]
pub fn stringify(input: JsValue) -> Result<String, JsValue> {
  if input.is_array() {
    throw_str("Array is not supported");
  }
  let obj: toml::Value = serde_wasm_bindgen::from_value(input)?;

  match toml::to_string(&obj) {
    Ok(val) => Ok(val),
    Err(e) => throw_str(e.to_string().as_str()),
  }
}

#[wasm_bindgen]
pub fn edit(input: &str, path: &str, value: JsValue) -> Result<String, JsValue> {
  let mut obj: DocumentMut = match input.parse() {
    Ok(value) => value,
    Err(e) => throw_str(e.to_string().as_str()),
  };

  set_value(obj.as_item_mut(), path, value)?;
  Ok(obj.to_string())
}

fn set_value(obj: &mut Item, path: &str, value: JsValue) -> Result<(), wasm_bindgen::JsValue> {
  let keys: Vec<&str> = path.split('.').collect();

  let mut current = obj;

  for key in &keys[..keys.len() - 1] {
    if !current.is_table() {
      throw_str("should be table");
    }

    if current.get(key).is_none() {
      current[key] = toml_edit::table();
    }

    current = &mut current[key];
  }

  if let Some(last_key) = keys.last() {
    current[last_key] = toml_edit::Item::Value(JsValueWrapper(value).into());
    Ok(())
  } else {
    throw_str("No path provided");
  }
}

#[cfg(test)]
mod tests {

  use indoc::indoc;
  use js_sys::{Object, Reflect};
  use wasm_bindgen::JsValue;
  use wasm_bindgen_test::*;

  wasm_bindgen_test_configure!(run_in_browser);

  #[wasm_bindgen_test]
  fn test_parse() {
    let input = indoc! { r#"
    [foo]
    bar = "baz"
    "#};
    let result = super::parse(input);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(
      format!("{:?}", result),
      r#"JsValue(Object({"foo":{"bar":"baz"}}))"#
    );
  }

  #[wasm_bindgen_test]
  fn test_parse_with_escape_quotes() {
    let input = indoc! { r#"
    [foo]
    bar = "baz\""
    "#};
    let result = super::parse(input);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(
      format!("{:?}", result),
      r#"JsValue(Object({"foo":{"bar":"baz\""}}))"#
    );
  }

  #[wasm_bindgen_test]
  fn test_stringify() {
    let root = Object::new();
    let foo = Object::new();
    Reflect::set(&foo, &JsValue::from_str("bar"), &JsValue::from_str("baz")).unwrap();
    Reflect::set(&root, &JsValue::from_str("foo"), &JsValue::from(foo)).unwrap();

    let result = super::stringify(root.into());
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
    "#};
    let result = super::edit(input, "foo.bar", JsValue::from_str("qux"));
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result, "[foo]\nbar = \"qux\"\n");
  }
}
