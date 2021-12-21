use itertools::Itertools;

use crate::grid::{Grid, WindowMut};

fn parse_state(input: char) -> u8 {
    match input {
        '#' => 1,
        '.' => 0,
        _ => unreachable!(),
    }
}

pub fn neighbours_and_self<T>(
    window: &WindowMut<T>,
    (x, y): (usize, usize),
) -> impl Iterator<Item = Option<(usize, usize)>> {
    let x = x as isize;
    let y = y as isize;
    let width = window.width();
    let height = window.height();
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
    .map(move |(i, j)| {
        if i >= 0 && (i as usize) < width && j >= 0 && (j as usize) < height {
            Some((i as usize, j as usize))
        } else {
            None
        }
    })
}

const PART_1_TURNS: usize = 2;
const PART_2_TURNS: usize = 50;

pub fn run(input: &str) {
    let (algorithm, base_grid) = input.split_once("\n\n").unwrap();
    let algorithm = algorithm.chars().map(parse_state).collect_vec();
    let initial_width = base_grid.lines().next().unwrap().len();
    let initial_height = base_grid.lines().count();
    let full_width = initial_width + 2 * PART_2_TURNS;
    let full_height = initial_height + 2 * PART_2_TURNS;

    let mut full_grid = Grid::new(full_width, full_height);
    let mut window = WindowMut::new(
        &mut full_grid,
        (PART_2_TURNS, PART_2_TURNS),
        (PART_2_TURNS + initial_width, PART_2_TURNS + initial_height),
    );

    for (y, line) in base_grid.lines().enumerate() {
        for (x, state) in line.chars().map(parse_state).enumerate() {
            window[(x, y)] = state;
        }
    }

    let mut next_grid = Grid::new(full_width, full_height);
    let mut next_window = WindowMut::new(
        &mut next_grid,
        (PART_2_TURNS, PART_2_TURNS),
        (PART_2_TURNS + initial_width, PART_2_TURNS + initial_height),
    );

    let mut fog_of_war = 0;

    for turn in 0..PART_2_TURNS {
        window = window.grow_once();
        next_window = next_window.grow_once();
        for y in [0, window.height() - 1] {
            for x in 0..window.width() {
                window[(x, y)] = fog_of_war;
            }
        }
        for y in 0..window.height() {
            for x in [0, window.width() - 1] {
                window[(x, y)] = fog_of_war;
            }
        }

        for y in 0..window.height() {
            for x in 0..window.width() {
                let algo_index = neighbours_and_self(&window, (x, y))
                    .map(|neighbour| {
                        neighbour
                            .map(|neighbour| window[neighbour])
                            .unwrap_or(fog_of_war)
                    })
                    .fold(0, |acc, state| (acc << 1) + state as usize);
                next_window[(x, y)] = algorithm[algo_index];
            }
        }

        for y in 0..window.height() {
            for x in 0..window.width() {
                window[(x, y)] = next_window[(x, y)];
            }
        }

        fog_of_war = algorithm[0usize.wrapping_sub(fog_of_war as usize) & 0b111_111_111usize];

        if turn == PART_1_TURNS - 1 {
            let mut result1 = 0;
            for y in 0..window.height() {
                for x in 0..window.width() {
                    result1 += window[(x, y)] as usize;
                }
            }
            println!("Part 1: {}", result1);
        }
    }

    let result2 = full_grid
        .into_flat_iter()
        .filter(|&state| state == 1)
        .count();

    println!("Part 2: {}", result2);
}
