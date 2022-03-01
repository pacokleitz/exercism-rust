pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c == '-' || c == '_' || c.is_whitespace())
        .flat_map(|word| word.chars().take(1))
        .collect::<String>()
        .to_uppercase()
}
