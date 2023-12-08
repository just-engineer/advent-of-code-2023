pub fn byte_to_digit(b: u8) -> u32 {
    char::from(b).to_digit(10).unwrap_or_else(|| panic!("{} is digit", b))
}

pub fn bytes_to_digit(bytes: &[u8]) -> u32 {
    byte_to_digit(bytes[0])
}

pub fn to_digit(current: char) -> i32 {
    current.to_digit(10).expect("is number") as i32
}