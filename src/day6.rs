use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn get_number_of_winners(&self) -> usize {
        let rt = self.time as f64;
        let d = self.distance;

        // distance = (race_time - charge_time) * charge_time
        // distance = race_time * charge_time - charge_time ^ 2
        // charge_time = (race_time - sqrt(race_time ^ 2 - 4 * distance)) / 2
        let ct = (rt - f64::sqrt((rt * rt) - (4 * d) as f64)) / 2.0;
        self.time - ct.floor() as usize - (ct.floor() as usize + 1)
    }
}

#[aoc_generator(day6, part1)]
fn parse_input_part1(input: &str) -> Vec<Race> {
    // Parses a line of the input into a vector of usize, skipping the first element.
    let parse_line = |line: &str| {
        line.split_whitespace()
            .skip(1)
            .map(|t| t.parse().unwrap())
            .collect::<Vec<usize>>()
    };

    // The first line contains the times, the second the distances.
    let mut lines = input.lines();
    let times = parse_line(lines.next().unwrap());
    let distances = parse_line(lines.next().unwrap());

    times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

#[aoc_generator(day6, part2)]
fn parse_input_part2(input: &str) -> Race {
    // Splits a line of input on the ':' character and returns the second element with all
    // whitespace removed as an integer.
    let parse_line = |line: &str| {
        line.split(':')
            .nth(1)
            .unwrap()
            .split_whitespace()
            .collect::<String>()
            .parse()
            .unwrap()
    };
    // The first line contains the times, the second the distances.
    let mut lines = input.lines();
    let time = parse_line(lines.next().unwrap());
    let distance = parse_line(lines.next().unwrap());
    Race { time, distance }
}

#[aoc(day6, part1)]
fn part1(input: &[Race]) -> usize {
    input.iter().map(|r| r.get_number_of_winners()).product()
}

#[aoc(day6, part2)]
fn part2(input: &Race) -> usize {
    input.get_number_of_winners()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input_part1() {
        let expected = vec![
            Race {
                time: 7,
                distance: 9,
            },
            Race {
                time: 15,
                distance: 40,
            },
            Race {
                time: 30,
                distance: 200,
            },
        ];

        assert_eq!(expected, parse_input_part1(get_test_input_part1()));
    }

    #[test]
    fn test_parse_input_part2() {
        let expected = Race {
            time: 71530,
            distance: 940200,
        };
        assert_eq!(expected, parse_input_part2(get_test_input_part2()));
    }

    #[test]
    fn part1_example() {
        let input = parse_input_part1(get_test_input_part1());
        assert_eq!(288, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input_part2(get_test_input_part2());
        assert_eq!(71503, part2(&input));
    }

    fn get_test_input_part1<'a>() -> &'a str {
        indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
        "}
    }

    fn get_test_input_part2<'a>() -> &'a str {
        get_test_input_part1()
    }
}
