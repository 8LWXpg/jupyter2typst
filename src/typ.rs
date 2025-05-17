pub fn escape_string(s: &str) -> String {
	const ESCAPE: &[char] = &['\\', '"'];

	let mut result = String::new();
	for c in s.chars() {
		if ESCAPE.contains(&c) {
			result.push('\\');
		}
		result.push(c);
	}
	result
}

pub fn escape_content(s: &str) -> String {
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
