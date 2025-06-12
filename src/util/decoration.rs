use toml_edit::{Array, RawString};

pub fn detect_array_decoration(arr: &mut Array) -> (RawString, RawString) {
    let second_item = arr.get(1);
    let prefix =
        second_item.and_then(|i| i.decor().prefix()).unwrap_or(&RawString::from(" ")).to_owned();
    let suffix =
        second_item.and_then(|i| i.decor().suffix()).unwrap_or(&RawString::from("")).to_owned();
    (prefix, suffix)
}
