use aoc::*;

#[allow(unused)]
use itertools::Itertools;
use std::path::{Path, PathBuf};

/// The path to the puzzle's input file.
pub fn input_path() -> PathBuf {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    crate_root.join("input")
}

fn board_has_won(board: &Vec<Vec<Option<u64>>>) -> bool {
    board
        .iter()
        .any(|row| row.iter().find(|n| n.is_some()).is_none())
        || (0..board[0].len()).any(|i| board.iter().find(|row| row[i].is_some()).is_none())
}

fn main() {
    #[allow(unused)]
    // let input: Vec<_> = read_lines(input_path()).map(parse_line).collect();
    let input_str = file_to_string(input_path());
    let mut input = input_str.split("\n\n");
    let moves = input
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();

    let mut boards = input
        .map(|board| {
            board
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|s| Some(s.parse::<u64>().unwrap()))
                        .collect_vec()
                })
                .collect_vec()
        })
        .collect_vec();

    let result1 = {
        let moves = moves.clone();
        let mut boards = boards.clone();
        let mut res = 0;
        'outer: for num in moves {
            for board in boards.iter_mut() {
                'board_loop: for row in board.iter_mut() {
                    for entry in row.iter_mut() {
                        let entry_val = entry.take();
                        if entry_val == Some(num) {
                            break 'board_loop;
                        } else {
                            *entry = entry_val;
                        }
                    }
                }
                if board_has_won(board) {
                    res = num
                        * board
                            .iter()
                            .map(|row| row.iter().filter_map(|x| x.as_ref()).sum::<u64>())
                            .sum::<u64>();
                    break 'outer;
                }
            }
        }
        res
    };

    println!("Part 1: {}", result1);

    let result2 = {
        let mut res = 0;
        'outer2: for num in moves {
            boards = boards
                .into_iter()
                .map(|board| {
                    let mut board = board.clone();
                    'board_loop: for row in board.iter_mut() {
                        for entry in row.iter_mut() {
                            let entry_val = entry.take();
                            if entry_val == Some(num) {
                                break 'board_loop;
                            } else {
                                *entry = entry_val;
                            }
                        }
                    }
                    board
                })
                .filter(|board| {
                    if board_has_won(board) {
                        res = num
                            * board
                                .iter()
                                .map(|row| row.iter().filter_map(|x| x.as_ref()).sum::<u64>())
                                .sum::<u64>();
                        // break 'outer;
                        return false;
                    }
                    true
                })
                .collect();
            if boards.len() == 0 {
                break 'outer2;
            }
        }
        res
    };

    println!("Part 2: {}", result2);
}
