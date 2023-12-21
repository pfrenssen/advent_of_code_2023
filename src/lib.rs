use aoc_runner_derive::aoc_lib;

mod day1;
mod day10;
mod day11;
pub mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

aoc_lib! { year = 2023 }

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}
