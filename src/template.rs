use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day0, part1)]
fn parse_input_part1(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc_generator(day0, part2)]
fn parse_input_part2(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day0, part1)]
fn part1(input: &[String]) -> usize {
0
}

#[aoc(day0, part2)]
fn part2(input: &[String]) -> usize {
    part1(input)
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
        assert_eq!(0, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input_part2(get_test_input_part2());
        assert_eq!(0, part2(&input));
    }

    fn get_test_input_part1<'a>() -> &'a str {
        indoc! {"
            example
            input
        "}
    }

    fn get_test_input_part2<'a>() -> &'a str {
        indoc! {"
            example
            input
        "}
    }
}
