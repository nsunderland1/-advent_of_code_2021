use std::collections::{hash_map::Entry, HashMap};

#[allow(unused)]
use itertools::Itertools;

fn parse_state(input: char) -> u8 {
    match input {
        '#' => 1,
        '.' => 0,
        _ => unreachable!(),
    }
}

pub fn neighbours_and_self((x, y): (isize, isize)) -> impl Iterator<Item = (isize, isize)> {
    [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
    .into_iter()
}

pub fn run(input: &str) {
    let (algorithm, grid) = input.split_once("\n\n").unwrap();
    let algorithm = algorithm.chars().map(parse_state).collect_vec();
    let grid: HashMap<(isize, isize), u8> = grid
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, cell)| ((x as isize, y as isize), parse_state(cell)))
        })
        .collect();

    let result1 = {
        let mut grid = grid.clone();

        for turn in 0..50 {
            let fog_of_war = turn % 2;
            for point in grid.keys().copied().collect_vec() {
                for neighbour in neighbours_and_self(point) {
                    if let Entry::Vacant(entry) = grid.entry(neighbour) {
                        entry.insert(fog_of_war);
                    }
                }
            }

            let mut next_grid = grid.clone();
            for point in grid.keys() {
                let algo_index = neighbours_and_self(*point).fold(0, |acc, neighbour| {
                    (acc << 1) + (*grid.get(&neighbour).unwrap_or(&fog_of_war) as usize)
                });
                next_grid.insert(*point, algorithm[algo_index]);
            }

            grid = next_grid;
        }

        grid.into_values().filter(|&state| state == 1).count()
    };

    println!("Part 1: {}", result1);

    let result2 = {
        // Part 2
        0
    };

    println!("Part 2: {}", result2);
}
