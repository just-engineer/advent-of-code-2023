use std::cmp::{max, min, min_by};
use std::str::FromStr;
use advent_of_code::utils::{byte_to_digit, bytes_to_digit};
advent_of_code::solution!(2);


const RULES: [u32; 3] = [12, 13, 14];

pub fn part_one(input: &str) -> Option<u32> {
    let sum = input.lines()
        .map(parse)
        .filter(|g| g.sets
            .iter()
            .all(is_valid))
        .map(|g| {
            eprintln!("g = {:?}", g);
            g.id
        })
        .sum();
    Some(sum)
}

fn is_valid(set: &GameSet) -> bool {
    set.red <= RULES[0] && set.green <= RULES[1] && set.blue <= RULES[2]
}

fn parse(line: &str) -> Game {
    let split: Vec<_> = line.split(": ")
        .collect();
    let game_id: Vec<_> = split[0].split(' ')
        .collect();
    let id = u32::from_str(game_id[1]).expect("is digit");
    let sets: Vec<_> = split[1].split(';')
        .map(|s| s.trim())
        .map(|s| s.split(','))
        .map(|split| split.map(|s| s.trim()).fold(GameSet::default(), parse_set))
        .collect();

    Game { id, sets }
}

fn parse_set(gameset: GameSet, set_part: &str) -> GameSet {
    let split: Vec<_> = set_part.split(' ')
        .collect();
    let num = u32::from_str(split[0]).expect("is digit");
    match split[1] {
        "red" => {
            GameSet {
                red: gameset.red + num,
                ..gameset
            }
        }
        "blue" => {
            GameSet {
                blue: gameset.blue + num,
                ..gameset
            }
        }
        "green" => {
            GameSet {
                green: gameset.green + num,
                ..gameset
            }
        }
        _ => panic!("parse failed {:?}", split)
    }
}


#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<GameSet>,
}

#[derive(Default, Debug)]
struct GameSet {
    red: u32,
    green: u32,
    blue: u32,
}


pub fn part_two(input: &str) -> Option<u32> {
    let sum = input.lines()
        .map(parse)
        .map(|g| find_min(g.sets))
        .map(|set| set.red * set.green * set.blue)
        .sum();

    Some(sum)
}

fn find_min(sets: Vec<GameSet>) -> GameSet {
    sets.into_iter()
        .reduce(min_set)
        .expect("gameset present")
}

fn min_set(acc: GameSet, s: GameSet) -> GameSet {
    GameSet {
        red: max(acc.red, s.red),
        green: max(acc.green, s.green),
        blue: max(acc.blue, s.blue),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
