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
