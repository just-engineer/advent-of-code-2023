advent_of_code::solution!(5);
use itertools::Itertools;

use advent_of_code::utils::{split_digits};


pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines()
        .collect_vec();
    let seeds = parse_seeds(lines[0]);
    let maps = parse_maps(lines);
    let mut results = Vec::new();
    for seed in seeds {
        let mut seed = seed;
        for map_range in &maps {
            let ranges = map_range.ranges();
            seed = ranges.iter()
                .find_map(|r| r.next(seed))
                .unwrap_or(seed);
        }
        results.push(seed);
    }
    results.into_iter()
        .min()
}

fn parse_maps(lines: Vec<&str>) -> Vec<MapRange> {
    let mut maps: Vec<MapRange> = Vec::new();
    let mut current_range = MapRange::SeedSoil(Vec::new());
    for line in lines.iter().skip(3) {
        if line.is_empty() {
            continue;
        }
        if line.contains("map") {
            let old = current_range;
            current_range = old.next().expect("couldn't find variant");
            maps.push(old);
            continue;
        }
        let digits = split_digits(line, " ");
        let range = Range::new(digits[0], digits[1], digits[2]);
        current_range.add(range);
    }
    maps.push(current_range);
    maps
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines()
        .collect_vec();
    let seeds = parse_seeds_pair(lines[0]);
    let maps = parse_maps(lines);
    let mut results = Vec::new();
    for range in seeds {
        for seed in range.0..range.0 + range.1 {
            let mut seed = seed;
            for map_range in &maps {
                let ranges = map_range.ranges();
                seed = ranges.iter()
                    .find_map(|r| r.next(seed))
                    .unwrap_or(seed);
            }
            results.push(seed);
        }
    }
    results.into_iter()
        .min()
}

fn find_location(maps: &[MapRange], seed: u32) -> u32 {
    let mut result = seed;
    for map_range in maps {
        let ranges = map_range.ranges();
        result = ranges.iter()
            .find_map(|r| r.next(seed))
            .unwrap_or(seed);
    }
    result
}

fn parse_seeds(line: &str) -> Vec<u32> {
    let split = line.splitn(2, ": ").collect_vec();
    split_digits(split[1], " ")
}

fn parse_seeds_pair(line: &str) -> Vec<(u32, u32)> {
    let split = line.splitn(2, ": ").collect_vec();
    let mut digits = split_digits(split[1], " ")
        .into_iter();
    let mut result = Vec::new();
    while let Some((first, length)) = digits.next_tuple() {
        result.push((first, length))
    }
    result
}


#[derive(Debug)]
enum MapRange {
    SeedSoil(Vec<Range>),
    SoilFertilizer(Vec<Range>),
    FertilizerWater(Vec<Range>),
    WaterLight(Vec<Range>),
    LightTemperature(Vec<Range>),
    TemperatureHumidity(Vec<Range>),
    HumidityLocation(Vec<Range>),
}

impl MapRange {
    pub fn next(&self) -> Option<Self> {
        match self {
            MapRange::SeedSoil(_) => Some(MapRange::SoilFertilizer(Vec::new())),
            MapRange::SoilFertilizer(_) => Some(MapRange::FertilizerWater(Vec::new())),
            MapRange::FertilizerWater(_) => Some(MapRange::WaterLight(Vec::new())),
            MapRange::WaterLight(_) => Some(MapRange::LightTemperature(Vec::new())),
            MapRange::LightTemperature(_) => Some(MapRange::TemperatureHumidity(Vec::new())),
            MapRange::TemperatureHumidity(_) => Some(MapRange::HumidityLocation(Vec::new())),
            MapRange::HumidityLocation(_) => None
        }
    }

    pub fn add(&mut self, range: Range) {
        let vec = self.ranges_mut();
        vec.push(range);
    }

    pub fn ranges_mut(&mut self) -> &mut Vec<Range> {
        match self {
            MapRange::SeedSoil(vec) => vec,
            MapRange::SoilFertilizer(vec) => vec,
            MapRange::FertilizerWater(vec) => vec,
            MapRange::WaterLight(vec) => vec,
            MapRange::LightTemperature(vec) => vec,
            MapRange::TemperatureHumidity(vec) => vec,
            MapRange::HumidityLocation(vec) => vec,
        }
    }

    pub fn ranges(&self) -> &Vec<Range> {
        match self {
            MapRange::SeedSoil(vec) => vec,
            MapRange::SoilFertilizer(vec) => vec,
            MapRange::FertilizerWater(vec) => vec,
            MapRange::WaterLight(vec) => vec,
            MapRange::LightTemperature(vec) => vec,
            MapRange::TemperatureHumidity(vec) => vec,
            MapRange::HumidityLocation(vec) => vec,
        }
    }
}

#[derive(Debug)]
struct Range {
    dest: u32,
    source: u32,
    length: u32,
}

impl Range {
    pub fn new(dest: u32, source: u32, length: u32) -> Self {
        Self {
            dest,
            source,
            length,
        }
    }

    pub fn next(&self, item: u32) -> Option<u32> {
        let negative = item < self.source;
        if !negative {
            let target = item - self.source;
            if target < self.length {
                Some(self.dest + target)
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
