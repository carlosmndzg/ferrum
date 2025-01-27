pub(crate) fn preprocess(input: &str) -> String {
    let mut output = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\u{000D}' => {
                if chars.peek() == Some(&'\u{000A}') {
                    chars.next();
                }
                output.push('\u{000A}');
            }
            '\u{000C}' => output.push('\u{000A}'),
            '\u{0000}' => output.push('\u{FFFD}'),
            _ => output.push(c),
        }
    }

    output
}
