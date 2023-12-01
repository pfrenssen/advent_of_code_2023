use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day1, part1)]
fn parse_input_part1(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc_generator(day1, part2)]
fn parse_input_part2(input: &str) -> Vec<String> {
    let spelled_out_numbers: HashMap<&str, &str> = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut result: Vec<String> = vec![];

    // Loop over each line in the input.
    for line in input.lines() {
        let mut result_line = String::new();
        let mut chars = line.chars();
        while chars.clone().count() > 0 {
            // If the first character is a digit, add it to the `result_line` and strip it.
            if chars.clone().next().unwrap().is_ascii_digit() {
                result_line.push(chars.next().unwrap());
                continue;
            }

            // Check if the chars start with one of the spelled out numbers.
            for (spelled_out_number, digit) in &spelled_out_numbers {
                if chars.clone().as_str().starts_with(spelled_out_number) {
                    // Add the digit to the `result_line` and move on to the next character.
                    result_line.push_str(digit);
                    chars.next();
                    continue;
                }
            }

            // No match, add the character to the `result_line` and move on to the next one.
            result_line.push(chars.next().unwrap());
        }
        result.push(result_line);
    };

    result
}

#[aoc(day1, part1)]
fn part1(calibration_values: &[String]) -> usize {
    // Strip all characters from the start until the first digit.
    let calibration_values: Vec<String> = calibration_values
        .iter()
        .map(|s| s.chars().skip_while(|c| !c.is_ascii_digit()).collect())
        .collect();
    // Strip all characters from the end until the last digit.
    let calibration_values: Vec<String> = calibration_values
        .iter()
        .map(|s| {
            s.chars()
                .rev()
                .skip_while(|c| !c.is_ascii_digit())
                .collect::<String>()
                .chars()
                .rev()
                .collect()
        })
        .collect();
    // Remove all empty strings.
    let calibration_values: Vec<String> = calibration_values
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    // If a string is only 1 character long, duplicate it.
    let calibration_values: Vec<String> = calibration_values
        .iter()
        .map(|s| {
            if s.len() == 1 {
                s.repeat(2)
            } else {
                s.to_string()
            }
        })
        .collect();
    // Strip all characters but keep the first and last ones.
    let calibration_values: Vec<String> = calibration_values
        .iter()
        .map(|s| {
            let mut chars = s.chars();
            let first = chars.next().unwrap();
            let last = chars.last().unwrap();
            format!("{}{}", first, last)
        })
        .collect();
    // Convert the strings to integers.
    let calibration_values: Vec<usize> = calibration_values
        .iter()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    // Return the sum of all calibration values.
    calibration_values.iter().sum()
}

#[aoc(day1, part2)]
fn part2(calibration_values: &[String]) -> usize {
    part1(calibration_values)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input_part1() {
        let expected = vec![
            "1abc2".to_string(),
            "pqr3stu8vwx".to_string(),
            "a1b2c3d4e5f".to_string(),
            "treb7uchet".to_string(),
        ];

        assert_eq!(expected, parse_input_part1(get_test_input_part1()));
    }

    #[test]
    fn test_parse_input_part2() {
        let expected = vec![
            "2wo19ine".to_string(),
            "8igh2wo3hree".to_string(),
            "abc1ne23hreexyz".to_string(),
            "x2w1ne34our".to_string(),
            "49ine8ight7even2".to_string(),
            "z1n8ight234".to_string(),
            "7pqrst6ixteen".to_string(),
        ];

        assert_eq!(expected, parse_input_part2(get_test_input_part2()));
    }

    #[test]
    fn part1_example() {
        let input = parse_input_part1(get_test_input_part1());
        assert_eq!(142, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input_part2(get_test_input_part2());
        assert_eq!(281, part2(&input));
    }

    fn get_test_input_part1<'a>() -> &'a str {
        indoc! {"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "}
    }

    fn get_test_input_part2<'a>() -> &'a str {
        indoc! {"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "}
    }
}
