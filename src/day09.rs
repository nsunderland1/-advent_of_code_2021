use std::{cmp::Reverse, iter::Rev};

#[allow(unused)]
use itertools::Itertools;

fn parse_line(s: &str) -> Vec<u32> {
    s.chars()
        .map(|c| String::from(c).parse::<u32>().unwrap())
        .collect()
}

fn is_low_point(map: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    for yi in y.saturating_sub(1)..=y.saturating_add(1).min(map.len() - 1) {
        if yi == y {
            continue;
        }
        if map[yi][x] <= map[y][x] {
            return false;
        }
    }

    for xi in x.saturating_sub(1)..=x.saturating_add(1).min(map[0].len() - 1) {
        if xi == x {
            continue;
        }
        if map[y][xi] <= map[y][x] {
            return false;
        }
    }

    true
}

fn neighbours(map: &Vec<Vec<u32>>, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    Iterator::chain(
        (x.saturating_sub(1)..=x.saturating_add(1).min(map[0].len() - 1))
            .filter(move |&xi| xi != x)
            .map(move |xi| (xi, y)),
        (y.saturating_sub(1)..=y.saturating_add(1).min(map.len() - 1))
            .filter(move |&yi| yi != y)
            .map(move |yi| (x, yi)),
    )
}

fn compute_basin_size(
    (x, y): (usize, usize),
    map: &Vec<Vec<u32>>,
    visited: &mut Vec<Vec<bool>>,
) -> usize {
    visited[y][x] = true;
    let mut total = 1;
    for neighbour @ (i, j) in neighbours(map, x, y) {
        if visited[j][i] || map[j][i] == 9 {
            continue;
        }
        total += compute_basin_size(neighbour, map, visited);
    }
    total
}

pub fn run(input: &str) {
    #[allow(unused)]
    let input: Vec<_> = input.lines().map(parse_line).collect();

    let result1 = {
        input
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(|(x, val)| {
                        if is_low_point(&input, x, y) {
                            Some(1 + val)
                        } else {
                            None
                        }
                    })
                    .sum::<u32>()
            })
            .sum::<u32>()
    };

    println!("Part 1: {}", result1);

    let result2 = {
        let low_points = input.iter().cloned().enumerate().flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(x, _)| {
                    if is_low_point(&input, x, y) {
                        Some((x, y))
                    } else {
                        None
                    }
                })
                .collect_vec()
                .into_iter()
        });
        let mut visited = vec![vec![false; input[0].len()]; input.len()];

        let mut basin_sizes = low_points
            .map(|low_point| compute_basin_size(low_point, &input, &mut visited))
            .collect_vec();

        basin_sizes.sort();

        basin_sizes.into_iter().rev().take(3).product::<usize>()
    };

    println!("Part 2: {}", result2);
}
