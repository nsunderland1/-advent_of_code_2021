use itertools::Itertools;

use crate::grid::Grid;

pub fn step(grid: &mut Grid<u32>) -> usize {
    let mut flashed_count = 0;
    let mut queue = Itertools::cartesian_product(0..grid.height(), 0..grid.width())
        .map(|(y, x)| (x, y))
        .collect_vec();

    while let Some(cell) = queue.pop() {
        grid[cell] += 1;
        if grid[cell] == 10 {
            flashed_count += 1;
            for neighbour in grid.neighbours(cell) {
                queue.push(neighbour);
            }
        }
    }

    for cell in grid.flat_iter_mut() {
        if *cell > 9 {
            *cell = 0;
        }
    }

    flashed_count
}

pub fn run(input: &str) {
    let mut grid: Grid<u32> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_string().parse().unwrap()))
        .collect();

    let mut total_flashes = 0;
    for _ in 1..=100 {
        total_flashes += step(&mut grid);
    }
    println!("Part 1: {}", total_flashes);

    // Can technically fail if part 2 ends before part 1, but it works for mine at least!
    // It doesn't really make a difference to performance, but it does make the code nicer
    for i in 101.. {
        if step(&mut grid) == grid.height() * grid.width() {
            println!("Part 2: {}", i);
            break;
        }
    }
}
