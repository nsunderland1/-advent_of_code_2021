use std::path::{Path, PathBuf};

#[macro_use]
mod grid;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;

const DAY_TABLE: &[fn(&str)] = &[
    day01::run,
    day02::run,
    day03::run,
    day04::run,
    day05::run,
    day06::run,
    day07::run,
    day08::run,
    day09::run,
    day10::run,
    day11::run,
    day12::run,
    day13::run,
    day14::run,
    day15::run,
    day16::run,
    day17::run,
];

/// Get the path to the input file for a given day
pub fn get_input_file(day: u32) -> PathBuf {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    crate_root.join("input").join(format!("{}.input", day))
}

/// Run a specific day with the given input as a string
pub fn run_day(day: u32, input: &str) {
    DAY_TABLE[day as usize - 1](input)
}
