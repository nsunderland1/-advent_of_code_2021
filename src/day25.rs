use itertools::Itertools;

use crate::grid::Grid;

pub fn run(input: &str) {
    let mut grid: Grid<_> = input.lines().map(|line| line.chars()).collect();
    let mut east_positions = Itertools::cartesian_product(0..grid.height(), 0..grid.width())
        .map(|(y, x)| (x, y))
        .filter(|&(y, x)| grid[(x, y)] == '>')
        .collect_vec();
    let mut south_positions = Itertools::cartesian_product(0..grid.height(), 0..grid.width())
        .map(|(y, x)| (x, y))
        .filter(|&pos| grid[pos] == 'v')
        .collect_vec();

    let mut updates =
        Vec::with_capacity(std::cmp::max(east_positions.len(), south_positions.len()));

    let mut res = 0;
    for i in 1.. {
        let mut updates_done = false;
        let grid_len = grid.height();
        let row_len = grid.width();
        {
            for pos in east_positions.iter_mut() {
                if grid[((pos.0 + 1) % grid.width(), pos.1)] == '.' {
                    updates.push(*pos);
                    pos.0 = (pos.0 + 1) % grid.width();
                    updates_done = true;
                }
            }

            for (x, y) in updates.drain(..) {
                grid[(x, y)] = '.';
                grid[((x + 1) % row_len, y)] = '>';
            }
        }

        {
            for pos in south_positions.iter_mut() {
                if grid[(pos.0, (pos.1 + 1) % grid.height())] == '.' {
                    updates.push(*pos);
                    pos.1 = (pos.1 + 1) % grid.height();
                    updates_done = true;
                }
            }

            for (x, y) in updates.drain(..) {
                grid[(x, y)] = '.';
                grid[(x, (y + 1) % grid_len)] = 'v';
            }
        }

        if !updates_done {
            res = i;
            break;
        }
    }

    println!("Part 1: {}", res);
}
