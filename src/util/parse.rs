//! Edit-path parsing helpers.

/// Parses an edit path into parent segments and the terminal value key.
///
/// Quoted segments can contain dots, e.g. `foo."bar.baz"`.
#[inline]
pub fn parse_edit_path(edit_path: &str) -> (Vec<&str>, &str) {
    if edit_path.is_empty() {
        return (vec![], "");
    }

    // Estimated capacity: number of points + 1
    let estimated_capacity = edit_path.bytes().filter(|&b| b == b'.').count() + 1;
    let mut path_keys = Vec::with_capacity(estimated_capacity);

    let mut start = 0;
    let mut in_quotes = false;
    let bytes = edit_path.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        match byte {
            b'"' => in_quotes = !in_quotes,
            b'.' if !in_quotes => {
                let (seg_start, seg_end) = trim_quotes(start, i, bytes);
                let segment = &edit_path[seg_start..seg_end];
                path_keys.push(segment);
                start = i + 1;
            }
            _ => {}
        }
    }

    let (seg_start, seg_end) = trim_quotes(start, bytes.len(), bytes);
    let value_key = &edit_path[seg_start..seg_end];

    (path_keys, value_key)
}

#[inline]
/// Removes one leading/trailing quote pair from a segment slice boundary.
fn trim_quotes(start: usize, end: usize, bytes: &[u8]) -> (usize, usize) {
    let mut seg_start = start;
    let mut seg_end = end;

    if bytes.get(seg_start) == Some(&b'"') {
        seg_start += 1;
    }
    if seg_end > seg_start && bytes.get(seg_end - 1) == Some(&b'"') {
        seg_end -= 1;
    }

    (seg_start, seg_end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_edit_path() {
        let (path_keys, value_key) = parse_edit_path(r#"foo."bar.baz""#);
        assert_eq!(path_keys, vec!["foo"]);
        assert_eq!(value_key, "bar.baz");

        let (path_keys, value_key) = parse_edit_path(r#"foo."bar.baz".[0].name"#);
        assert_eq!(path_keys, vec!["foo", "bar.baz", "[0]"]);
        assert_eq!(value_key, "name");

        let (path_keys, value_key) = parse_edit_path(r#"foo.bar.[12]"#);
        assert_eq!(path_keys, vec!["foo", "bar"]);
        assert_eq!(value_key, "[12]");

        let (path_keys, value_key) = parse_edit_path(r#"foo." bar".baz"#);
        assert_eq!(path_keys, vec!["foo", " bar"]);
        assert_eq!(value_key, "baz");
    }

    #[test]
    fn test_parse_edit_path_empty_input() {
        let (path_keys, value_key) = parse_edit_path("");
        assert!(path_keys.is_empty());
        assert_eq!(value_key, "");
    }

    #[test]
    fn test_parse_edit_path_single_quoted_segment() {
        let (path_keys, value_key) = parse_edit_path(r#""foo.bar""#);
        assert!(path_keys.is_empty());
        assert_eq!(value_key, "foo.bar");
    }
}
