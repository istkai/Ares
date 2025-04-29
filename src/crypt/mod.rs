pub fn bitwise_xor(text: &str) -> Option<String> {
    let result = text
        .chars()
        .map(|char| ((char as u8) ^ 0x1F) as char)
        .collect();

    Some(result)
}

