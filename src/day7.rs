use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Hand {
    cards: String,
    bid: usize,
    joker: bool,
}

impl<T> From<T> for Hand
where
    T: Into<String>,
{
    fn from(line: T) -> Self {
        let line = line.into();
        let cards = line[..5].to_string();
        let bid = line[6..].trim().parse().unwrap();
        Hand {
            cards,
            bid,
            joker: false,
        }
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let t1: Type = (*self).clone().into();
        let t2: Type = (*other).clone().into();
        if t1 > t2 {
            return Ordering::Greater;
        }
        if t1 < t2 {
            return Ordering::Less;
        }
        // Types are the same, compare the cards starting from the first.
        let hand_1_chars = self.cards.chars().collect::<Vec<_>>();
        let hand_2_chars = other.cards.chars().collect::<Vec<_>>();
        for i in 0..5 {
            let v1 = get_card_value(hand_1_chars[i], self.joker);
            let v2 = get_card_value(hand_2_chars[i], other.joker);
            match v1.cmp(&v2) {
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
                _ => (),
            }
        }
        Ordering::Equal
    }
}

#[derive(Debug, PartialEq, Ord, PartialOrd, Eq, Clone, Copy)]
enum Type {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPairs = 2,
    OnePair = 1,
    HighCard = 0,
}

impl<T> From<T> for Type
where
    T: Into<Hand>,
{
    fn from(hand: T) -> Self {
        let hand = hand.into();
        if !hand.joker {
            return get_type_for_hand(&hand.cards);
        }
        // If we have one or more jokers, substitute them for each card in the deck and get the
        // type for each hand.
        let mut highest_type = Type::HighCard;
        // Retrieve the cards in the hand, and remove duplicates.
        let mut held_cards: Vec<char> = hand.cards.chars().collect();
        held_cards.sort();
        held_cards.dedup();
        // Get a version of the hand with the jokers removed.
        let cards_with_jokers_removed =
            hand.cards.chars().filter(|c| *c != 'J').collect::<String>();

        // If there are no jokers, return the type of the hand.
        if cards_with_jokers_removed.len() == 5 {
            return get_type_for_hand(&hand.cards);
        }

        held_cards.iter().for_each(|c| {
            // If we already have a FiveOfAKind, we don't need to check any more.
            if highest_type == Type::FiveOfAKind {
                return;
            }
            let mut cards = cards_with_jokers_removed.clone();
            // Fill up to 5 cards.
            while cards.len() < 5 {
                cards.push(*c);
            }
            let t = get_type_for_hand(&cards);
            if t > highest_type {
                highest_type = t;
            }
        });

        highest_type
    }
}

fn get_type_for_hand(hand: &str) -> Type {
    let mut cards: HashMap<char, usize> = HashMap::new();
    hand.chars().for_each(|c| {
        *cards.entry(c).or_insert(0) += 1;
    });
    let mut values: Vec<usize> = cards.values().copied().collect();
    values.sort();
    match values.as_slice() {
        [1, 1, 1, 1, 1] => Type::HighCard,
        [1, 1, 1, 2] => Type::OnePair,
        [1, 2, 2] => Type::TwoPairs,
        [1, 1, 3] => Type::ThreeOfAKind,
        [2, 3] => Type::FullHouse,
        [1, 4] => Type::FourOfAKind,
        [5] => Type::FiveOfAKind,
        _ => unreachable!(),
    }
}

fn get_card_value(card: char, joker: bool) -> usize {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => match joker {
            true => 0,
            false => 11,
        },
        'T' => 10,
        _ => card.to_digit(10).unwrap_or(0) as usize,
    }
}
#[aoc_generator(day7, part1)]
fn parse_input_part1(input: &str) -> Vec<Hand> {
    input.lines().map(|line| line.into()).collect()
}

#[aoc_generator(day7, part2)]
fn parse_input_part2(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| line.into())
        .map(|mut hand: Hand| {
            hand.joker = true;
            hand
        })
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &[Hand]) -> usize {
    let mut input = input.to_vec();
    input.sort();
    input
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &[Hand]) -> usize {
    part1(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_card_into_type() {
        let tests = vec![
            ("32T4K", Type::HighCard),
            ("T5QJ5", Type::OnePair),
            ("KK677", Type::TwoPairs),
            ("KTJJJ", Type::ThreeOfAKind),
            ("QQQAA", Type::FullHouse),
            ("42444", Type::FourOfAKind),
            ("AAAAA", Type::FiveOfAKind),
        ];
        for (cards, expected) in tests {
            let hand = Hand {
                cards: cards.to_string(),
                bid: 0,
                joker: false,
            };
            assert_eq!(expected, Type::from(hand));
        }
    }

    #[test]
    fn test_card_ordering() {
        let tests = vec![
            // A HighCard to a OnePair.
            ("32T4K", "T5QJ5", Ordering::Less),
            // A OnePair to a TwoPairs.
            ("T5QJ5", "KK677", Ordering::Less),
            // A TwoPairs to a HighCard.
            ("KK677", "32T4K", Ordering::Greater),
            // A TwoPairs to a ThreeOfAKind.
            ("KK677", "KTJJJ", Ordering::Less),
            // A FiveOfAKind to a FullHouse.
            ("AAAAA", "AQAQA", Ordering::Greater),
            // A FullHouse to a OnePair.
            ("QQQAA", "5QTJ5", Ordering::Greater),
            // Two identical hands.
            ("7K67K", "7K67K", Ordering::Equal),
            // Two cards with the same type.
            ("KK677", "KTJJT", Ordering::Greater),
            ("KK677", "KKJJT", Ordering::Less),
            ("KK677", "KK6AA", Ordering::Less),
            ("KK699", "KK6TT", Ordering::Less),
        ];
        for (cards1, cards2, expected) in tests {
            let hand1 = Hand {
                cards: cards1.to_string(),
                bid: 0,
                joker: false,
            };
            let hand2 = Hand {
                cards: cards2.to_string(),
                bid: 0,
                joker: false,
            };
            assert_eq!(expected, hand1.cmp(&hand2));
        }
    }

    #[test]
    fn test_parse_input_part1() {
        let expected = vec![
            Hand {
                cards: "32T3K".to_string(),
                bid: 765,
                joker: false,
            },
            Hand {
                cards: "T55J5".to_string(),
                bid: 684,
                joker: false,
            },
            Hand {
                cards: "KK677".to_string(),
                bid: 28,
                joker: false,
            },
            Hand {
                cards: "KTJJT".to_string(),
                bid: 220,
                joker: false,
            },
            Hand {
                cards: "QQQJA".to_string(),
                bid: 483,
                joker: false,
            },
        ];

        assert_eq!(expected, parse_input_part1(get_test_input_part1()));
    }

    #[test]
    fn test_parse_input_part2() {
        let expected = vec![
            Hand {
                cards: "32T3K".to_string(),
                bid: 765,
                joker: true,
            },
            Hand {
                cards: "T55J5".to_string(),
                bid: 684,
                joker: true,
            },
            Hand {
                cards: "KK677".to_string(),
                bid: 28,
                joker: true,
            },
            Hand {
                cards: "KTJJT".to_string(),
                bid: 220,
                joker: true,
            },
            Hand {
                cards: "QQQJA".to_string(),
                bid: 483,
                joker: true,
            },
        ];

        assert_eq!(expected, parse_input_part2(get_test_input_part2()));
    }

    #[test]
    fn part1_example() {
        let input = parse_input_part1(get_test_input_part1());
        assert_eq!(6440, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input_part2(get_test_input_part2());
        assert_eq!(5905, part2(&input));
    }

    fn get_test_input_part1<'a>() -> &'a str {
        indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "}
    }

    fn get_test_input_part2<'a>() -> &'a str {
        get_test_input_part1()
    }
}
