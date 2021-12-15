use std::collections::HashMap;

use itertools::Itertools;

fn step_by(
    n: usize,
    rules: &HashMap<(char, char), char>,
    mut char_counts: HashMap<char, usize>,
    mut pair_counts: HashMap<(char, char), usize>,
) -> (HashMap<char, usize>, HashMap<(char, char), usize>) {
    for _ in 0..n {
        let mut new_pair_counts = HashMap::with_capacity(pair_counts.len());
        for (pair, count) in pair_counts.into_iter() {
            let inner = rules[&pair];
            *new_pair_counts.entry((pair.0, inner)).or_insert(0) += count;
            *new_pair_counts.entry((inner, pair.1)).or_insert(0) += count;
            *char_counts.entry(inner).or_insert(0) += count;
        }
        pair_counts = new_pair_counts;
    }
    (char_counts, pair_counts)
}

pub fn run(input: &str) {
    let (start, rules) = input.split_once("\n\n").unwrap();
    let rules: HashMap<_, _> = rules
        .lines()
        .map(|line| {
            let (lhs, rhs) = line.split_once(" -> ").unwrap();
            let lhs: (_, _) = lhs.chars().collect_tuple().unwrap();
            let rhs = rhs.chars().next().unwrap();
            (lhs, rhs)
        })
        .collect();

    let char_counts = start.chars().counts();
    let pair_counts: HashMap<(char, char), _> = start.chars().tuple_windows().counts();

    let (char_counts, pair_counts) = step_by(10, &rules, char_counts, pair_counts);
    let result1 = {
        let (min, max) = char_counts.values().minmax().into_option().unwrap();
        max - min
    };
    println!("Part 1: {}", result1);

    let (char_counts, _) = step_by(30, &rules, char_counts, pair_counts);
    let result2 = {
        let (min, max) = char_counts.into_values().minmax().into_option().unwrap();
        max - min
    };
    println!("Part 2: {}", result2);
}
