#[inline]
/// Removes one trailing newline from text (`\n` or `\r\n`).
pub fn remove_final_newline(text: &mut String) {
    if text.ends_with('\n') {
        text.pop();
        if text.ends_with('\r') {
            text.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::remove_final_newline;

    #[test]
    fn remove_final_newline_removes_unix_newline() {
        let mut text = String::from("alpha\n");
        remove_final_newline(&mut text);
        assert_eq!(text, "alpha");
    }

    #[test]
    fn remove_final_newline_removes_windows_newline() {
        let mut text = String::from("alpha\r\n");
        remove_final_newline(&mut text);
        assert_eq!(text, "alpha");
    }

    #[test]
    fn remove_final_newline_keeps_text_without_trailing_newline() {
        let mut text = String::from("alpha");
        remove_final_newline(&mut text);
        assert_eq!(text, "alpha");
    }
}
