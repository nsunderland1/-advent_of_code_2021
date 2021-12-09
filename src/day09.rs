use itertools::Itertools;

fn parse_line(s: &str) -> Vec<u32> {
    s.chars()
        .map(|c| String::from(c).parse::<u32>().unwrap())
        .collect()
}

fn is_low_point(map: &Vec<Vec<u32>>, &(x, y): &(usize, usize)) -> bool {
    neighbours(map, x, y).all(|(xi, yi)| map[yi][xi] > map[y][x])
}

// I don't know why Rust needs an anonymous lifetime on the `impl Iterator` here...
fn neighbours(
    map: &Vec<Vec<u32>>,
    x: usize,
    y: usize,
) -> impl Iterator<Item = (usize, usize)> + '_ {
    [
        (x.saturating_sub(1), y),
        (x.saturating_add(1), y),
        (x, y.saturating_sub(1)),
        (x, y.saturating_add(1)),
    ]
    .into_iter()
    .filter(move |&(xi, yi)| xi != x || yi != y)
    .filter(|&(xi, yi)| yi < map.len() && xi < map[yi].len())
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
    let input: Vec<_> = input.lines().map(parse_line).collect();

    let low_points = input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, _)| (x, y)))
        .filter(|point| is_low_point(&input, point))
        .collect_vec();

    let result1 = {
        low_points
            .iter()
            .map(|&(x, y)| input[y][x] + 1)
            .sum::<u32>()
    };

    println!("Part 1: {}", result1);

    let result2 = {
        let mut visited = vec![vec![false; input[0].len()]; input.len()];

        let mut basin_sizes = low_points
            .into_iter()
            .map(|low_point| compute_basin_size(low_point, &input, &mut visited))
            .collect_vec();

        basin_sizes.sort();
        basin_sizes.into_iter().rev().take(3).product::<usize>()
    };

    println!("Part 2: {}", result2);
}
