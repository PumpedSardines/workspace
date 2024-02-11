pub fn validate_name(name: &String) -> bool {
    if name.len() == 0 {
        return false;
    }

    return name
        .chars()
        .filter(|c| c.is_ascii_alphabetic() || c == &'-')
        .count()
        == name.len();
}
