use std::collections::VecDeque;
use nom::{IResult};
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::space0;
use nom::character::complete::u32 as u32_parser;
use nom::multi::{many1, many_till};
use nom::sequence::Tuple;
use nom::sequence::delimited;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let sum = input.lines()
        .map(parse)
        .map(|g| g.points())
        .sum();

    Some(sum)
}

fn parse(line: &str) -> Game {
    let (_, (_, _, (win, _), cards)) = parse_nums(line).unwrap();
    Game {
        win,
        cards,
    }
}

fn parse_nums(input: &str) -> IResult<&str, (&str, &str, (Vec<u32>, &str), Vec<u32>)> {
    let until_num = take_until(":");
    let column = tag(":");
    let numbers1 = many_till(delimited(space0, u32_parser, space0), tag("|"));
    let numbers2 = many1(delimited(space0, u32_parser, space0));
    (until_num, column, numbers1, numbers2).parse(input)
}

#[derive(Debug)]
struct Game {
    win: Vec<u32>,
    cards: Vec<u32>,
}

impl Game {
    pub fn points(&self) -> u32 {
        let points = self.cards.iter()
            .fold(Points::default(), |acc, card| acc.add(*card, &self.win));
        points.value
    }

    pub fn copies(&self) -> u32 {
        self.cards
            .iter()
            .filter(|c| self.win.contains(*c))
            .count() as u32
    }
}

#[derive(Default)]
struct Points {
    value: u32,
}

impl Points {
    pub fn add(mut self, card: u32, win: &[u32]) -> Points {
        if win.contains(&card) {
            if self.value > 0 {
                self.value *= 2;
            } else {
                self.value += 1;
            }
        }
        self
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let counter = input.lines()
        .map(parse)
        .fold(CopyCounter::default(), |mut acc, g| {
            let current = acc.copies.pop_front().unwrap_or(0u32);
            let current = current + 1;
            acc.count += current;
            let copies = g.copies();
            for i in 0..copies {
                if i < acc.copies.len() as u32 {
                    acc.copies[i as usize] += current;
                } else {
                    acc.copies.push_back(current);
                }
            }
            acc
        });
    Some(counter.count)
}

#[derive(Default, Debug)]
struct CopyCounter {
    count: u32,
    copies: VecDeque<u32>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
