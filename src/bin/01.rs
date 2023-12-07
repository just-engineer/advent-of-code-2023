advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let answer = input.lines()
        .map(scan_line)
        .sum();
    Some(answer)
}

fn scan_line(line: &str) -> u32 {
    find_num(line.chars()) * 10 + find_num(line.chars().rev())
}

fn find_num(chars: impl Iterator<Item=char>) -> u32 {
    for x in chars {
        match x {
            '0'..='9' => return x.to_digit(10).expect("is number"),
            _ => continue
        }
    };
    panic!("not found a number")
}

pub fn part_two(input: &str) -> Option<u32> {
    let answer = input.lines()
        .map(multi_scan_line)
        .sum();
    Some(answer)
}

const NUMBERS: [(&str, u32); 9] = [
    ("one", 1u32),
    ("two", 2u32),
    ("three", 3u32),
    ("four", 4u32),
    ("five", 5u32),
    ("six", 6u32),
    ("seven", 7u32),
    ("eight", 8u32),
    ("nine", 9u32),
];

fn multi_scan_line(line: &str) -> u32 {
    let mut i = 0;
    let mut result = (None, None);
    while i < line.len() {
        for (num, value) in NUMBERS {
            if result.0.is_none() {
                result.0 = find_num_str_start(&line[i..], num, value);
            }
            if result.1.is_none() {
                result.1 = find_num_str_end(&line[..line.len() - i], num, value);
            }
            if result.0.is_some() && result.1.is_some() {
                break;
            }
        }
        if result.0.is_some() && result.1.is_some() {
            break;
        }
        i += 1;
    }
    eprintln!("line = {:?}", line);
    let i1 = result.0.expect("number is found") * 10 + result.1.expect("number is found");
    eprintln!("i1 = {:?}", i1);
    i1
}

fn find_num_str_start(slice: &str, num: &str, value: u32) -> Option<u32> {
    if slice.starts_with(num) {
        return Some(value);
    }
    try_to_digit(slice.as_bytes()[0])
}

fn find_num_str_end(slice: &str, num: &str, value: u32) -> Option<u32> {
    if slice.ends_with(num) {
        return Some(value);
    }
    try_to_digit(slice.as_bytes()[slice.len() - 1])
}

fn try_to_digit(slice: u8) -> Option<u32> {
    if slice.is_ascii_digit() {
        return char::from(slice).to_digit(10);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
