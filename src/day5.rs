use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::ops::Range;

#[derive(Clone, Debug, PartialEq)]
struct Seed {
    id: usize,
    soil: usize,
    fertilizer: usize,
    water: usize,
    light: usize,
    temperature: usize,
    humidity: usize,
    location: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Maps {
    maps: HashMap<MapType, Vec<[usize; 3]>>,
}

impl Maps {
    fn new() -> Maps {
        Maps {
            maps: HashMap::new(),
        }
    }

    fn get_mapped_value(&self, map_type: MapType, i: usize) -> usize {
        let mut value = i;
        for [dest_start, src_start, len] in self.maps.get(&map_type).unwrap() {
            let src_range = *src_start..(*src_start + *len);
            if src_range.contains(&i) {
                value = dest_start + i - src_start;
            }
        }
        value
    }
    fn get_soil(&self, seed: usize) -> usize {
        self.get_mapped_value(MapType::SeedToSoil, seed)
    }
    fn get_fertilizer(&self, seed: usize) -> usize {
        self.get_mapped_value(MapType::SoilToFertilizer, self.get_soil(seed))
    }
    fn get_water(&self, seed: usize) -> usize {
        self.get_mapped_value(MapType::FertilizerToWater, self.get_fertilizer(seed))
    }
    fn get_light(&self, seed: usize) -> usize {
        self.get_mapped_value(MapType::WaterToLight, self.get_water(seed))
    }
    fn get_temperature(&self, seed: usize) -> usize {
        self.get_mapped_value(MapType::LightToTemperature, self.get_light(seed))
    }
    fn get_humidity(&self, seed: usize) -> usize {
        self.get_mapped_value(MapType::TemperatureToHumidity, self.get_temperature(seed))
    }
    fn get_location(&self, seed: usize) -> usize {
        self.get_mapped_value(MapType::HumidityToLocation, self.get_humidity(seed))
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum MapType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl MapType {
    fn from_string(s: &str) -> MapType {
        match s {
            "seed-to-soil" => MapType::SeedToSoil,
            "soil-to-fertilizer" => MapType::SoilToFertilizer,
            "fertilizer-to-water" => MapType::FertilizerToWater,
            "water-to-light" => MapType::WaterToLight,
            "light-to-temperature" => MapType::LightToTemperature,
            "temperature-to-humidity" => MapType::TemperatureToHumidity,
            "humidity-to-location" => MapType::HumidityToLocation,
            _ => unreachable!(),
        }
    }
}

fn get_maps_from_input(input: &str) -> Maps {
    // Skip the first line, which contains the seed IDs.
    let mut lines = input.lines();
    lines.next();

    let mut maps = Maps::new();

    let new_map_re = Regex::new(r"^(.*) map:$").unwrap();
    let mut current_mapping: Option<MapType> = None;
    for line in lines {
        if line.is_empty() {
            continue;
        }

        // Detect if we are starting a new map.
        if let Some(caps) = new_map_re.captures(line) {
            current_mapping = Some(MapType::from_string(&caps[1]));
            continue;
        }

        // If we are not starting a new map, we are adding to the current map.
        let map = maps
            .maps
            .entry(current_mapping.clone().unwrap())
            .or_default();
        let row = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<usize>>();
        map.push([row[0], row[1], row[2]]);
    }

    maps
}

#[aoc_generator(day5, part1)]
fn parse_input_part1(input: &str) -> (Vec<Seed>, Maps) {
    // Get the first line, which contains the seed IDs.
    let mut lines = input.lines();
    let seedline = lines.next().unwrap();
    let re = Regex::new(r"^seeds: (.*)$").unwrap();
    let caps = re.captures(seedline).unwrap();
    let seed_ids: Vec<usize> = caps[1]
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    // Populate the maps.
    let maps = get_maps_from_input(input);

    let mut seeds: Vec<Seed> = vec![];
    for seed_id in seed_ids {
        seeds.append(&mut vec![Seed {
            id: seed_id,
            soil: maps.get_soil(seed_id),
            fertilizer: maps.get_fertilizer(seed_id),
            water: maps.get_water(seed_id),
            light: maps.get_light(seed_id),
            temperature: maps.get_temperature(seed_id),
            humidity: maps.get_humidity(seed_id),
            location: maps.get_location(seed_id),
        }]);
    }

    (seeds, maps)
}

#[aoc_generator(day5, part2)]
fn parse_input_part2(input: &str) -> (Vec<Range<usize>>, Maps) {
    // Get the first line, which contains the seed IDs.
    let mut lines = input.lines();
    let seedline = lines.next().unwrap();
    let re = Regex::new(r"^seeds: (.*)$").unwrap();
    let caps = re.captures(seedline).unwrap();
    let mut seed_ranges: Vec<Range<usize>> = vec![];
    // Split the seed IDs into pairs.
    for seed_pair in caps[1]
        .split_whitespace()
        .map(|v| v.parse::<usize>().unwrap())
        .tuples::<(usize, usize)>()
    {
        let seed_range = seed_pair.0..seed_pair.0 + seed_pair.1;
        seed_ranges.push(seed_range);
    }

    // Populate the maps.
    let maps = get_maps_from_input(input);

    (seed_ranges, maps)
}

#[aoc(day5, part1)]
fn part1(input: &(Vec<Seed>, Maps)) -> usize {
    let seeds = input.0.clone();
    let seed = seeds
        .iter()
        .sorted_by(|a, b| Ord::cmp(&a.location, &b.location))
        .next()
        .unwrap();
    seed.location
}

#[aoc(day5, part2)]
fn part2(input: &(Vec<Range<usize>>, Maps)) -> usize {
    let seed_ranges = input.0.clone();
    let maps = &input.1;
    let mut closest_seed: Option<Seed> = None;
    // Loop through each seed range.
    for seed_range in seed_ranges {
        // Loop through each seed in the range.
        // Todo: Find a different way, the ranges are huge.
        for seed_id in seed_range {
            let seed = Seed {
                id: seed_id,
                soil: maps.get_soil(seed_id),
                fertilizer: maps.get_fertilizer(seed_id),
                water: maps.get_water(seed_id),
                light: maps.get_light(seed_id),
                temperature: maps.get_temperature(seed_id),
                humidity: maps.get_humidity(seed_id),
                location: maps.get_location(seed_id),
            };
            // If this is the first seed, set it as the closest seed.
            if closest_seed.is_none() {
                closest_seed = Some(seed);
                continue;
            }
            // If this seed is closer than the current closest seed, set it as the closest seed.
            if seed.location < closest_seed.clone().unwrap().location {
                closest_seed = Some(seed);
            }
        }
    }
    closest_seed.unwrap().location
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input_part1() {
        let seeds = vec![
            Seed {
                id: 79,
                soil: 81,
                fertilizer: 81,
                water: 81,
                light: 74,
                temperature: 78,
                humidity: 78,
                location: 82,
            },
            Seed {
                id: 14,
                soil: 14,
                fertilizer: 53,
                water: 49,
                light: 42,
                temperature: 42,
                humidity: 43,
                location: 43,
            },
            Seed {
                id: 55,
                soil: 57,
                fertilizer: 57,
                water: 53,
                light: 46,
                temperature: 82,
                humidity: 82,
                location: 86,
            },
            Seed {
                id: 13,
                soil: 13,
                fertilizer: 52,
                water: 41,
                light: 34,
                temperature: 34,
                humidity: 35,
                location: 35,
            },
        ];

        let mut maps = HashMap::new();
        maps.insert(MapType::SeedToSoil, vec![[50, 98, 2], [52, 50, 48]]);
        maps.insert(
            MapType::SoilToFertilizer,
            vec![[0, 15, 37], [37, 52, 2], [39, 0, 15]],
        );
        maps.insert(
            MapType::FertilizerToWater,
            vec![[49, 53, 8], [0, 11, 42], [42, 0, 7], [57, 7, 4]],
        );
        maps.insert(MapType::WaterToLight, vec![[88, 18, 7], [18, 25, 70]]);
        maps.insert(
            MapType::LightToTemperature,
            vec![[45, 77, 23], [81, 45, 19], [68, 64, 13]],
        );
        maps.insert(MapType::TemperatureToHumidity, vec![[0, 69, 1], [1, 0, 69]]);
        maps.insert(MapType::HumidityToLocation, vec![[60, 56, 37], [56, 93, 4]]);

        assert_eq!(
            (seeds, Maps { maps }),
            parse_input_part1(get_test_input_part1())
        );
    }

    #[test]
    fn test_parse_input_part2() {
        let seed_ranges = vec![79..93, 55..68];

        let mut maps = HashMap::new();
        maps.insert(MapType::SeedToSoil, vec![[50, 98, 2], [52, 50, 48]]);
        maps.insert(
            MapType::SoilToFertilizer,
            vec![[0, 15, 37], [37, 52, 2], [39, 0, 15]],
        );
        maps.insert(
            MapType::FertilizerToWater,
            vec![[49, 53, 8], [0, 11, 42], [42, 0, 7], [57, 7, 4]],
        );
        maps.insert(MapType::WaterToLight, vec![[88, 18, 7], [18, 25, 70]]);
        maps.insert(
            MapType::LightToTemperature,
            vec![[45, 77, 23], [81, 45, 19], [68, 64, 13]],
        );
        maps.insert(MapType::TemperatureToHumidity, vec![[0, 69, 1], [1, 0, 69]]);
        maps.insert(MapType::HumidityToLocation, vec![[60, 56, 37], [56, 93, 4]]);

        assert_eq!(
            (seed_ranges, Maps { maps }),
            parse_input_part2(get_test_input_part2())
        );
    }

    #[test]
    fn part1_example() {
        let input = parse_input_part1(get_test_input_part1());
        assert_eq!(35, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input_part2(get_test_input_part2());
        assert_eq!(46, part2(&input));
    }

    fn get_test_input_part1<'a>() -> &'a str {
        indoc! {"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        "}
    }

    fn get_test_input_part2<'a>() -> &'a str {
        get_test_input_part1()
    }
}
