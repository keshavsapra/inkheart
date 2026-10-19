pub fn hash_string(input: &String) -> String {
    let digest = md5::compute(input.as_bytes());
    format!("{:x}", digest)
}

const FORBIDDEN_CHARS: &[char] = &['/', '\\', ':', '*', '?', '"', '<', '>', '|'];

pub fn sanitize_filename_component(input: &str) -> String {
    input
        .chars()
        .filter_map(|c| {
            if c.is_control() || FORBIDDEN_CHARS.contains(&c) {
                None
            } else if c.is_whitespace() {
                Some('_')
            } else if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                Some(c.to_ascii_lowercase())
            } else {
                None
            }
        })
        .collect()
}
