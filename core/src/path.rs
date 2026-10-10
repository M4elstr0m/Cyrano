pub fn sanitize_filename(name: &str) -> String {
    name.chars()
        .filter_map(|c| match c {
            ' ' | '-' | '_' => Some('_'),
            c if c.is_ascii_alphanumeric() => Some(c),
            _ => None,
        })
        .collect()
}
