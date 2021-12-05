use itertools::Itertools;
use std::cmp;

fn parse_line(s: &str) -> Vec<u8> {
    s.as_bytes().into_iter().map(|b| b - b'0').collect()
}

fn bin_to_int(binary: &Vec<u8>) -> u128 {
    binary.iter().fold(0, |acc, &bit| (acc << 1) + bit as u128)
}

pub fn run(input: &str) {
    let input: Vec<_> = input.lines().map(parse_line).collect();

    let result1 = {
        let (maxes, mins): (Vec<_>, Vec<_>) = (0..input[0].len())
            .map(|i| {
                let counts = input.iter().map(|s| s[i]).counts();
                let max = cmp::max_by_key(0, 1, |i| counts[i]);
                (max, 1 - max)
            })
            .unzip();

        bin_to_int(&maxes) * bin_to_int(&mins)
    };

    println!("Part 1: {}", result1);

    let result2 = {
        let oxygen = {
            let mut remaining = input.clone();
            for i in 0..input[0].len() {
                match remaining.as_slice() {
                    [] => unreachable!(),
                    [_] => break,
                    _ => {
                        let counts = remaining.iter().map(|s| s[i]).counts();
                        let max = cmp::max_by_key(0, 1, |i| counts[i]);
                        remaining.retain(|num| num[i] == max);
                    }
                }
            }
            assert_eq!(remaining.len(), 1);
            bin_to_int(&remaining[0])
        };

        let co2 = {
            let mut remaining = input.clone();
            for i in 0..input[0].len() {
                match remaining.as_slice() {
                    [] => unreachable!(),
                    [_] => break,
                    _ => {
                        let counts = remaining.iter().map(|s| s[i]).counts();
                        let min = cmp::min_by_key(0, 1, |i| counts[i]);
                        remaining.retain(|num| num[i] == min);
                    }
                }
            }
            assert_eq!(remaining.len(), 1);
            bin_to_int(&remaining[0])
        };

        oxygen * co2
    };

    println!("Part 2: {}", result2);
}
