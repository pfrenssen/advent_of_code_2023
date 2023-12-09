use crate::day9;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, PartialEq)]
struct HistoricValues {
    values: Vec<isize>,
}

impl<T> From<T> for HistoricValues
where
    T: Into<String>,
{
    fn from(line: T) -> Self {
        let line = line.into();
        let values = line
            .split(' ')
            .map(|v| v.trim().parse().unwrap())
            .collect::<Vec<_>>();
        HistoricValues { values }
    }
}

impl HistoricValues {
    fn get_sequences(&self) -> Vec<Vec<isize>> {
        let mut sequences = vec![];
        sequences.push(self.values.clone());
        let mut sequence = HistoricValues::get_next_sequence(&self.values);
        loop {
            sequences.push(sequence.clone());
            if sequence.iter().all(|v| *v == 0) {
                break;
            }
            sequence = day9::HistoricValues::get_next_sequence(&sequence);
        }

        sequences
    }

    fn get_next_sequence(sequence: &[isize]) -> Vec<isize> {
        let mut next_sequence = vec![];
        // Loop over the values in the sequence, and get the difference between the current and next
        // values.
        let len = sequence.len();
        for i in 0..len - 1 {
            let diff = sequence[i + 1] - sequence[i];
            next_sequence.push(diff);
        }

        next_sequence
    }
}
#[aoc_generator(day9, part1)]
fn parse_input_part1(input: &str) -> Vec<HistoricValues> {
    input.lines().map(|line| line.into()).collect()
}

#[aoc_generator(day9, part2)]
fn parse_input_part2(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day9, part1)]
fn part1(input: &[HistoricValues]) -> isize {
    input
        .iter()
        .flat_map(|v| {
            v.get_sequences()
                .into_iter()
                .map(|seq| *seq.last().unwrap())
        })
        .sum()
}

#[aoc(day9, part2)]
fn part2(input: &[String]) -> usize {
    
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input_part1() {
        let expected = vec![
            HistoricValues {
                values: vec![0, 3, 6, 9, 12, 15],
            },
            HistoricValues {
                values: vec![1, 3, 6, 10, 15, 21],
            },
            HistoricValues {
                values: vec![10, 13, 16, 21, 30, 45],
            },
        ];

        assert_eq!(expected, parse_input_part1(get_test_input_part1()));
    }

    #[test]
    fn test_parse_input_part2() {
        test_parse_input_part1();
    }

    #[test]
    fn part1_example() {
        let input = parse_input_part1(get_test_input_part1());
        assert_eq!(114, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input_part2(get_test_input_part2());
        assert_eq!(2, part2(&input));
    }

    fn get_test_input_part1<'a>() -> &'a str {
        indoc! {"
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "}
    }

    fn get_test_input_part2<'a>() -> &'a str {
        get_test_input_part1()
    }
}
