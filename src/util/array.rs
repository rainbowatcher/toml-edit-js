#[inline]
pub fn parse_array_index(key: &str) -> Result<usize, ()> {
    key.strip_prefix('[')
        .and_then(|s| s.strip_suffix(']'))
        .ok_or_else(|| ())
        .and_then(|i| i.parse::<usize>().map_err(|_| ()))
}
