use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3, part1)]
fn parse_input_part1(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc_generator(day3, part2)]
fn parse_input_part2(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[derive(Clone, Copy, Debug)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl Coordinate {
    fn x_distance(&self, other: &Coordinate) -> usize {
        (self.x - other.x).unsigned_abs()
    }
    fn y_distance(&self, other: &Coordinate) -> usize {
        (self.y - other.y).unsigned_abs()
    }
    fn is_adjacent(&self, other: &Coordinate) -> bool {
        self.x_distance(other) <= 1 && self.y_distance(other) <= 1
    }
}

#[derive(Clone, Debug)]
struct PartNumber {
    number: usize,
    coordinates: Vec<Coordinate>,
}

#[derive(Clone, Debug)]
struct Symbol {
    symbol: char,
    coordinate: Coordinate,
}

fn get_part_numbers_and_symbols(lines: &[String]) -> (Vec<PartNumber>, Vec<Symbol>) {
    let mut part_numbers: Vec<PartNumber> = vec![];
    let mut symbols: Vec<Symbol> = vec![];

    for (y, line) in lines.iter().enumerate() {
        let mut number_found = false;
        let mut number = String::new();
        let mut coordinates: Vec<Coordinate> = vec![];
        for (x, symbol) in line.chars().enumerate() {
            if symbol.is_ascii_digit() {
                // If we are already in a number, add the digit to the number.
                if number_found {
                    number.push(symbol);
                } else {
                    // If we are not in a number, start a new one.
                    number_found = true;
                    number = symbol.to_string();
                    coordinates = vec![];
                }
                // Add the coordinate to the list of coordinates.
                coordinates.push(Coordinate {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                });
            }
            // Current symbol is not a digit.
            else {
                // If we were tracking a number, add the number to the list of part numbers.
                if number_found {
                    part_numbers.push(PartNumber {
                        number: number.parse().unwrap(),
                        coordinates: coordinates.clone(),
                    });
                    number_found = false;
                }

                // If it is anything other than a number or a period, add it to the list of symbols.
                if symbol != '.' {
                    symbols.push(Symbol {
                        symbol,
                        coordinate: Coordinate {
                            x: x.try_into().unwrap(),
                            y: y.try_into().unwrap(),
                        },
                    });
                }
            }
        }
        // If we were tracking a number, add the number to the list of part numbers.
        if number_found {
            part_numbers.push(PartNumber {
                number: number.parse().unwrap(),
                coordinates: coordinates.clone(),
            });
        }
    }

    (part_numbers, symbols)
}

#[aoc(day3, part1)]
fn part1(lines: &[String]) -> usize {
    // Get all part numbers and symbols.
    let (part_numbers, symbols) = get_part_numbers_and_symbols(lines);

    // Loop over all part numbers and filter out the ones that are not adjacent to any symbol.
    let adjacent_part_numbers: Vec<PartNumber> = part_numbers
        .iter()
        .filter(|part_number| {
            symbols.iter().any(|symbol| {
                part_number
                    .coordinates
                    .iter()
                    .any(|c| c.is_adjacent(&symbol.coordinate))
            })
        })
        .cloned()
        .collect();

    // Sum the numbers of the adjacent part numbers.
    adjacent_part_numbers
        .iter()
        .map(|part_number| part_number.number)
        .sum()
}

#[aoc(day3, part2)]
fn part2(lines: &[String]) -> usize {
    let (part_numbers, symbols) = get_part_numbers_and_symbols(lines);

    // All gears are symbols identified by a '*'.
    let gears: Vec<Symbol> = symbols
        .iter()
        .filter(|symbol| symbol.symbol == '*')
        .cloned()
        .collect();

    // Reduce the list of part numbers to only those that are adjacent to a gear.
    let part_numbers: Vec<PartNumber> = part_numbers
        .iter()
        .filter(|part_number| {
            gears.iter().any(|gear| {
                part_number
                    .coordinates
                    .iter()
                    .any(|c| c.is_adjacent(&gear.coordinate))
            })
        })
        .cloned()
        .collect();

    let mut result = 0;

    // Loop over all gears.
    for gear in gears {
        // Get all parts that are adjacent to the current gear.
        let adjacent_part_numbers: Vec<PartNumber> = part_numbers
            .iter()
            .filter(|part_number| {
                part_number
                    .coordinates
                    .iter()
                    .any(|c| c.is_adjacent(&gear.coordinate))
            })
            .cloned()
            .collect();
        // If there are exactly two adjacent parts, add their product to the result.
        if adjacent_part_numbers.len() == 2 {
            result += adjacent_part_numbers[0].number * adjacent_part_numbers[1].number;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input_part1() {
        let expected = vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
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
        assert_eq!(4361, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input_part2(get_test_input_part2());
        assert_eq!(467835, part2(&input));
    }

    fn get_test_input_part1<'a>() -> &'a str {
        indoc! {"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "}
    }

    fn get_test_input_part2<'a>() -> &'a str {
        get_test_input_part1()
    }
}
