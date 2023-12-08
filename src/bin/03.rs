use itertools::Itertools;
use advent_of_code::utils::to_digit;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse(input);
    let mut count = 0;
    let mut numbers = map.numbers;
    for x in map.symbols {
        numbers.retain(|num|
            if num.is_bounded(x.position) {
                count += num.value;
                false
            } else { true }
        )
    }
    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse(input);
    let gears = map.symbols.into_iter()
        .filter(|sym| sym.value == '*')
        .collect_vec();
    let mut count = 0;
    let numbers = map.numbers;
    for x in gears {
        let possible_gears = numbers.iter()
            .filter(|num| num.is_bounded(x.position))
            .collect_vec();
        if possible_gears.len() == 2 {
            let multiplied: i32 = possible_gears.iter()
                .map(|num| num.value)
                .reduce(|n1, n2| n1 * n2).expect("at least 2 items");
            count += multiplied;
        }
    }

    Some(count as u32)
}

fn parse(input: &str) -> EngineMap {
    let parsers: Vec<_> = input.lines()
        .enumerate()
        .map(|(row, line)| {
            let parser = line.chars()
                .enumerate()
                .fold(Parser::default(),
                      |acc, (column, value)| acc.parse(value, row, column));

            parser.end()
        }).collect();
    let mut engine_map = EngineMap::default();
    for mut x in parsers {
        engine_map.symbols.append(&mut x.0);
        engine_map.numbers.append(&mut x.1);
    }
    engine_map
}


#[derive(Default, Debug)]
struct EngineMap {
    symbols: Vec<Symbol>,
    numbers: Vec<Number>,
}

#[derive(Default, Debug)]
struct Parser {
    prev_state: State,
    prev_column: usize,
    symbols: Vec<Symbol>,
    numbers: Vec<Number>,
}

impl Parser {
    pub fn parse(mut self, current: char, row: usize, column: usize) -> Self {
        match current {
            '.' => {
                match self.prev_state {
                    State::Num(value, row, col) => {
                        let number = Number {
                            value,
                            start: (row, col),
                            stop: (row, column - 1),
                        };
                        self.numbers.push(number);
                    }
                    State::Sym(value, row, col) => {
                        let symbol = Symbol {
                            value,
                            position: (row, col),
                        };
                        self.symbols.push(symbol);
                    }
                    State::Dot => {}
                }
                self.prev_state = State::Dot;
            }
            '0'..='9' => {
                match self.prev_state {
                    State::Num(value, row, col) => {
                        let new_value = value * 10 + to_digit(current);
                        let state = State::Num(new_value, row, col);
                        self.prev_state = state
                    }
                    State::Sym(value, row, col) => {
                        let symbol = Symbol {
                            value,
                            position: (row, col),
                        };
                        self.symbols.push(symbol);
                        self.prev_state = State::Num(to_digit(current), row, column);
                    }
                    State::Dot => {
                        self.prev_state = State::Num(to_digit(current), row, column);
                    }
                }
            }
            _ => { // symbol case
                match self.prev_state {
                    State::Dot => {}
                    State::Sym(value, row, col) => {
                        self.symbols.push(Symbol { value, position: (row, col) });
                    }
                    State::Num(value, row, col) => {
                        self.numbers.push(Number { value, start: (row, col), stop: (row, column) })
                    }
                }
                self.prev_state = State::Sym(current, row, column);
            }
        }
        self.prev_column = column;
        self
    }

    pub fn end(mut self) -> (Vec<Symbol>, Vec<Number>) {
        match self.prev_state {
            State::Sym(value, row, col) => {
                self.symbols.push(Symbol { value, position: (row, col) });
            }
            State::Num(value, row, col) => {
                self.numbers.push(Number { value, start: (row, col), stop: (row, self.prev_column) })
            }
            State::Dot => {}
        }
        (self.symbols, self.numbers)
    }
}


#[derive(Debug)]
#[derive(Default)]
enum State {
    Sym(char, usize, usize),
    //value, start row, column
    #[default]
    Dot,
    Num(i32, usize, usize), // value, start row, column
}

#[derive(Debug)]
struct Symbol {
    value: char,
    position: (usize, usize),
}

#[derive(Debug)]
struct Number {
    value: i32,
    start: (usize, usize),
    stop: (usize, usize),
}

impl Number {
    pub fn is_bounded(&self, coordinate: (usize, usize)) -> bool {
        for row in self.start.0 as i32 - 1..=self.start.0 as i32 + 1 {
            for column in self.start.1 as i32 - 1..=self.stop.1 as i32 + 1 {
                if coordinate.0 as i32 == row && coordinate.1 as i32 == column {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
