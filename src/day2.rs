use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Handful {
    red: usize,
    green: usize,
    blue: usize,
}

impl Handful {
    fn new() -> Handful {
        Handful {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
    fn power(&self) -> usize {
        self.blue * self.red * self.green
    }
}

#[aoc_generator(day2, part1)]
fn parse_input_part1(input: &str) -> HashMap<usize, Vec<Handful>> {
    let mut games: HashMap<usize, Vec<Handful>> = HashMap::new();

    let re = Regex::new(r"^Game (\d+): (.*)$").unwrap();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let game_number = caps[1].parse().unwrap();
        let handfuls = caps[2]
            .split(';')
            .map(|handful| {
                let mut parsed_handful = Handful::new();
                for color in handful.split(", ") {
                    let re = Regex::new(r"(\d+) (\w+)").unwrap();
                    let caps = re.captures(color).unwrap();
                    let count: usize = caps[1].parse().unwrap();
                    match &caps[2] {
                        "red" => parsed_handful.red = count,
                        "green" => parsed_handful.green = count,
                        "blue" => parsed_handful.blue = count,
                        _ => unreachable!(),
                    }
                }

                parsed_handful
            })
            .collect();
        games.insert(game_number, handfuls);
    }

    games
}

#[aoc_generator(day2, part2)]
fn parse_input_part2(input: &str) -> HashMap<usize, Vec<Handful>> {
    parse_input_part1(input)
}

#[aoc(day2, part1)]
fn part1(input: &HashMap<usize, Vec<Handful>>) -> usize {
    input
        .iter()
        .filter(|(_, handfuls)| {
            let result: bool = handfuls
                .iter()
                .map(|handful| handful.blue <= 14 && handful.red <= 12 && handful.green <= 13)
                .all(|x| x);
            result
        })
        .map(|(game_number, _)| game_number)
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &HashMap<usize, Vec<Handful>>) -> usize {
    input
        .iter()
        .map(|(_, handfuls)| {
            let mut min_set = Handful::new();
            handfuls.iter().for_each(|handful| {
                if handful.red > min_set.red {
                    min_set.red = handful.red;
                }
                if handful.green > min_set.green {
                    min_set.green = handful.green;
                }
                if handful.blue > min_set.blue {
                    min_set.blue = handful.blue;
                }
            });
            min_set.power()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input_part1() {
        let mut expected: HashMap<usize, Vec<Handful>> = HashMap::new();
        expected.insert(
            1,
            vec![
                Handful {
                    red: 4,
                    green: 0,
                    blue: 3,
                },
                Handful {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
                Handful {
                    red: 0,
                    green: 2,
                    blue: 0,
                },
            ],
        );
        expected.insert(
            2,
            vec![
                Handful {
                    red: 0,
                    green: 2,
                    blue: 1,
                },
                Handful {
                    red: 1,
                    green: 3,
                    blue: 4,
                },
                Handful {
                    red: 0,
                    green: 1,
                    blue: 1,
                },
            ],
        );
        expected.insert(
            3,
            vec![
                Handful {
                    red: 20,
                    green: 8,
                    blue: 6,
                },
                Handful {
                    red: 4,
                    green: 13,
                    blue: 5,
                },
                Handful {
                    red: 1,
                    green: 5,
                    blue: 0,
                },
            ],
        );
        expected.insert(
            4,
            vec![
                Handful {
                    red: 3,
                    green: 1,
                    blue: 6,
                },
                Handful {
                    red: 6,
                    green: 3,
                    blue: 0,
                },
                Handful {
                    red: 14,
                    green: 3,
                    blue: 15,
                },
            ],
        );
        expected.insert(
            5,
            vec![
                Handful {
                    red: 6,
                    green: 3,
                    blue: 1,
                },
                Handful {
                    red: 1,
                    green: 2,
                    blue: 2,
                },
            ],
        );

        assert_eq!(expected, parse_input_part1(get_test_input_part1()));
    }

    #[test]
    fn test_parse_input_part2() {
        test_parse_input_part1();
    }

    #[test]
    fn part1_example() {
        let input = parse_input_part1(get_test_input_part1());
        assert_eq!(8, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input_part2(get_test_input_part2());
        assert_eq!(2286, part2(&input));
    }

    fn get_test_input_part1<'a>() -> &'a str {
        indoc! {"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "}
    }

    fn get_test_input_part2<'a>() -> &'a str {
        get_test_input_part1()
    }
}
