use wasm_bindgen::throw_str;

#[inline]
pub fn parse_array_index(key: &str) -> usize {
    let index = key
        .strip_prefix('[')
        .and_then(|s| s.strip_suffix(']'))
        .unwrap_or_else(|| throw_str(&format!("Invalid array index format: {key}")));

    index
        .parse::<usize>()
        .unwrap_or_else(|_| throw_str(&format!("Invalid array index: '{}'", index)))
}
