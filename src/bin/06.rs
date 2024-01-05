use std::iter::zip;
use itertools::Itertools;
use advent_of_code::utils::{split_digits, to_digits_u64};
advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines()
        .collect_vec();
    let time = digits_from_line(lines[0], "Time:");
    let time_digits = split_digits(time, " ");
    let distance = digits_from_line(lines[1], "Distance:");
    let distance_digits = split_digits(distance, " ");
    zip(time_digits, distance_digits)
        .map(|(time, distance)| (time as u64, distance as u64))
        .map(|(time, distance)| calc_wins(time, distance))
        .reduce(|a, b| a * b)
}

fn calc_wins(time: u64, distance: u64) -> u32 {
    eprintln!("=== time: {:?} distance: {:?}", time, distance);
    let t = time as i64;
    let d = distance as i64;
    let disc = (t.pow(2) - 4 * d) as f64;
    eprintln!("disc = {:?}", disc);
    let x1 = (-t as f64 - disc.powf(0.5)) / -2f64;
    let x2 = (-t as f64 + disc.powf(0.5)) / -2f64;
    eprintln!("x1 = {:?}, x2 = {:?}", x1, x2);

    let x1i = x1.floor() as i64;
    let x2i = if x2.ceil() == x2.floor() {
        x2.floor() as i64 + 1
    } else {
        x2.floor() as i64
    };
    eprintln!("x1i = {:?}, x2i = {:?}", x1i, x2i);

    let end = min_under(t, x1i) as u32;
    let start = min_under(t, x2i) as u32;
    eprintln!("start = {:?}, end = {:?}", start, end);

    let result = end - start;
    eprintln!("result = {:?}", result);
    result
}

fn min_under(t: i64, value: i64) -> i64 {
    if value > t {
        t
    } else {
        value
    }
}

fn digits_from_line<'a>(line: &'a str, prefix: &str) -> &'a str {
    line.split_once(prefix)
        .map(|x| x.1)
        .unwrap_or_else(|| panic!("numbers after {}", prefix))
        .trim()
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines()
        .collect_vec();
    let time = digits_from_line(lines[0], "Time:");
    let time = time.replace(" ", "");
    let distance = digits_from_line(lines[1], "Distance:");
    let distance = distance.replace(" ", "");
    let time = to_digits_u64(&time).expect("time is present");
    let distance = to_digits_u64(&distance).expect("distance is present");
    eprintln!("time = {:?}", time);
    eprintln!("distance = {:?}", distance);
    let result = calc_wins(time, distance);
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
