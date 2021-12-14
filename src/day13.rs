use std::{collections::HashSet, str::FromStr};

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

impl FromStr for Fold {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (axis, offset) = s
            .split(' ')
            .last()
            .ok_or(String::from("Invalid fold"))?
            .split('=')
            .collect_tuple()
            .ok_or(String::from("Invalid fold"))?;

        let offset = offset
            .parse()
            .map_err(|_| String::from("Failed to parse offset"))?;

        match axis {
            "x" => Ok(Fold::X(offset)),
            "y" => Ok(Fold::Y(offset)),
            c => Err(format!("Invalid axis: {}", c)),
        }
    }
}

impl Fold {
    fn apply(&self, points: HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
        points
            .into_iter()
            .map(|(x, y)| match *self {
                Fold::X(offset) if x > offset => (offset - (x - offset), y),
                Fold::Y(offset) if y > offset => (x, offset - (y - offset)),
                _ => (x, y),
            })
            .collect()
    }
}

pub fn run(input: &str) {
    let (points, folds) = input.split_once("\n\n").unwrap();
    let points: HashSet<_> = points.lines().map(parse_point).collect();
    let mut folds = folds.lines().map(str::parse::<Fold>).map(Result::unwrap);

    let points = folds.next().unwrap().apply(points);
    println!("Part 1: {}", points.len());

    let points = folds.fold(points, |points, fold| fold.apply(points));

    let max_x = points.iter().map(|point| point.0).max().unwrap();
    let max_y = points.iter().map(|point| point.1).max().unwrap();

    let mut grid = Grid::new(max_x + 1, max_y + 1);
    for point in points {
        grid[point] = true;
    }

    println!("Part 2: ");
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            print!("{}", if grid[(x, y)] { '#' } else { ' ' });
        }
        println!("");
    }
}
