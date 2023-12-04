use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
struct ScratchCard {
    id: usize,
    winning_numbers: Vec<u8>,
    numbers: Vec<u8>,
    copies: usize,
}

impl ScratchCard {
    fn from_puzzle_input(id: &str, winning_numbers: &str, numbers: &str) -> ScratchCard {
        ScratchCard {
            id: id.parse().unwrap(),
            winning_numbers: winning_numbers
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
            numbers: numbers
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
            copies: 1,
        }
    }
    fn get_matches(&self) -> Vec<u8> {
        self.winning_numbers
            .iter()
            .filter(|n| self.numbers.contains(n))
            .copied()
            .collect()
    }
    fn get_score(&self) -> usize {
        let matches = self.get_matches();
        if matches.is_empty() {
            return 0;
        }
        let mut score = 1;
        for _ in 1..matches.len() {
            score *= 2;
        }
        score
    }
}

#[aoc_generator(day4, part1)]
fn parse_input_part1(input: &str) -> BTreeMap<usize, ScratchCard> {
    let re = Regex::new(r"^Card +(\d+): ([\d ]+) \| ([\d ]+)$").unwrap();
    let mut scratch_cards: BTreeMap<usize, ScratchCard> = BTreeMap::new();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let scratch_card = ScratchCard::from_puzzle_input(&caps[1], &caps[2], &caps[3]);
        scratch_cards.insert(scratch_card.id, scratch_card);
    }
    scratch_cards
}

#[aoc_generator(day4, part2)]
fn parse_input_part2(input: &str) -> BTreeMap<usize, ScratchCard> {
    parse_input_part1(input)
}

#[aoc(day4, part1)]
fn part1(input: &BTreeMap<usize, ScratchCard>) -> usize {
    input.iter().map(|(_, card)| card.get_score()).sum()
}

#[aoc(day4, part2)]
fn part2(input: &BTreeMap<usize, ScratchCard>) -> usize {
    let mut scratch_cards: BTreeMap<usize, ScratchCard> = input.clone();
    for id in scratch_cards.clone().keys() {
        let card = scratch_cards.get_mut(id).unwrap();
        let matches = card.get_matches();
        if matches.is_empty() {
            continue;
        }
        for _ in 0..card.copies {
            for i in id + 1..id + matches.len() + 1 {
                let card = scratch_cards.get_mut(&i).unwrap();
                card.copies += 1;
            }
        }
    }

    scratch_cards.values().map(|c| c.copies).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input_part1() {
        let mut expected: BTreeMap<usize, ScratchCard> = BTreeMap::new();
        expected.insert(
            1,
            ScratchCard {
                id: 1,
                winning_numbers: vec![41, 48, 83, 86, 17],
                numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
                copies: 1,
            },
        );
        expected.insert(
            2,
            ScratchCard {
                id: 2,
                winning_numbers: vec![13, 32, 20, 16, 61],
                numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
                copies: 1,
            },
        );
        expected.insert(
            3,
            ScratchCard {
                id: 3,
                winning_numbers: vec![1, 21, 53, 59, 44],
                numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
                copies: 1,
            },
        );
        expected.insert(
            4,
            ScratchCard {
                id: 4,
                winning_numbers: vec![41, 92, 73, 84, 69],
                numbers: vec![59, 84, 76, 51, 58, 5, 54, 83],
                copies: 1,
            },
        );
        expected.insert(
            5,
            ScratchCard {
                id: 5,
                winning_numbers: vec![87, 83, 26, 28, 32],
                numbers: vec![88, 30, 70, 12, 93, 22, 82, 36],
                copies: 1,
            },
        );
        expected.insert(
            6,
            ScratchCard {
                id: 6,
                winning_numbers: vec![31, 18, 13, 56, 72],
                numbers: vec![74, 77, 10, 23, 35, 67, 36, 11],
                copies: 1,
            },
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
        assert_eq!(13, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input_part2(get_test_input_part2());
        assert_eq!(30, part2(&input));
    }

    fn get_test_input_part1<'a>() -> &'a str {
        indoc! {"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "}
    }

    fn get_test_input_part2<'a>() -> &'a str {
        get_test_input_part1()
    }
}
