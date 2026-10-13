pub fn humanize(name: &str) -> String {
    let mut words = name.split(['_', '-']).filter(|word| !word.is_empty());

    let Some(first) = words.next() else {
        return String::new();
    };

    let mut text = first.to_lowercase();

    for word in words {
        text.push(' ');
        text.push_str(&word.to_lowercase());
    }

    let mut chars = text.chars();

    match chars.next() {
        Some(initial) => initial.to_uppercase().chain(chars).collect(),
        None => text,
    }
}
