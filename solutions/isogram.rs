pub fn is_isogram(s: &str) -> bool {
    let mut already_seen = HashSet::new();

    s.chars()
        .filter(|c| !c.is_whitespace() && *c != '-')
        .all(|c| already_seen.insert(c.to_ascii_lowercase()))
}
