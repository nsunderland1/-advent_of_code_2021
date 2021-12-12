use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    str::FromStr,
};

use itertools::Itertools;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Kind {
    Big,
    Small,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Cave {
    kind: Kind,
    name: String,
}

impl FromStr for Cave {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kind = if s.to_ascii_lowercase() == s {
            Kind::Small
        } else {
            Kind::Big
        };

        Ok(Self {
            kind,
            name: s.to_string(),
        })
    }
}

fn parse_line(s: &str) -> (Cave, Cave) {
    let (from, to) = s.split_once('-').unwrap();
    (from.parse().unwrap(), to.parse().unwrap())
}

fn find_paths(
    graph: &HashMap<Cave, Vec<Cave>>,
    visited: &mut HashSet<Cave>,
    current: &Cave,
    goal: &Cave,
) -> usize {
    if *current == *goal {
        1
    } else if visited.contains(current) {
        0
    } else if let Some(next_caves) = graph.get(current) {
        if current.kind == Kind::Small {
            visited.insert(current.clone());
        }
        let count = next_caves
            .iter()
            .map(|cave| find_paths(graph, visited, &cave, goal))
            .sum();
        visited.remove(current);
        count
    } else {
        0
    }
}

fn find_paths_2(
    graph: &HashMap<Cave, Vec<Cave>>,
    visited: HashSet<Cave>,
    current: &Cave,
    goal: &Cave,
    mut doubled: bool,
) -> usize {
    let start = Cave {
        kind: Kind::Small,
        name: String::from("start"),
    };

    if *current == *goal {
        1
    } else if (doubled || *current == *goal || *current == start) && visited.contains(current) {
        0
    } else if let Some(next_caves) = graph.get(current) {
        let mut next_visited = visited.clone();
        if visited.contains(current) {
            assert!(!doubled);
            doubled = true;
        } else if current.kind == Kind::Small {
            next_visited.insert(current.clone());
        }
        next_caves
            .iter()
            .map(|cave| find_paths_2(graph, next_visited.clone(), &cave, goal, doubled))
            .sum()
    } else {
        0
    }
}

pub fn run(input: &str) {
    let edges = input.lines().map(parse_line);
    let graph: HashMap<Cave, Vec<Cave>> = {
        let mut graph = HashMap::new();
        for (from, to) in edges {
            graph
                .entry(from.clone())
                .or_insert_with(|| Vec::new())
                .push(to.clone());
            graph.entry(to).or_insert_with(|| Vec::new()).push(from);
        }
        graph
    };

    let start = Cave {
        kind: Kind::Small,
        name: String::from("start"),
    };

    let end = Cave {
        kind: Kind::Small,
        name: String::from("end"),
    };

    let result1 = {
        let mut visited = HashSet::new();
        find_paths(&graph, &mut visited, &start, &end)
    };

    println!("Part 1: {}", result1);

    let result2 = {
        let mut visited = HashSet::new();
        find_paths_2(&graph, visited, &start, &end, false)
    };

    println!("Part 2: {}", result2);
}
