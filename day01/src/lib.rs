use std::path::{Path, PathBuf};

use aoc::*;
use itertools::Itertools;

/// The path to the puzzle's input file
pub fn input_path() -> PathBuf {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    crate_root.join("input")
}

type Input = u32;
type Output = usize;

fn parse_line(s: &str) -> Input {
    s.parse().unwrap()
}

const STATIC_INPUT: &str = include_str!("../input");
pub fn parse_input() -> Vec<Input> {
    STATIC_INPUT.lines().map(parse_line).collect()
}

pub fn part1(input: Vec<Input>) -> Output {
    input.iter().tuple_windows().filter(|(l, r)| l < r).count()
}

pub fn part2(input: Vec<Input>) -> Output {
    input
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(l, r)| l < r)
        .count()
}
