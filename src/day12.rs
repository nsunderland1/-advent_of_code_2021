use std::collections::{HashMap, HashSet};

use itertools::Itertools;

// use std::{
//     collections::{HashMap, HashSet},
//     convert::Infallible,
//     str::FromStr,
// };

// #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// enum Kind {
//     Big,
//     Small,
// }

// #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// struct Cave {
//     kind: Kind,
//     name: String,
// }

// impl FromStr for Cave {
//     type Err = Infallible;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let kind = if s.to_ascii_lowercase() == s {
//             Kind::Small
//         } else {
//             Kind::Big
//         };

//         Ok(Self {
//             kind,
//             name: s.to_string(),
//         })
//     }
// }

// fn parse_line(s: &str) -> (Cave, Cave) {
//     let (from, to) = s.split_once('-').unwrap();
//     (from.parse().unwrap(), to.parse().unwrap())
// }

// fn find_paths(
//     graph: &HashMap<Cave, Vec<Cave>>,
//     visited: &mut HashSet<Cave>,
//     current: &Cave,
//     goal: &Cave,
// ) -> usize {
//     if *current == *goal {
//         1
//     } else if visited.contains(current) {
//         0
//     } else if let Some(next_caves) = graph.get(current) {
//         if current.kind == Kind::Small {
//             visited.insert(current.clone());
//         }
//         let count = next_caves
//             .iter()
//             .map(|cave| find_paths(graph, visited, &cave, goal))
//             .sum();
//         visited.remove(current);
//         count
//     } else {
//         0
//     }
// }

// fn find_paths_2(
//     graph: &HashMap<Cave, Vec<Cave>>,
//     visited: &mut HashMap<Cave, usize>,
//     current: &Cave,
//     goal: &Cave,
//     mut doubled: bool,
// ) -> usize {
//     let start = Cave {
//         kind: Kind::Small,
//         name: String::from("start"),
//     };

//     let entry = visited.entry(current.clone()).or_default();

//     if *current == *goal {
//         1
//     } else if (doubled || *current == start) && *entry > 0 {
//         0
//     } else if let Some(next_caves) = graph.get(current) {
//         if *entry == 1 {
//             assert!(!doubled);
//             doubled = true;
//             *entry += 1
//         } else if current.kind == Kind::Small {
//             *entry += 1;
//         }
//         let count = next_caves
//             .iter()
//             .map(|cave| find_paths_2(graph, visited, &cave, goal, doubled))
//             .sum();
//         let entry = visited.entry(current.clone()).or_default();
//         *entry = entry.saturating_sub(1);

//         count
//     } else {
//         0
//     }
// }

// pub fn run(input: &str) {
//     let edges = input.lines().map(parse_line);
//     let graph: HashMap<Cave, Vec<Cave>> = {
//         let mut graph = HashMap::new();
//         for (from, to) in edges {
//             graph
//                 .entry(from.clone())
//                 .or_insert_with(|| Vec::new())
//                 .push(to.clone());
//             graph.entry(to).or_insert_with(|| Vec::new()).push(from);
//         }
//         graph
//     };

//     let start = Cave {
//         kind: Kind::Small,
//         name: String::from("start"),
//     };

//     let end = Cave {
//         kind: Kind::Small,
//         name: String::from("end"),
//     };

//     let result1 = {
//         let mut visited = HashSet::new();
//         find_paths(&graph, &mut visited, &start, &end)
//     };

//     println!("Part 1: {}", result1);

//     let result2 = {
//         let mut visited = HashMap::new();
//         find_paths_2(&graph, &mut visited, &start, &end, false)
//     };

//     println!("Part 2: {}", result2);
// }

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
