pub fn byte_to_digit(b: u8) -> u32 {
    char::from(b).to_digit(10).unwrap_or_else(|| panic!("{} is digit", b))
}

pub fn bytes_to_digit(bytes: &[u8]) -> u32 {
    byte_to_digit(bytes[0])
}

pub fn to_digit_i32(current: char) -> i32 {
    to_digit(current) as i32
}

pub fn to_digit(current: char) -> u32 {
    current.to_digit(10).unwrap_or_else(|| panic!("expect {} is digit", current))
}

pub fn split_digits(line: &str, pattern: &str) -> Vec<u32> {
    line.split(pattern)
        .filter(|l| !l.is_empty())
        .filter_map(to_digits)
        .map(|a| a as u32)
        .collect()
}

fn to_digits(l: &str) -> Option<u32> {
    l.chars().map(to_digit).reduce(|a, b| a * 10 + b)
}
