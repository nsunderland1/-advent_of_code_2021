use aoc::*;

use itertools::Itertools;
use std::path::{Path, PathBuf};

/// The path to the puzzle's input file.
pub fn input_path() -> PathBuf {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    crate_root.join("input")
}

fn parse_line(s: String) -> (String, u32) {
    let s: (&str, &str) = s.split(" ").collect_tuple().unwrap();
    (s.0.to_string(), s.1.parse().unwrap())
}

fn main() {
    let input: Vec<_> = read_lines(input_path()).map(parse_line).collect();

    let result1 = {
        input
            .iter()
            .fold((0, 0), |(x, y), (dir, amt)| match dir.as_str() {
                "forward" => (x + amt, y),
                "down" => (x, y + amt),
                "up" => (x, y - amt),
                _ => unreachable!(),
            })
    };

    println!("Part 1: {}", result1.0 * result1.1);

    let result2 = {
        input
            .iter()
            .fold((0, 0, 0), |(x, y, aim), (dir, amt)| match dir.as_str() {
                "forward" => (x + amt, y + aim * amt, aim),
                "down" => (x, y, aim + amt),
                "up" => (x, y, aim - amt),
                _ => unreachable!(),
            })
    };

    println!("Part 2: {}", result2.0 * result2.1);
}
