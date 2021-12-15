use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug)]
struct Cave {
    // name: String,
    is_small: bool,
    neighbours: Vec<usize>,
}

fn find_paths(caves: &Vec<Cave>, is_visited: &mut Vec<bool>, current: usize, goal: usize) -> usize {
    if current == goal {
        1
    } else if is_visited[current] {
        0
    } else {
        if caves[current].is_small {
            is_visited[current] = true;
        }

        let count = caves[current]
            .neighbours
            .iter()
            .map(|&cave| find_paths(caves, is_visited, cave, goal))
            .sum();
        is_visited[current] = false;
        count
    }
}

fn find_paths_2(
    caves: &Vec<Cave>,
    visit_count: &mut Vec<usize>,
    current: usize,
    start: usize,
    goal: usize,
    mut can_repeat: bool,
) -> usize {
    if current == goal {
        1
    } else if visit_count[current] > 0 && (!can_repeat || current == start) {
        0
    } else {
        if caves[current].is_small {
            visit_count[current] += 1;
            if visit_count[current] > 1 {
                can_repeat = false;
            }
        }

        let count = caves[current]
            .neighbours
            .iter()
            .map(|&cave| find_paths_2(caves, visit_count, cave, start, goal, can_repeat))
            .sum();

        if caves[current].is_small {
            visit_count[current] -= 1;
        }

        count
    }
}

pub fn run(input: &str) {
    let cave_names: HashSet<_> = input.lines().flat_map(|line| line.split('-')).collect();
    let name_to_id: HashMap<_, _> = cave_names
        .into_iter()
        .enumerate()
        .map(|(i, name)| (name, i))
        .collect();

    // Build up a list of caves where the index corresponds to the ID
    let mut caves = name_to_id
        .iter()
        .sorted_by_key(|(_, &id)| id)
        .map(|(&name, _)| Cave {
            // name: String::from(name),
            is_small: name.to_ascii_lowercase() == name,
            neighbours: Vec::new(),
        })
        .collect_vec();

    for (cave1, cave2) in input.lines().map(|line| line.split_once('-').unwrap()) {
        let id_1 = name_to_id[cave1];
        let id_2 = name_to_id[cave2];
        caves[id_1].neighbours.push(id_2);
        caves[id_2].neighbours.push(id_1);
    }

    let start = name_to_id["start"];
    let end = name_to_id["end"];

    let mut is_visited = vec![false; caves.len()];
    println!(
        "Part 1: {}",
        find_paths(&caves, &mut is_visited, start, end)
    );

    let mut visit_count = vec![0; caves.len()];
    println!(
        "Part 1: {}",
        find_paths_2(&caves, &mut visit_count, start, start, end, true)
    );
}
