use std::error::Error;

pub fn bitwise_xor(text: &str) -> Result<String, Box<dyn Error>> {
    let result = text.chars()
        .map(|char| ((char as u8) ^ 0x1F) as char)
        .collect();

    Ok(result)
}