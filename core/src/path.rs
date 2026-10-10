pub fn sanitize_filename(name: &str) -> String {
    name.chars()
        .filter_map(|c| match c {
            ' ' | '-' => Some('_'),
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => None,
            c if c.is_control() => None,
            c => Some(c),
        })
        .collect()
}
