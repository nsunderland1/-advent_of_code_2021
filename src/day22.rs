use std::ops::RangeInclusive;

use itertools::iproduct;
#[allow(unused)]
use itertools::Itertools;

#[derive(Debug)]
struct Step {
    state: bool,
    x: RangeInclusive<isize>,
    y: RangeInclusive<isize>,
    z: RangeInclusive<isize>,
}

fn parse_line(s: &str) -> Step {
    let (state, ranges) = s.split_once(' ').unwrap();
    let state = match state {
        "on" => true,
        "off" => false,
        _ => unreachable!(),
    };
    let (x, y, z) = ranges
        .split(',')
        .map(|range| {
            let (start, end) = range
                .split('=')
                .skip(1)
                .next()
                .unwrap()
                .split("..")
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap();
            start..=end
        })
        .collect_tuple()
        .unwrap();

    Step { state, x, y, z }
}

fn index((x, y, z): (isize, isize, isize)) -> Option<(usize, usize, usize)> {
    let (x, y, z) = (x + 50, y + 50, z + 50);
    if x >= 0 && y >= 0 && z >= 0 && x <= 100 && y <= 100 && z <= 100 {
        Some((x as usize, y as usize, z as usize))
    } else {
        None
    }
}

pub fn run(input: &str) {
    let input: Vec<_> = input.lines().map(parse_line).collect();

    let mut grid = vec![vec![vec![false; 101]; 101]; 101];

    let result1 = {
        for step in input.iter() {
            if *step.x.start() > 50
                || *step.x.end() < -50
                || *step.y.start() > 50
                || *step.y.end() < -50
                || *step.z.start() > 50
                || *step.z.end() < -50
            {
                continue;
            }
            for (x, y, z) in
                iproduct!(step.x.clone(), step.y.clone(), step.z.clone()).filter_map(index)
            {
                grid[x][y][z] = step.state;
            }
        }

        grid.iter()
            .flat_map(|plane| plane.iter().map(|row| row.iter().filter(|v| **v).count()))
            .sum::<usize>()
    };

    println!("Part 1: {}", result1);

    let result2 = {
        // Part 2
        0
    };

    println!("Part 2: {}", result2);
}
