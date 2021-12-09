use itertools::Itertools;

use crate::grid::Grid;

fn parse_line(s: &str) -> Vec<u32> {
    s.chars()
        .map(|c| String::from(c).parse::<u32>().unwrap())
        .collect()
}

fn is_low_point(map: &Grid<u32>, point: &(usize, usize)) -> bool {
    neighbours(map, point).all(|neighbour| map[neighbour] > map[*point])
}

fn neighbours<'a>(
    map: &'a Grid<u32>,
    &(x, y): &(usize, usize),
) -> impl Iterator<Item = (usize, usize)> + 'a {
    vec![
        (x.saturating_sub(1), y),
        (x.saturating_add(1), y),
        (x, y.saturating_sub(1)),
        (x, y.saturating_add(1)),
    ]
    .into_iter()
    .filter(move |&(xi, yi)| xi != x || yi != y)
    .filter(|&(xi, yi)| xi < map.width() && yi < map.height())
}

fn compute_basin_size(point: (usize, usize), map: &Grid<u32>, visited: &mut Grid<bool>) -> usize {
    visited[point] = true;

    let mut total = 1;
    for neighbour in neighbours(map, &point) {
        if visited[neighbour] || map[neighbour] == 9 {
            continue;
        }
        total += compute_basin_size(neighbour, map, visited);
    }
    total
}

pub fn run(input: &str) {
    let input: Grid<_> = input.lines().map(parse_line).collect();

    let low_points = Itertools::cartesian_product(0..input.width(), 0..input.height())
        .filter(|point| is_low_point(&input, point))
        .collect_vec();

    let result1 = {
        low_points
            .iter()
            .map(|&point| input[point] + 1)
            .sum::<u32>()
    };

    println!("Part 1: {}", result1);

    let result2 = {
        let mut visited = Grid::new(input.width(), input.height());

        let mut basin_sizes = low_points
            .into_iter()
            .map(|low_point| compute_basin_size(low_point, &input, &mut visited))
            .collect_vec();

        basin_sizes.sort();
        basin_sizes.into_iter().rev().take(3).product::<usize>()
    };

    println!("Part 2: {}", result2);
}
