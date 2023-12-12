use aoc_runner_derive::{aoc, aoc_generator};
use nalgebra::{DMatrix, Dyn, RowVector, VecStorage, U1};

#[aoc_generator(day11, part1)]
fn parse_input_part1(input: &str) -> DMatrix<u16> {
    // Count the number of rows and lines.
    let mut lines = vec![];
    let mut galaxy_count = 0;
    for line in input.lines() {
        let row: Vec<u16> = line
            .chars()
            .map(|c| match c {
                '.' => 0,
                '#' => {
                    galaxy_count += 1;
                    galaxy_count
                }
                _ => unreachable!("Unknown character"),
            })
            .collect();
        lines.push(RowVector::<u16, Dyn, VecStorage<u16, U1, Dyn>>::from_vec(
            row,
        ));
    }
    DMatrix::from_rows(&lines)
}

fn expand_universe(mut universe: DMatrix<u16>) -> DMatrix<u16> {
    // Find empty rows and columns.
    let empty_rows: Vec<_> = universe
        .row_iter()
        .enumerate()
        .filter(|(_, row)| row.sum() == 0)
        .map(|(i, _)| i)
        .collect();
    let empty_cols: Vec<_> = universe
        .column_iter()
        .enumerate()
        .filter(|(_, col)| col.sum() == 0)
        .map(|(i, _)| i)
        .collect();

    // Expand the universe.
    for i in empty_rows.iter().rev() {
        universe = universe.insert_row(*i, 0);
    }
    for i in empty_cols.iter().rev() {
        universe = universe.insert_column(*i, 0);
    }
    universe
}

fn get_galaxies(universe: &DMatrix<u16>) -> Vec<(usize, usize)> {
    let mut galaxies = vec![];
    for y in 0..universe.nrows() {
        for x in 0..universe.ncols() {
            if universe[(y, x)] != 0 {
                galaxies.push((x, y));
            }
        }
    }
    galaxies
}

fn get_sum_of_distances(universe: &DMatrix<u16>, growth_rate: usize) -> usize {
    let universe = universe.clone();
    // Locate the galaxies in the universe.
    let mut galaxies = get_galaxies(&universe);

    // Find empty rows and columns.
    let empty_rows: Vec<_> = universe
        .row_iter()
        .enumerate()
        .filter(|(_, row)| row.sum() == 0)
        .map(|(i, _)| i)
        .collect();
    let empty_cols: Vec<_> = universe
        .column_iter()
        .enumerate()
        .filter(|(_, col)| col.sum() == 0)
        .map(|(i, _)| i)
        .collect();

    // Expand the galaxy.
    for i in empty_cols.iter().rev() {
        galaxies.iter_mut().for_each(|(x, _)| {
            if *x > *i {
                *x += growth_rate - 1;
            }
        });
    }
    for i in empty_rows.iter().rev() {
        galaxies.iter_mut().for_each(|(_, y)| {
            if *y > *i {
                *y += growth_rate - 1;
            }
        });
    }
    get_distances(&galaxies).iter().sum()
}

fn get_distances(galaxies: &[(usize, usize)]) -> Vec<usize> {
    let mut distances = vec![];
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let (x1, y1) = galaxies[i];
            let (x2, y2) = galaxies[j];
            let distance = ((x1 as i64 - x2 as i64).abs() + (y1 as i64 - y2 as i64).abs()) as usize;
            distances.push(distance);
        }
    }
    distances
}

#[aoc_generator(day11, part2)]
fn parse_input_part2(input: &str) -> DMatrix<u16> {
    parse_input_part1(input)
}

#[aoc(day11, part1)]
fn part1(universe: &DMatrix<u16>) -> usize {
    let universe = expand_universe(universe.clone());
    let galaxies = get_galaxies(&universe);
    let distances = get_distances(&galaxies);
    distances.iter().sum()
}

#[aoc(day11, part2)]
fn part2(universe: &DMatrix<u16>) -> usize {
    get_sum_of_distances(universe, 1000000)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input_part1() {
        let mut expected = DMatrix::zeros(10, 10);
        expected[(0, 3)] = 1;
        expected[(1, 7)] = 2;
        expected[(2, 0)] = 3;
        expected[(4, 6)] = 4;
        expected[(5, 1)] = 5;
        expected[(6, 9)] = 6;
        expected[(8, 7)] = 7;
        expected[(9, 0)] = 8;
        expected[(9, 4)] = 9;

        assert_eq!(expected, parse_input_part1(get_test_input_part1()));

        // Expand the universe.
        let universe = expand_universe(expected.clone());

        let mut expected = DMatrix::zeros(12, 13);
        expected[(0, 4)] = 1;
        expected[(1, 9)] = 2;
        expected[(2, 0)] = 3;
        expected[(5, 8)] = 4;
        expected[(6, 1)] = 5;
        expected[(7, 12)] = 6;
        expected[(10, 9)] = 7;
        expected[(11, 0)] = 8;
        expected[(11, 5)] = 9;

        assert_eq!(expected, universe);
    }

    #[test]
    fn test_parse_input_part2() {
        test_parse_input_part1();
    }

    #[test]
    fn part1_example() {
        let input = parse_input_part1(get_test_input_part1());
        assert_eq!(374, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input_part2(get_test_input_part2());
        assert_eq!(1030, get_sum_of_distances(&input, 10));
        assert_eq!(8410, get_sum_of_distances(&input, 100));
    }

    fn get_test_input_part1<'a>() -> &'a str {
        indoc! {"
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        "}
    }

    fn get_test_input_part2<'a>() -> &'a str {
        get_test_input_part1()
    }
}
