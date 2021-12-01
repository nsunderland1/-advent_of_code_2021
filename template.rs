use aoc::*;

use itertools::Itertools;
use std::path::{Path, PathBuf};

/// The path to the puzzle's input file.
pub fn input_path() -> PathBuf {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    crate_root.join("input")
}

fn parse_line(s: String) -> u32 {
    s.parse().unwrap()
}

fn main() {
    let input: Vec<_> = read_lines(input_path()).map(parse_line).collect();

    let result1 = {};

    println!("Part 1: {}", result1);

    let result2 = {};

    println!("Part 2: {}", result2);
}
