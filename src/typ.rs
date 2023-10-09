pub fn escape_string(s: String) -> String {
    let mut result = String::new();
    for c in s.chars() {
        if c == '"' || c == '\\' {
            result.push('\\');
        }
        result.push(c);
    }
    result
}

pub fn escape_content(s: String) -> String {
    // https://typst.app/docs/reference/syntax/#markup
    const ESCAPE: &[char] = &[
        '*', '_', '`', '<', '>', '@', '=', '-', '+', '/', '$', '\\', '\'', '"', '~', '#',
    ];

    let mut result = String::new();
    for c in s.chars() {
        if ESCAPE.contains(&c) {
            result.push('\\');
        }
        result.push(c);
    }
    result
}
