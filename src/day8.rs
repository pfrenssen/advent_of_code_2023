use aoc_runner_derive::{aoc, aoc_generator};
use num_integer::Integer;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Node {
    id: String,
    children: [String; 2],
}

impl Node {
    fn get_child_id_by_direction(&self, d: &char) -> String {
        match d {
            'L' => self.children[0].clone(),
            'R' => self.children[1].clone(),
            _ => unreachable!(),
        }
    }
}

#[aoc_generator(day8, part1)]
fn parse_input_part1(input: &str) -> (String, HashMap<String, Node>) {
    let mut lines = input.lines();

    // The first line contains the directions.
    let directions = lines.next().unwrap().to_string();

    // Skip the empty line.
    lines.next();

    let re = regex::Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    let nodes: HashMap<String, Node> = lines
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let id = caps[1].to_string();
            let children = [caps[2].to_string(), caps[3].to_string()];
            (id.clone(), Node { id, children })
        })
        .collect();

    (directions, nodes)
}

#[aoc_generator(day8, part2)]
fn parse_input_part2(input: &str) -> (String, HashMap<String, Node>) {
    parse_input_part1(input)
}

#[aoc(day8, part1)]
fn part1(input: &(String, HashMap<String, Node>)) -> usize {
    let (directions, nodes) = input;
    let directions: Vec<char> = directions.chars().collect();
    let mut steps = 0;
    let mut current_node = &nodes["AAA"];
    while current_node.id != "ZZZ" {
        let current_direction = directions[steps % directions.len()];
        let next_node_id = current_node.get_child_id_by_direction(&current_direction);
        current_node = &nodes[&next_node_id];
        steps += 1;
    }
    steps
}

#[aoc(day8, part2)]
fn part2(input: &(String, HashMap<String, Node>)) -> usize {
    let (directions, nodes) = input;
    let directions: Vec<char> = directions.chars().collect();

    // Find all nodes whose ids end with "A".
    let current_nodes: Vec<&Node> = nodes
        .values()
        .filter(|node| node.id.ends_with('A'))
        .collect();

    // Retrieve the cycle length for each node.
    let cycle_lengths: Vec<usize> = current_nodes
        .iter()
        .map(|node| {
            let mut current_node = *node;
            let mut steps = 0;
            while !current_node.id.ends_with('Z') {
                let current_direction = directions[steps % directions.len()];
                let next_node_id = current_node.get_child_id_by_direction(&current_direction);
                current_node = &nodes[&next_node_id];
                steps += 1;
            }
            steps
        })
        .collect();

    // Calculate the lowest common multiple of the cycle lengths.
    cycle_lengths
        .into_iter()
        .reduce(|acc, l| acc.lcm(&l))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input_part1() {
        let mut nodes = HashMap::new();
        nodes.insert(
            "AAA".to_string(),
            Node {
                id: "AAA".to_string(),
                children: ["BBB".to_string(), "CCC".to_string()],
            },
        );
        nodes.insert(
            "BBB".to_string(),
            Node {
                id: "BBB".to_string(),
                children: ["DDD".to_string(), "EEE".to_string()],
            },
        );
        nodes.insert(
            "CCC".to_string(),
            Node {
                id: "CCC".to_string(),
                children: ["ZZZ".to_string(), "GGG".to_string()],
            },
        );
        nodes.insert(
            "DDD".to_string(),
            Node {
                id: "DDD".to_string(),
                children: ["DDD".to_string(), "DDD".to_string()],
            },
        );
        nodes.insert(
            "EEE".to_string(),
            Node {
                id: "EEE".to_string(),
                children: ["EEE".to_string(), "EEE".to_string()],
            },
        );
        nodes.insert(
            "GGG".to_string(),
            Node {
                id: "GGG".to_string(),
                children: ["GGG".to_string(), "GGG".to_string()],
            },
        );
        nodes.insert(
            "ZZZ".to_string(),
            Node {
                id: "ZZZ".to_string(),
                children: ["ZZZ".to_string(), "ZZZ".to_string()],
            },
        );
        let expected = ("RL".to_string(), nodes);
        assert_eq!(expected, parse_input_part1(get_test_input_part1_example1()));

        let mut nodes = HashMap::new();
        nodes.insert(
            "AAA".to_string(),
            Node {
                id: "AAA".to_string(),
                children: ["BBB".to_string(), "BBB".to_string()],
            },
        );
        nodes.insert(
            "BBB".to_string(),
            Node {
                id: "BBB".to_string(),
                children: ["AAA".to_string(), "ZZZ".to_string()],
            },
        );
        nodes.insert(
            "ZZZ".to_string(),
            Node {
                id: "ZZZ".to_string(),
                children: ["ZZZ".to_string(), "ZZZ".to_string()],
            },
        );
        let expected = ("LLR".to_string(), nodes);
        assert_eq!(expected, parse_input_part1(get_test_input_part1_example2()));
    }

    #[test]
    fn test_parse_input_part2() {
        let mut nodes = HashMap::new();
        nodes.insert(
            "11A".to_string(),
            Node {
                id: "11A".to_string(),
                children: ["11B".to_string(), "XXX".to_string()],
            },
        );
        nodes.insert(
            "11B".to_string(),
            Node {
                id: "11B".to_string(),
                children: ["XXX".to_string(), "11Z".to_string()],
            },
        );
        nodes.insert(
            "11Z".to_string(),
            Node {
                id: "11Z".to_string(),
                children: ["11B".to_string(), "XXX".to_string()],
            },
        );
        nodes.insert(
            "22A".to_string(),
            Node {
                id: "22A".to_string(),
                children: ["22B".to_string(), "XXX".to_string()],
            },
        );
        nodes.insert(
            "22B".to_string(),
            Node {
                id: "22B".to_string(),
                children: ["22C".to_string(), "22C".to_string()],
            },
        );
        nodes.insert(
            "22C".to_string(),
            Node {
                id: "22C".to_string(),
                children: ["22Z".to_string(), "22Z".to_string()],
            },
        );
        nodes.insert(
            "22Z".to_string(),
            Node {
                id: "22Z".to_string(),
                children: ["22B".to_string(), "22B".to_string()],
            },
        );
        nodes.insert(
            "XXX".to_string(),
            Node {
                id: "XXX".to_string(),
                children: ["XXX".to_string(), "XXX".to_string()],
            },
        );
        assert_eq!(
            ("LR".to_string(), nodes),
            parse_input_part2(get_test_input_part2())
        );
    }

    #[test]
    fn part1_example() {
        let input = parse_input_part1(get_test_input_part1_example1());
        assert_eq!(2, part1(&input));
        let input = parse_input_part1(get_test_input_part1_example2());
        assert_eq!(6, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input_part2(get_test_input_part2());
        assert_eq!(6, part2(&input));
    }

    fn get_test_input_part1_example1<'a>() -> &'a str {
        indoc! {"
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
        "}
    }

    fn get_test_input_part1_example2<'a>() -> &'a str {
        indoc! {"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        "}
    }

    fn get_test_input_part2<'a>() -> &'a str {
        indoc! {"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        "}
    }
}
