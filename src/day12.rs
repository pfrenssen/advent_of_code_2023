use crate::Coordinate;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
struct SpringField {
    rows: Vec<(Vec<SpringCondition>, Vec<usize>)>,
}

impl From<&str> for SpringField {
    fn from(s: &str) -> Self {
        let mut rows: Vec<(Vec<SpringCondition>, Vec<usize>)> = vec![];
        for line in s.lines() {
            let (map, damaged_springs) = line.split_once(' ').unwrap();
            let mut row: Vec<SpringCondition> = vec![];
            let mut indexes: Vec<usize> = vec![];
            for (x, c) in map.chars().enumerate() {
                row.push(c.into());
            }
            for dcount in damaged_springs.split(',') {
                indexes.push(dcount.parse::<usize>().unwrap());
            }
            rows.push((row, indexes));
        }
        SpringField { rows }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpringCondition {
    Damaged,
    Operational,
    Unknown,
}

impl From<char> for SpringCondition {
    fn from(c: char) -> Self {
        match c {
            '#' => SpringCondition::Damaged,
            '.' => SpringCondition::Operational,
            _ => SpringCondition::Unknown,
        }
    }
}

#[aoc_generator(day12, part1)]
fn parse_input_part1(input: &str) -> SpringField {
    input.into()
}

#[aoc_generator(day12, part2)]
fn parse_input_part2(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day12, part1)]
fn part1(springfield: &SpringField) -> usize {
    let mut springfield = springfield.clone();
    let mut rows: &Vec<(Vec<SpringCondition>, Vec<usize>)> = &mut springfield.rows;

    let mut arrangements_per_row: Vec<usize> = vec![];

    for row in rows {
        let (map, consecutive_damaged_springs) = row;
        let arrangements = count_arrangements(map, consecutive_damaged_springs);
    }

    // ????#???..?.?? 5,1,1
    // There is a group of 5 consecutive `#`s at the beginning of the row.
    // This can be reduced to:
    // ???##???..?.?? 5,1,1
    // What precedes the group of 5 consecutive `#`s can be disregarded.
    // This can be reduced to:
    // .....???..?.?? 1,1
    // This can be reduced to:
    // .....?.?..?.?. 1,1

    // ????#???..?.?? 5,1,1
    // #####.??..?.?? -> ??..?.?? 1,1
    // .#####.?..?.?? -> ?..?.?? 1,1
    // ..#####...?.?? -> ?.?? 1,1
    // ...#####..?.?? -> ?.?? 1,1

    0
}

fn count_arrangements(
    map: &Vec<SpringCondition>,
    consecutive_damaged_springs: &Vec<usize>,
) -> usize {
    let mut arrangements = 0;
    let mut consecutive_damaged_springs = consecutive_damaged_springs.clone();
    let mut map = map.clone();

    println!("\nConsecutive damaged springs to map: {:?}", consecutive_damaged_springs);
    println!("Original map: {:?}", map);

    // Clean up the map by removing all the operational springs at the beginning and end of the map.
    while let Some(SpringCondition::Operational) = map.first() {
        map.remove(0);
    }
    while let Some(SpringCondition::Operational) = map.last() {
        map.pop();
    }
    // If there are multiple consecutive operational springs in the map, remove all but one.
    map.dedup_by(|a, b| {
        a == &SpringCondition::Operational && b == &SpringCondition::Operational
    });

    println!("Cleaned up map: {:?}", map);

    // Split the map into groups of unknown or damaged springs.
    let mut groups: Vec<Vec<SpringCondition>> = map.split(|c| c == &SpringCondition::Operational).map(|s| s.to_vec()).collect();
    println!("Groups: {:?}", groups);

    // Use the first group to determine the search size.
    let search_size = consecutive_damaged_springs.get(0).unwrap();
    println!("search_size: {}", search_size);

    // Loop through the groups until we find a group that is large enough to contain the search size. Remove the
    // previous groups from the vector.
    remove_undersized_groups(&mut groups, search_size);
    println!("groups: {:?}", groups);

    // We know now that the first group is large enough to contain the search size. We can split the first group up, but
    // we need to take into account that there might be a known operational spring at the end of the group.
    // For example if we are searching for 3 consecutive damaged springs, and our group is `???##?`, we can deduce that
    // our group of 3 consecutive damaged springs can be either `??###?` or `???##` but the first two `?`s do not match.
    let split_point = get_split_point(&groups[0], search_size);

    // Check if the remaining groups
    // fit the remaining consecutive damaged springs.
    // Get a slice of the remaining consecutive damaged springs.
    let mut remaining_consecutive_damaged_springs = consecutive_damaged_springs.split_off(1);
    println!("remaining_consecutive_damaged_springs: {:?}", remaining_consecutive_damaged_springs);
    // Get a slice of the remaining groups prefixed with the first group, but with search_size + 1 elements removed.

    let mut remaining_groups = groups.split_off(1);
    println!("remaining_groups: {:?}", remaining_groups);



    0
}

/// Split the given group up by the given search size. We need to take into account that there might be a known
/// operational spring at the end of the group.
/// For example if we are searching for 3 consecutive damaged springs, and our group is `???##?`, we can deduce that
/// our group of 3 consecutive damaged springs can be either `??###?` or `???##` but the first two `?`s do not match.
///
/// # Examples
///
/// ```
/// use advent_of_code_2023::day12::SpringCondition;
/// use advent_of_code_2023::day12::get_split_point;
///
/// let group = vec![
///     SpringCondition::Unknown,
///     SpringCondition::Unknown,
///     SpringCondition::Unknown,
///     SpringCondition::Damaged,
///     SpringCondition::Damaged,
///     SpringCondition::Unknown,
/// ];
/// let search_size = 2;
/// assert_eq!(2, get_split_point(&group, &search_size));
/// let search_size = 3;
/// assert_eq!(5, get_split_point(&group, &search_size));
/// let search_size = 6;
/// assert_eq!(6, get_split_point(&group, &search_size));
/// ```
pub fn get_split_point(group: &Vec<SpringCondition>, search_size: &usize) -> usize {
    let mut split_point = search_size.to_owned();
    while split_point < group.len() {
        println!("split_point: {}", split_point);
        println!("group[split_point]: {:?}", group[split_point]);
        if group[split_point] != SpringCondition::Damaged {
            break;
        }
        split_point += 1;
    }

    split_point
}
fn remove_undersized_groups(groups: &mut Vec<Vec<SpringCondition>>, search_size: &usize) {
    loop {
        if groups.is_empty() {
            break;
        }
        if groups[0].len() >= *search_size {
            break;
        }

        groups.remove(0);
    }
}

#[aoc(day12, part2)]
fn part2(input: &[String]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input_part1() {
        let mut rows: Vec<(Vec<SpringCondition>, Vec<usize>)> = vec![];

        // ???.### 1,1,3
        let map: Vec<SpringCondition> = vec![
            SpringCondition::Unknown,
            SpringCondition::Unknown,
            SpringCondition::Unknown,
            SpringCondition::Operational,
            SpringCondition::Damaged,
            SpringCondition::Damaged,
            SpringCondition::Damaged,
        ];
        rows.push((map, vec![1, 1, 3]));

        // .??..??...?##. 1,1,3
        let map: Vec<SpringCondition> = vec![
            SpringCondition::Operational,
            SpringCondition::Unknown,
            SpringCondition::Unknown,
            SpringCondition::Operational,
            SpringCondition::Operational,
            SpringCondition::Unknown,
            SpringCondition::Unknown,
            SpringCondition::Operational,
            SpringCondition::Operational,
            SpringCondition::Operational,
            SpringCondition::Unknown,
            SpringCondition::Damaged,
            SpringCondition::Damaged,
            SpringCondition::Operational,
        ];
        rows.push((map, vec![1, 1, 3]));

        // ?#?#?#?#?#?#?#? 1,3,1,6
        let map: Vec<SpringCondition> = vec![
            SpringCondition::Unknown,
            SpringCondition::Damaged,
            SpringCondition::Unknown,
            SpringCondition::Damaged,
            SpringCondition::Unknown,
            SpringCondition::Damaged,
            SpringCondition::Unknown,
            SpringCondition::Damaged,
            SpringCondition::Unknown,
            SpringCondition::Damaged,
            SpringCondition::Unknown,
            SpringCondition::Damaged,
            SpringCondition::Unknown,
            SpringCondition::Damaged,
            SpringCondition::Unknown,
        ];
        rows.push((map, vec![1, 3, 1, 6]));

        // ????.#...#... 4,1,1
        let map: Vec<SpringCondition> = vec![
            SpringCondition::Unknown,
            SpringCondition::Unknown,
            SpringCondition::Unknown,
            SpringCondition::Unknown,
            SpringCondition::Operational,
            SpringCondition::Damaged,
            SpringCondition::Operational,
            SpringCondition::Operational,
            SpringCondition::Operational,
            SpringCondition::Damaged,
            SpringCondition::Operational,
            SpringCondition::Operational,
            SpringCondition::Operational,
        ];
        rows.push((map, vec![4, 1, 1]));

        // ????.######..#####. 1,6,5
        let map: Vec<SpringCondition> = vec![
            SpringCondition::Unknown,
            SpringCondition::Unknown,
            SpringCondition::Unknown,
            SpringCondition::Unknown,
            SpringCondition::Operational,
            SpringCondition::Damaged,
            SpringCondition::Damaged,
            SpringCondition::Damaged,
            SpringCondition::Damaged,
            SpringCondition::Damaged,
            SpringCondition::Damaged,
            SpringCondition::Operational,
            SpringCondition::Operational,
            SpringCondition::Damaged,
            SpringCondition::Damaged,
            SpringCondition::Damaged,
            SpringCondition::Damaged,
            SpringCondition::Damaged,
            SpringCondition::Operational,
        ];
        rows.push((map, vec![1, 6, 5]));

        // ?###???????? 3,2,1
        let map: Vec<SpringCondition> = vec![
            SpringCondition::Unknown,
            SpringCondition::Damaged,
            SpringCondition::Damaged,
            SpringCondition::Damaged,
            SpringCondition::Unknown,
            SpringCondition::Unknown,
            SpringCondition::Unknown,
            SpringCondition::Unknown,
            SpringCondition::Unknown,
            SpringCondition::Unknown,
            SpringCondition::Unknown,
            SpringCondition::Unknown,
        ];
        rows.push((map, vec![3, 2, 1]));

        assert_eq!(
            SpringField { rows },
            parse_input_part1(get_test_input_part1())
        );
    }

    #[test]
    fn test_parse_input_part2() {
        test_parse_input_part1();
    }

    #[test]
    fn part1_example() {
        let input = parse_input_part1(get_test_input_part1());
        assert_eq!(21, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input_part2(get_test_input_part2());
        assert_eq!(0, part2(&input));
    }

    fn get_test_input_part1<'a>() -> &'a str {
        indoc! {"
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
            ??.????#???..?.?? 5,1,1
        "}
    }

    fn get_test_input_part2<'a>() -> &'a str {
        get_test_input_part1()
    }
}
