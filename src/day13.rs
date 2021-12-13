use std::collections::HashSet;

#[allow(unused)]
use itertools::Itertools;

use crate::grid::Grid;

fn parse_point(s: &str) -> (usize, usize) {
    s.split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect_tuple()
        .unwrap()
}

enum Fold {
    X(usize),
    Y(usize),
}

fn parse_fold(s: &str) -> Fold {
    let (axis, offset) = s
        .split(' ')
        .last()
        .unwrap()
        .split('=')
        .collect_tuple()
        .unwrap();
    let offset = offset.parse().unwrap();
    match axis {
        "x" => Fold::X(offset),
        "y" => Fold::Y(offset),
        _ => unreachable!(),
    }
}

pub fn run(input: &str) {
    // let input: Vec<_> = input.lines().map(parse_line).collect();
    let (points, folds) = input.split_once("\n\n").unwrap();
    let points: HashSet<_> = points.lines().map(parse_point).collect();
    let folds = folds.lines().map(parse_fold).collect_vec();

    let result1 = {
        let mut points = points.clone();
        for fold in folds.iter().take(1) {
            points = points
                .into_iter()
                .map(|(x, y)| match fold {
                    &Fold::X(offset) if x > offset => (offset - (x - offset), y),
                    &Fold::Y(offset) if y > offset => (x, offset - (y - offset)),
                    _ => (x, y),
                })
                .collect();
        }
        points.len()
    };

    println!("Part 1: {}", result1);

    let result2 = {
        let mut points = points.clone();
        for fold in folds.iter() {
            points = points
                .into_iter()
                .map(|(x, y)| match fold {
                    &Fold::X(offset) if x > offset => (offset - (x - offset), y),
                    &Fold::Y(offset) if y > offset => (x, offset - (y - offset)),
                    _ => (x, y),
                })
                .collect();
        }
        let max_x = points.iter().map(|point| point.0).max().unwrap();
        let max_y = points.iter().map(|point| point.1).max().unwrap();

        let mut grid = Grid::new(max_x + 1, max_y + 1);
        for point in points {
            grid[point] = true;
        }

        for y in 0..grid.height() {
            for x in 0..grid.width() {
                print!("{}", if grid[(x, y)] { '#' } else { ' ' });
            }
            println!("");
        }
    };
}
