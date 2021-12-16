use std::{
    cmp::{max, min},
    collections::HashSet,
};

use itertools::Itertools;

struct Vertical {
    x: usize,
    min_y: usize,
    max_y: usize,
}

struct Horizontal {
    y: usize,
    min_x: usize,
    max_x: usize,
}

enum Line {
    Horizontal(Horizontal),
    Vertical(Vertical),
    Diagonal {
        start: (usize, usize),
        end: (usize, usize),
    }, // TODO: split
}

impl Line {
    fn new(start: (usize, usize), end: (usize, usize)) -> Self {
        if start.0 == end.0 {
            Self::Vertical(Vertical {
                x: start.0,
                min_y: min(start.1, end.1),
                max_y: max(start.1, end.1),
            })
        } else if start.1 == end.1 {
            Self::Horizontal(Horizontal {
                y: start.1,
                min_x: min(start.0, end.0),
                max_x: max(start.0, end.0),
            })
        } else {
            Self::Diagonal { start, end }
        }
    }

    fn intersections(&self, other: &Line) -> Vec<(usize, usize)> {
        match (self, other) {
            (Self::Vertical(vertical), Self::Horizontal(horizontal))
            | (Self::Horizontal(horizontal), Self::Vertical(vertical)) => {
                if (horizontal.min_x..=horizontal.max_x).contains(&vertical.x)
                    && (vertical.min_y..=vertical.max_y).contains(&horizontal.y)
                {
                    vec![(vertical.x, horizontal.y)]
                } else {
                    vec![]
                }
            }
            (Self::Vertical(v1), Self::Vertical(v2)) => {
                if v1.x == v2.x && v1.min_y <= v2.max_y && v2.min_y <= v1.max_y {
                    (v1.min_y.clamp(v2.min_y, v2.max_y)..v1.max_y.clamp(v2.min_y, v2.max_y))
                        .map(|y| (v1.x, y))
                        .collect()
                } else {
                    vec![]
                }
            }
            (Self::Horizontal(h1), Self::Horizontal(h2)) => {
                if h1.y == h2.y && h1.min_x <= h2.max_x && h2.min_x <= h1.max_x {
                    (h1.min_x.clamp(h2.min_x, h2.max_x)..h1.max_x.clamp(h2.min_x, h2.max_x))
                        .map(|x| (x, h1.y))
                        .collect()
                } else {
                    vec![]
                }
            }
            (Self::Diagonal { .. }, _) | (_, Self::Diagonal { .. }) => unreachable!(), // skip for now
        }
    }
}

fn parse_line(s: &str) -> Line {
    let (start, end) = s
        .split(" -> ")
        .map(|s| {
            s.split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect_tuple()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();
    Line::new(start, end)
}

fn count_overlaps(lines: &Vec<Line>) -> usize {
    lines
        .iter()
        .tuple_combinations()
        .flat_map(|(l, r)| l.intersections(r).into_iter())
        .sorted()
        .dedup()
        .count()
}

pub fn run(input: &str) {
    let lines: Vec<_> = input.lines().map(parse_line).collect();
    let lines = lines
        .into_iter()
        .filter(|line| !matches!(line, Line::Diagonal { .. }))
        .collect_vec();

    println!("Part 1: {}", count_overlaps(&lines));
    // println!("Part 2: {}", count_overlaps(&input, true));
}
