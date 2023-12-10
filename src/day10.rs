use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
enum TileType {
    Empty = b'.',
    NorthSouth = b'|',
    EastWest = b'-',
    NorthEast = b'L',
    NorthWest = b'J',
    SouthEast = b'F',
    SouthWest = b'7',
    Start = b'S',
}

impl<T> From<T> for TileType
where
    T: Into<char>,
{
    fn from(c: T) -> Self {
        match c.into() {
            '.' => TileType::Empty,
            '|' => TileType::NorthSouth,
            '-' => TileType::EastWest,
            'L' => TileType::NorthEast,
            'J' => TileType::NorthWest,
            'F' => TileType::SouthEast,
            '7' => TileType::SouthWest,
            'S' => TileType::Start,
            _ => unreachable!(),
        }
    }
}

impl TileType {
    fn neighbouring_directions(&self) -> Vec<Direction> {
        match self {
            TileType::Empty => vec![],
            TileType::NorthSouth => vec![Direction::North, Direction::South],
            TileType::EastWest => vec![Direction::East, Direction::West],
            TileType::NorthEast => vec![Direction::North, Direction::East],
            TileType::NorthWest => vec![Direction::North, Direction::West],
            TileType::SouthEast => vec![Direction::South, Direction::East],
            TileType::SouthWest => vec![Direction::South, Direction::West],
            TileType::Start => vec![
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ],
        }
    }

    fn get_box_drawing_character(&self) -> char {
        match self {
            TileType::Empty => '.',
            TileType::NorthSouth => '│',
            TileType::EastWest => '─',
            TileType::NorthEast => '╰',
            TileType::NorthWest => '╯',
            TileType::SouthEast => '╭',
            TileType::SouthWest => '╮',
            TileType::Start => 'S',
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn try_from_xy(x: isize, y: isize) -> Option<Self> {
        if x < 0 || y < 0 {
            return None;
        }
        Some(Coordinate {
            x: x as usize,
            y: y as usize,
        })
    }

    fn neighbor(&self, d: &Direction) -> Option<Coordinate> {
        match d {
            Direction::North => Coordinate::try_from_xy(self.x as isize, self.y as isize - 1),
            Direction::East => Coordinate::try_from_xy(self.x as isize + 1, self.y as isize),
            Direction::South => Coordinate::try_from_xy(self.x as isize, self.y as isize + 1),
            Direction::West => Coordinate::try_from_xy(self.x as isize - 1, self.y as isize),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Debug, PartialEq)]
struct Tiles {
    tiles: HashMap<Coordinate, (TileType, Option<usize>)>,
    size: (usize, usize),
}

impl<T> From<T> for Tiles
where
    T: Into<String>,
{
    fn from(input: T) -> Self {
        let mut tiles = HashMap::new();
        let mut x = 0;
        let mut y = 0;
        for line in input.into().lines() {
            x = 0;
            for c in line.chars() {
                tiles.insert(Coordinate { x, y }, (TileType::from(c), None));
                x += 1;
            }
            y += 1;
        }
        Tiles {
            tiles,
            size: (x, y),
        }
    }
}

impl Tiles {
    fn get_start_coordinate(&self) -> Coordinate {
        for (coordinate, (tile_type, _)) in self.tiles.iter() {
            if *tile_type == TileType::Start {
                return coordinate.clone();
            }
        }
        unreachable!();
    }

    fn get_neighbors(&self, c: &Coordinate) -> Vec<Coordinate> {
        let mut neighbors = vec![];
        let (tile_type, _) = self.tiles.get(c).unwrap();
        for direction in tile_type.neighbouring_directions() {
            let neighbor = c.neighbor(&direction);
            if let Some(n) = neighbor {
                let (neighbor_tile_type, _) = self.tiles.get(&n).unwrap();
                // If we are on a start tile we need to check if the tile we are moving to is
                // connected to us.
                if *tile_type == TileType::Start {
                    let n_directions = neighbor_tile_type.neighbouring_directions();
                    let is_connected = match direction {
                        Direction::North => n_directions.contains(&Direction::South),
                        Direction::East => n_directions.contains(&Direction::West),
                        Direction::South => n_directions.contains(&Direction::North),
                        Direction::West => n_directions.contains(&Direction::East),
                    };
                    if is_connected {
                        neighbors.push(n);
                    }
                } else {
                    // We are not on a start tile, we can assume we are in a closed loop and the
                    // next neighbor links back to us.
                    neighbors.push(n);
                }
            }
        }
        neighbors
    }

    fn next(&self, curpos: &Coordinate) -> Option<Coordinate> {
        let neighbors = self.get_neighbors(curpos);
        for neighbor in neighbors {
            let (_, distance) = self.tiles.get(&neighbor).unwrap();
            // Only return the neighbor if we didn't visit it before.
            if distance.is_none() {
                return Some(neighbor);
            }
        }
        None
    }

    fn walk(&mut self) -> usize {
        let mut distance = 0;

        // Start walking at the start tile.
        let mut curpos = self.get_start_coordinate();

        loop {
            // Mark the current tile as visited.
            let (_, d) = self.tiles.get_mut(&curpos).unwrap();
            *d = Some(distance);

            // Walk to the next tile.
            distance += 1;
            if let Some(nextpos) = self.next(&curpos) {
                curpos = nextpos;
            } else {
                // If there is no next tile we looped back to the start.
                break;
            }
        }
        // Return the distance walked.
        distance
    }

    /// Returns whether the tile at the given coordinate is empty or not. A tile is empty if it is
    /// an empty tile or if it is unvisited after walking the entire pipe.
    fn is_empty(&self, c: &Coordinate) -> bool {
        let (tile_type, distance) = self.tiles.get(c).unwrap();
        *tile_type == TileType::Empty || distance.is_none()
    }
}

// A display formatter for tiles.
impl std::fmt::Display for Tiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pipe = String::new();
        let mut clean = String::new();
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let (tile_type, distance) = self.tiles.get(&Coordinate { x, y }).unwrap();
                let c = tile_type.get_box_drawing_character();
                pipe.push(c);
                if distance.is_some() {
                    clean.push(c);
                } else {
                    clean.push('.');
                }
            }
            pipe.push('\n');
            clean.push('\n');
        }
        write!(f, "{}\n{}", pipe, clean)
    }
}

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Tiles {
    input.into()
}

#[aoc(day10, part1)]
fn part1(tiles: &Tiles) -> usize {
    let mut tiles: Tiles = tiles.clone();
    tiles.walk() / 2
}

#[aoc(day10, part2)]
fn part2(tiles: &Tiles) -> usize {
    use TileType::*;
    // Walk the tiles so we know which tiles are part of our pipe.
    let mut tiles: Tiles = tiles.clone();
    tiles.walk();

    // Convert the tiles to a format that we can use to calculate whether a tile is inside our pipe.
    let mut count = 0;
    for y in 0..tiles.size.1 {
        let mut found_vertical_pipes = 0;
        let mut prev = Empty;
        for x in 0..tiles.size.0 {
            let coord = Coordinate { x, y };
            let mut cur = tiles.tiles.get(&Coordinate { x, y }).unwrap().0;
            // If the current tile is a piece of junk pipe, consider it empty.
            if tiles.is_empty(&coord) {
                cur = Empty;
            }
            match cur {
                // If the tile is a vertical pipe, we need to track how many we have seen so far.
                NorthSouth => found_vertical_pipes += 1,
                // If the tile is a horizontal pipe, we ignore it.
                EastWest => {
                    cur = prev;
                }
                // If the tile is a corner pipe, we need to check if it continues the direction of
                // the previous tile. Then it counts as a vertical pipe.
                NorthWest => {
                    if prev == SouthEast {
                        found_vertical_pipes += 1;
                    }
                }
                SouthWest => {
                    if prev == NorthEast {
                        found_vertical_pipes += 1;
                    }
                }
                // If the tile is empty, we need to check if it is enclosed by the pipe. Tiles are
                // inside if there are an odd number of vertical pipes to the left (or right).
                Empty => {
                    if found_vertical_pipes % 2 == 1 {
                        count += 1;
                    }
                }
                _ => {}
            }
            prev = cur;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_get_neighbors() {
        let tiles = parse_input(get_test_input_part1_example_1());
        let test_cases = vec![
            (
                Coordinate { x: 1, y: 1 },
                vec![Coordinate { x: 2, y: 1 }, Coordinate { x: 1, y: 2 }],
            ),
            (
                Coordinate { x: 2, y: 1 },
                vec![Coordinate { x: 3, y: 1 }, Coordinate { x: 1, y: 1 }],
            ),
            (
                Coordinate { x: 3, y: 1 },
                vec![Coordinate { x: 3, y: 2 }, Coordinate { x: 2, y: 1 }],
            ),
        ];
        // Test example 1.
        for (coordinate, expected) in &test_cases {
            assert_eq!(*expected, tiles.get_neighbors(coordinate));
        }
        // Test example 2. Should have the same neighbors as example 1.
        let tiles = parse_input(get_test_input_part1_example_2());
        for (coordinate, expected) in &test_cases {
            assert_eq!(*expected, tiles.get_neighbors(coordinate));
        }

        // Test example 3.
        let test_cases = vec![
            (
                Coordinate { x: 0, y: 2 },
                vec![Coordinate { x: 1, y: 2 }, Coordinate { x: 0, y: 3 }],
            ),
            (
                Coordinate { x: 0, y: 4 },
                vec![Coordinate { x: 0, y: 3 }, Coordinate { x: 1, y: 4 }],
            ),
            (
                Coordinate { x: 1, y: 3 },
                vec![Coordinate { x: 1, y: 4 }, Coordinate { x: 2, y: 3 }],
            ),
            (
                Coordinate { x: 4, y: 2 },
                vec![Coordinate { x: 4, y: 3 }, Coordinate { x: 3, y: 2 }],
            ),
        ];

        let tiles = parse_input(get_test_input_part1_example_3());
        for (coordinate, expected) in &test_cases {
            assert_eq!(*expected, tiles.get_neighbors(coordinate));
        }
    }

    #[test]
    fn test_parse_input_part1() {
        use TileType::*;
        let expected = vec![
            vec![Empty, Empty, Empty, Empty, Empty],
            vec![Empty, Start, EastWest, SouthWest, Empty],
            vec![Empty, NorthSouth, Empty, NorthSouth, Empty],
            vec![Empty, NorthEast, EastWest, NorthWest, Empty],
            vec![Empty, Empty, Empty, Empty, Empty],
        ];
        let tiles = Tiles {
            tiles: expected
                .iter()
                .enumerate()
                .flat_map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .map(move |(x, tile_type)| (Coordinate { x, y }, (*tile_type, None)))
                })
                .collect(),
            size: (5, 5),
        };

        assert_eq!(tiles, parse_input(get_test_input_part1_example_1()));
    }

    #[test]
    fn test_parse_input_part2() {
        test_parse_input_part1();
    }

    #[test]
    fn part1_example() {
        let input = parse_input(get_test_input_part1_example_1());
        assert_eq!(4, part1(&input));
        let input = parse_input(get_test_input_part1_example_2());
        assert_eq!(4, part1(&input));
        let input = parse_input(get_test_input_part1_example_3());
        assert_eq!(8, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input(get_test_input_part2_example_1());
        assert_eq!(4, part2(&input));
        let input = parse_input(get_test_input_part2_example_2());
        assert_eq!(8, part2(&input));
        let input = parse_input(get_test_input_part2_example_3());
        assert_eq!(10, part2(&input));
    }

    fn get_test_input_part1_example_1<'a>() -> &'a str {
        indoc! {"
            .....
            .S-7.
            .|.|.
            .L-J.
            .....
        "}
    }

    fn get_test_input_part1_example_2<'a>() -> &'a str {
        indoc! {"
            -L|F7
            7S-7|
            L|7||
            -L-J|
            L|-JF
        "}
    }

    fn get_test_input_part1_example_3<'a>() -> &'a str {
        indoc! {"
            7-F7-
            .FJ|7
            SJLL7
            |F--J
            LJ.LJ
        "}
    }

    fn get_test_input_part2_example_1<'a>() -> &'a str {
        indoc! {"
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........
        "}
    }

    fn get_test_input_part2_example_2<'a>() -> &'a str {
        indoc! {"\
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...
        "}
    }

    fn get_test_input_part2_example_3<'a>() -> &'a str {
        indoc! {"\
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L
        "}
    }
}
