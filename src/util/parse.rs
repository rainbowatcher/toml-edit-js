/// parse edit path to path_keys and value_key
#[inline]
pub fn parse_edit_path(edit_path: &str) -> (Vec<&str>, &str) {
    if edit_path.is_empty() {
        return (vec![], "");
    }

    let mut path_keys = Vec::new();
    let mut start = 0;
    let mut in_quotes = false;
    let bytes = edit_path.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b'"' {
            in_quotes = !in_quotes;
        } else if byte == b'.' && !in_quotes {
            let mut seg_start = start;
            let mut seg_end = i;

            if bytes.get(seg_start) == Some(&b'"') {
                seg_start += 1;
            }
            if seg_end > seg_start && bytes.get(seg_end - 1) == Some(&b'"') {
                seg_end -= 1;
            }

            let segment = unsafe { std::str::from_utf8_unchecked(&bytes[seg_start..seg_end]) };
            path_keys.push(segment);

            start = i + 1;
        }
    }

    let mut seg_start = start;
    let mut seg_end = bytes.len();

    if bytes.get(seg_start) == Some(&b'"') {
        seg_start += 1;
    }
    if seg_end > seg_start && bytes.get(seg_end - 1) == Some(&b'"') {
        seg_end -= 1;
    }

    let value_key = unsafe { std::str::from_utf8_unchecked(&bytes[seg_start..seg_end]) };

    (path_keys, value_key)
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
}
