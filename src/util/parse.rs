/// parse edit path to path_keys and value_key
///
/// ## Example
/// ```rs
/// let (path_keys, value_key) = parse_edit_path(r#"foo."bar.baz""#);
/// assert_eq!(path_keys, vec!["foo"]);
/// assert_eq!(value_key, "bar.baz");
///
/// let (path_keys, value_key) = parse_edit_path(r#"foo."bar.baz".[0].name"#);
/// assert_eq!(path_keys, vec!["foo", "bar.baz", "[0]"]);
/// assert_eq!(value_key, "name");
///
/// let (path_keys, value_key) = parse_edit_path(r#"foo.bar.[12]"#);
/// assert_eq!(path_keys, vec!["foo", "bar"]);
/// assert_eq!(value_key, "[12]");
/// ```
#[inline]
pub fn parse_edit_path(edit_path: &str) -> (Vec<String>, String) {
    let mut path_keys = vec![];
    let mut current_key = String::new();
    let mut quote_open = false;

    for c in edit_path.chars() {
        match c {
            '"' => {
                quote_open = !quote_open;
            }
            '.' if !quote_open => {
                if !current_key.is_empty() {
                    path_keys.push(current_key.to_owned());
                    current_key.clear();
                }
            }
            _ => current_key.push(c),
        }
    }

    let value_key = current_key;

    (path_keys, value_key)
}
