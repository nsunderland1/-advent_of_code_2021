use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn step(s: &str, rules: &HashMap<(char, char), char>) -> String {
    let seps = s.chars().tuple_windows().map(|pair| rules[&pair]);
    s.chars().interleave(seps).collect()
}

fn counts_after_n(s: &str, rules: &HashMap<(char, char), char>, n: usize) -> HashMap<char, usize> {
    (0..n)
        .fold(s.to_string(), |acc, _| step(&acc, &rules))
        .chars()
        .counts()
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

    let result1 = {
        let res = (0..10).fold(start.to_string(), |acc, _| step(&acc, &rules));
        let counts = res.chars().counts();
        let (min, max) = counts.values().minmax().into_option().unwrap();
        max - min
    };

    println!("Part 1: {}", result1);

    let result2 = {
        let letters: HashSet<_> = rules.values().copied().collect();

        let counts_after_20: HashMap<(char, char), HashMap<char, usize>> = rules
            .keys()
            .map(|&(l, r)| ((l, r), counts_after_n(&format!("{}{}", l, r), &rules, 20)))
            .collect();

        let after_20 = (0..20).fold(start.to_string(), |acc, _| step(&acc, &rules));
        let mut total_counts = after_20
            .chars()
            .tuple_windows()
            .map(|pair| &counts_after_20[&pair])
            .fold(HashMap::new(), |acc: HashMap<char, usize>, counts| {
                letters
                    .iter()
                    .map(|&c| (c, acc.get(&c).unwrap_or(&0) + counts.get(&c).unwrap_or(&0)))
                    .collect()
            });

        let after_20_chars = after_20.chars().collect_vec();
        for c in after_20_chars[1..(after_20_chars.len() - 1)].into_iter() {
            *total_counts.get_mut(c).unwrap() -= 1;
        }

        let (min, max) = total_counts.values().minmax().into_option().unwrap();
        max - min
    };

    println!("Part 2: {}", result2);
}
