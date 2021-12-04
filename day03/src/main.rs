use aoc::*;

#[allow(unused)]
use itertools::Itertools;
use std::{
    cmp,
    path::{Path, PathBuf},
};

/// The path to the puzzle's input file.
pub fn input_path() -> PathBuf {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    crate_root.join("input")
}

fn parse_line(s: String) -> Vec<u8> {
    s.as_bytes().into_iter().map(|b| b - b'0').collect()
}

fn bin_to_int(binary: &Vec<u8>) -> u128 {
    binary.iter().fold(0, |acc, &bit| (acc << 1) + bit as u128)
}

fn main() {
    #[allow(unused)]
    let input: Vec<_> = read_lines(input_path()).map(parse_line).collect();

    let (maxes, mins): (Vec<_>, Vec<_>) = (0..input[0].len())
        .map(|i| {
            let counts = input.iter().map(|s| s[i]).counts();
            let max = cmp::max_by_key(0, 1, |i| counts[i]);
            (max, 1 - max)
        })
        .unzip();

    let result1 = bin_to_int(&maxes) * bin_to_int(&mins);

    println!("Part 1: {}", result1);

    let result2 = {
        let longest_match = |other: Vec<u8>| {
            input
                .iter()
                .max_by_key(|binary| {
                    binary
                        .iter()
                        .zip(other.iter())
                        .take_while(|(l, r)| *l == *r)
                        .count()
                })
                .unwrap()
        };

        bin_to_int(&longest_match(maxes)) * bin_to_int(&longest_match(mins))
    };

    println!("Part 2: {}", result2);
}
