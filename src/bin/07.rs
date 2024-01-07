use std::cmp::Ordering;
use std::collections::{HashMap};


use itertools::Itertools;

use advent_of_code::utils::to_digits;
advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let vec = input.lines()
        .filter_map(|l| l.split_once(' '))
        .map(|(hand, bid)| (ParsedHand::new_v1(hand), to_digits(bid).unwrap_or_else(|| panic!("bid: {} is digits", bid))))
        .sorted_by(|(h1, _), (h2, _)| cmp(h1, h2, ParsedHand::input_positions_v1))
        // .inspect(|it| eprintln!("it = {:?}", it))
        .collect_vec();
    let sum = calc_result(vec);
    Some(sum)
}

#[derive(Debug, Clone)]
struct ParsedHand {
    input: String,
    kind: Kind,
}

fn cmp(h1: &ParsedHand, h2: &ParsedHand, positions_func: fn(h: &ParsedHand) -> Vec<usize>) -> Ordering {
    let positions = positions_func(h1);
    let other_input_positions = positions_func(h2);
    h1.kind.cmp(&h2.kind)
        .then(positions.cmp(&other_input_positions).reverse())
}

impl ParsedHand {
    pub fn new_v1(hand: &str) -> Self {
        Self {
            input: hand.to_owned(),
            kind: Kind::new_v1(hand),
        }
    }

    pub fn new_v2(hand: &str) -> Self {
        Self {
            input: hand.to_owned(),
            kind: Kind::new_v2(hand),
        }
    }


    pub fn input_positions_v1(&self) -> Vec<usize> {
        self.input_positions(find_char_pos_v1)
    }

    pub fn input_positions_v2(&self) -> Vec<usize> {
        self.input_positions(find_char_pos_v2)
    }

    pub fn input_positions(&self, positions_fn: fn(char) -> usize) -> Vec<usize> {
        self.input.chars()
            .map(positions_fn)
            .collect_vec()
    }
}

const LABELS: [char; 13] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Copy, Clone)]
enum Kind {
    High,
    Pair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

fn find_char_pos_v1(symbol: char) -> usize {
    find_char_pos(&LABELS, symbol)
}

fn find_char_pos_v2(symbol: char) -> usize {
    find_char_pos(&NEW_LABELS, symbol)
}

fn find_char_pos(labels: &[char], symbol: char) -> usize {
    labels.iter().find_position(|c| **c == symbol)
        .map(|o| o.0)
        .expect("found label")
}

impl Kind {
    pub fn new_v1(hand: &str) -> Self {
        let doubles = hand.chars()
            .fold(HashMap::new(), |mut acc, c| {
                let x = acc.entry(c).or_insert(0);
                *x += 1;
                acc
            });
        let doubles = doubles.into_iter()
            .map(|(value, count)| (value, count))
            .sorted_by(|(card1, count1), (card2, count2)| {
                count1.cmp(count2)
                    .then(find_char_pos_v1(*card1)
                        .cmp(&find_char_pos_v1(*card2))
                        .reverse())
            })
            .rev()
            .collect_vec();
        Self::find_kind(hand, doubles).unwrap_or_else(|| panic!("expected kind for hand {}", hand))
    }

    fn find_kind(hand: &str, doubles: Vec<(char, i32)>) -> Option<Kind> {
        let mut iter = doubles.into_iter();
        if let Some((_card, count)) = iter.next() {
            let kind = match count {
                5 => Kind::Five,
                4 => Kind::Four,
                3 => {
                    if let Some((_next_card, next_count)) = iter.next() {
                        if next_count == 2 {
                            Kind::FullHouse
                        } else {
                            Kind::Three
                        }
                    } else {
                        Kind::Three
                    }
                }
                2 => {
                    if let Some((_next_card, next_count)) = iter.next() {
                        if next_count == 2 {
                            Kind::TwoPair
                        } else {
                            Kind::Pair
                        }
                    } else {
                        Kind::Pair
                    }
                }
                1 => Kind::High,
                _ => panic!("couldn't parse {}", hand)
            };
            Some(kind)
        } else {
            None
        }
    }

    pub fn new_v2(hand: &str) -> Self {
        let (doubles, j_count) = hand.chars()
            .fold((HashMap::new(), 0), |mut acc, c| {
                if c == 'J' {
                    acc.1 += 1
                } else {
                    let x = acc.0.entry(c).or_insert(0);
                    *x += 1;
                }
                acc
            });
        let doubles = doubles.into_iter()
            .map(|(value, count)| (value, count))
            .sorted_by(|(card1, count1), (card2, count2)| {
                count1.cmp(count2)
                    .then(find_char_pos_v2(*card1)
                        .cmp(&find_char_pos_v2(*card2))
                        .reverse())
            })
            .rev()
            .collect_vec();
        let kind = Self::find_kind(hand, doubles);
        if kind.is_none() {
            if j_count == 5 {
                return Kind::Five
            } else {
                panic!("couldn't parse {}", hand)
            }
        }
        let kind = kind.unwrap();
        if j_count == 0 {
            return kind;
        }
        match kind {
            Kind::High => match j_count {
                1 => Kind::Pair,
                2 => Kind::Three,
                3 => Kind::Four,
                4 => Kind::Five,
                _ => unreachable!()
            }
            Kind::Pair => match j_count {
                1 => Kind::Three,
                2 => Kind::Four,
                3 => Kind::Five,
                _ => unreachable!(),
            },
            Kind::TwoPair => match j_count {
                1 => Kind::FullHouse,
                _ => unreachable!(),
            },
            Kind::Three => match j_count {
                1 => Kind::Four,
                2 => Kind::Five,
                _ => unreachable!(),
            },
            Kind::Four => match j_count {
                1 => Kind::Five,
                _ => unreachable!(),
            },
            _ => kind
        }
    }
}

const NEW_LABELS: [char; 13] = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];

pub fn part_two(input: &str) -> Option<u32> {
    let vec = input.lines()
        .filter_map(|l| l.split_once(' '))
        .map(|(hand, bid)| (ParsedHand::new_v2(hand), to_digits(bid).unwrap_or_else(|| panic!("bid: {} is digits", bid))))
        // .inspect(|it| eprintln!("it = {:?}", it))
        .sorted_by(|(h1, _), (h2, _)| cmp(h1, h2, ParsedHand::input_positions_v2))
        // .inspect(|it| eprintln!("it = {:?}", it))
        .collect_vec();
    // eprintln!("vec = {:?}", vec);
    let sum = calc_result(vec);

    Some(sum)
}

fn calc_result(vec: Vec<(ParsedHand, u32)>) -> u32 {
    
    vec.into_iter()
        .map(|it| it.1)
        .enumerate()
        .map(|(pos, bid)| {
            let pos = (pos + 1) as u32;
            pos * bid
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
