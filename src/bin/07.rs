use std::cmp::Ordering;
use std::collections::{HashMap};


use itertools::Itertools;

use advent_of_code::utils::to_digits;
advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let vec = input.lines()
        .filter_map(|l| l.split_once(' '))
        .map(|(hand, bid)| (ParsedHand::new(hand), to_digits(bid).unwrap_or_else(|| panic!("bid: {} is digits", bid))))
        // .inspect(|it| eprintln!("it = {:?}", it))
        .sorted_by_key(|(hand, _)| hand.clone())
        .inspect(|it| eprintln!("it = {:?}", it))
        .collect_vec();
    // eprintln!("vec = {:?}", vec);
    let sum = vec.into_iter()
        .map(|it| it.1)
        .enumerate()
        .map(|(pos, bid)| {
            let pos = (pos + 1) as u32;
            pos * bid
        })
        .sum();

    Some(sum)
}

#[derive(Debug, Clone)]
struct ParsedHand {
    input: String,
    kind: Kind,
}

impl Eq for ParsedHand {}

impl PartialEq<Self> for ParsedHand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd<Self> for ParsedHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ParsedHand {
    fn cmp(&self, other: &Self) -> Ordering {
        let positions = self.input_positions();
        let other_input_positions = other.input_positions();
        self.kind.cmp(&other.kind)
            .then(positions.cmp(&other_input_positions))

    }
}

impl ParsedHand {
    pub fn new(hand: &str) -> Self {
        let kind = Kind::new(hand);
        Self {
            input: hand.to_owned(),
            kind,
        }
    }

    fn input_positions(&self) -> Vec<usize> {
        self.input.chars()
            .map(|c| find_char_pos(c))
            .collect_vec()
    }
}

const LABELS: [char; 13] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];

#[derive(Debug, Eq, Copy, Clone)]
enum Kind {
    High(Hand),
    Pair(Hand),
    TwoPair(Hand, Hand),
    Tree(Hand),
    FullHouse(Hand, Hand),
    Four(Hand),
    Five(Hand),
}

impl PartialEq<Self> for Kind {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd<Self> for Kind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Kind {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Kind::High(c) => {
                match other {
                    Kind::High(o) => c.cmp(o),
                    _ => Ordering::Less
                }
            }
            Kind::Pair(c) => {
                match other {
                    Kind::High(_) => Ordering::Greater,
                    Kind::Pair(o) => c.cmp(o),
                    _ => Ordering::Less
                }
            }
            Kind::TwoPair(c1, c2) => {
                match other {
                    Kind::High(_) | Kind::Pair(_) => Ordering::Greater,
                    Kind::TwoPair(h1, h2) => Self::cmp_double((c1, c2), (h1, h2)),
                    _ => Ordering::Less
                }
            }
            Kind::Tree(c) => {
                match other {
                    Kind::High(_) | Kind::Pair(_) | Kind::TwoPair(_, _) => Ordering::Greater,
                    Kind::Tree(h) => c.cmp(h),
                    _ => Ordering::Less
                }
            }
            Kind::FullHouse(c1, c2) => {
                match other {
                    Kind::FullHouse(h1, h2) => Self::cmp_double((c1, c2), (h1, h2)),
                    Kind::Four(_) | Kind::Five(_) => Ordering::Less,
                    _ => Ordering::Greater
                }
            }
            Kind::Four(c) => {
                match other {
                    Kind::Four(h) => c.cmp(h),
                    Kind::Five(_) => Ordering::Less,
                    _ => Ordering::Greater
                }
            }
            Kind::Five(c) => {
                match other {
                    Kind::Five(h) => c.cmp(h),
                    _ => Ordering::Greater
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Hand(char);

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.find_label_pos()
            .cmp(&other.find_label_pos())
            .reverse()
    }
}

impl Hand {
    fn find_label_pos(&self) -> usize {
        find_char_pos(self.0)
    }
}

fn find_char_pos(symbol: char) -> usize {
    LABELS.iter().find_position(|c| **c == symbol)
        .map(|o| o.0)
        .expect("found label")
}

impl Kind {
    pub fn new(hand: &str) -> Self {
        let doubles = hand.chars()
            .fold(HashMap::new(), |mut acc, c| {
                let x = acc.entry(c).or_insert(0);
                *x += 1;
                acc
            });
        let doubles = doubles.into_iter()
            .map(|(value, count)| (Hand(value), count))
            .sorted_by_key(|(card, count)| (*count, *card))
            .rev()
            .collect_vec();
        eprintln!("doubles = {:?}", doubles);
        let mut iter = doubles.into_iter();
        if let Some((card, count)) = iter.next() {
            match count {
                5 => Kind::Five(card),
                4 => Kind::Four(card),
                3 => {
                    if let Some((next_card, next_count)) = iter.next() {
                        if next_count == 2 {
                            Kind::FullHouse(card, next_card)
                        } else {
                            Kind::Tree(card)
                        }
                    } else {
                        Kind::Tree(card)
                    }
                }
                2 => {
                    if let Some((next_card, next_count)) = iter.next() {
                        if next_count == 2 {
                            Kind::TwoPair(card, next_card)
                        } else {
                            Kind::Pair(card)
                        }
                    } else {
                        Kind::Pair(card)
                    }
                }
                1 => Kind::High(card),
                _ => panic!("couldn't parse {}", hand)
            }
        } else {
            panic!("couldn't parse {}", hand);
        }
    }

    fn cmp_double(c: (&Hand, &Hand), h: (&Hand, &Hand)) -> Ordering {
        (c.0.find_label_pos() + c.1.find_label_pos())
            .cmp(&(h.0.find_label_pos() + h.1.find_label_pos()))
    }
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
