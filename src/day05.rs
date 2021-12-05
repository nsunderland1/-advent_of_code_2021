use itertools::{Either, Itertools};

fn parse_line(s: &str) -> ((usize, usize), (usize, usize)) {
    s.split(" -> ")
        .map(|s| {
            s.split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect_tuple()
                .unwrap()
        })
        .collect_tuple()
        .unwrap()
}

pub fn count_overlaps(
    input: &Vec<((usize, usize), (usize, usize))>,
    include_diagonals: bool,
) -> usize {
    let mut grid: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];
    for (start, end) in input.iter() {
        if start.0 == end.0 {
            let min = std::cmp::min(start.1, end.1);
            let max = std::cmp::max(start.1, end.1);
            for y in min..=max {
                grid[y][start.0] += 1;
            }
        } else if start.1 == end.1 {
            let min = std::cmp::min(start.0, end.0);
            let max = std::cmp::max(start.0, end.0);
            for x in min..=max {
                grid[start.1][x] += 1;
            }
        } else if include_diagonals {
            let x_range = if start.0 < end.0 {
                Either::Left(start.0..=end.0)
            } else {
                Either::Right((end.0..=start.0).rev())
            };
            let y_range = if start.1 < end.1 {
                Either::Left(start.1..=end.1)
            } else {
                Either::Right((end.1..=start.1).rev())
            };
            for (x, y) in x_range.zip(y_range) {
                grid[y][x] += 1;
            }
        }
    }

    grid.into_iter()
        .flat_map(Vec::into_iter)
        .filter(|&n| n > 1)
        .count()
}

pub fn run(input: &str) {
    let input: Vec<_> = input.lines().map(parse_line).collect();

    println!("Part 1: {}", count_overlaps(&input, false));
    println!("Part 2: {}", count_overlaps(&input, true));
}
