use std::{
    cmp::{min, Reverse},
    collections::BinaryHeap,
};

use crate::grid::Grid;

fn parse_line(s: &str) -> Vec<u8> {
    s.chars()
        .map(|c| String::from(c).parse().unwrap())
        .collect()
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Marker {
    Unvisited,
    Visited,
}

#[derive(PartialEq, Eq)]
struct PointWithDistance((usize, usize), u32);

impl PartialOrd for PointWithDistance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

impl Ord for PointWithDistance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}

fn risk(grid: &Grid<u8>) -> u32 {
    let mut markers = grid![Marker::Unvisited; grid.width(), grid.height()];
    let mut distance = grid![u32::MAX; grid.width(), grid.height()];

    distance[(0, 0)] = 0;

    let mut frontier = BinaryHeap::new();
    frontier.push(Reverse(PointWithDistance((0, 0), 0)));

    loop {
        let Reverse(PointWithDistance(current, _)) = frontier.pop().unwrap();
        if markers[current] == Marker::Visited {
            continue;
        }

        if current == (grid.width() - 1, grid.height() - 1) {
            return distance[current];
        }

        let unvisited_neighbours = grid
            .neighbours_orthogonal(current)
            .filter(|&cell| markers[cell] == Marker::Unvisited);

        for neighbour in unvisited_neighbours {
            distance[neighbour] = min(
                distance[neighbour],
                grid[neighbour] as u32 + distance[current],
            );
            frontier.push(Reverse(PointWithDistance(neighbour, distance[neighbour])));
        }

        markers[current] = Marker::Visited;
    }
}

pub fn run(input: &str) {
    let grid: Grid<_> = input.lines().map(parse_line).collect();

    let result1 = risk(&grid);

    println!("Part 1: {}", result1);

    let result2 = {
        let mut big_grid = Grid::new(5 * grid.width(), 5 * grid.height());
        for y in 0..big_grid.height() {
            for x in 0..big_grid.width() {
                let x_chunk = x / grid.width();
                let y_chunk = y / grid.height();
                let source_x = x % grid.width();
                let source_y = y % grid.height();
                big_grid[(x, y)] =
                    (grid[(source_x, source_y)] + x_chunk as u8 + y_chunk as u8 - 1) % 9 + 1;
            }
        }

        risk(&big_grid)
    };

    println!("Part 2: {}", result2);
}
