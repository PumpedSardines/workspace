pub fn validate_name(name: &String) -> bool {
    if name.len() == 0 {
        return false;
    }

    return name
        .chars()
        .filter(|c| match c {
            'a'..='z' => true,
            'A'..='Z' => true,
            '0'..='9' => true,
            '_' => true,
            '-' => true,
            '.' => true,
            ',' => true,
            _ => false,
        })
        .count()
        == name.len();
}
