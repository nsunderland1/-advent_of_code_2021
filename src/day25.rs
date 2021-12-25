#[allow(unused)]
use itertools::Itertools;

pub fn run(input: &str) {
    let mut grid: Vec<_> = input.lines().map(|s| s.chars().collect_vec()).collect();

    let result1 = {
        let mut res = 0;
        for i in 1.. {
            let mut updates_done = false;
            let grid_len = grid.len();
            let row_len = grid[0].len();
            {
                let mut updates = Vec::with_capacity(grid.len() * grid[0].len());

                for row in 0..grid.len() {
                    for col in 0..grid[row].len() {
                        if grid[row][col] == '>' && grid[row][(col + 1) % grid[row].len()] == '.' {
                            updates.push((row, col));
                            updates_done = true;
                        }
                    }
                }

                for (row, col) in updates {
                    grid[row][col] = '.';
                    grid[row][(col + 1) % row_len] = '>';
                }
            }

            {
                let mut updates = Vec::with_capacity(grid.len() * grid[0].len());

                for row in 0..grid.len() {
                    for col in 0..grid[row].len() {
                        if grid[row][col] == 'v' && grid[(row + 1) % grid.len()][col] == '.' {
                            updates.push((row, col));
                            updates_done = true;
                        }
                    }
                }

                for (row, col) in updates {
                    grid[row][col] = '.';
                    grid[(row + 1) % grid_len][col] = 'v';
                }
            }

            if !updates_done {
                res = i;
                break;
            }
        }

        res
    };

    println!("Part 1: {}", result1);

    let result2 = {
        // Part 2
        0
    };

    println!("Part 2: {}", result2);
}
