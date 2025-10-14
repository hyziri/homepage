/// Converts text seprated by - or _ to spaces with capitalization for titles
pub fn format_title(segment: &str) -> String {
    // Replace common separators with spaces, then title-case each word.
    let normalized = segment.replace('-', " ").replace('_', " ");
    normalized
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let first = first.to_uppercase().collect::<String>();
                    let rest = chars.as_str().to_lowercase();
                    first + &rest
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
