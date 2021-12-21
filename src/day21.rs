use std::collections::HashMap;

use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct State {
    p1_score: usize,
    p1_pos: usize,
    p2_score: usize,
    p2_pos: usize,
}

fn dirac_die_roll3() -> impl Iterator<Item = (usize, usize, usize)> {
    (1..=3).flat_map(|i| (1..=3).flat_map(move |j| (1..=3).map(move |k| (i, j, k))))
}

fn part2(state_cache: &mut HashMap<State, (usize, usize)>, state: State) -> (usize, usize) {
    let mut wins = (0, 0);
    for (a, b, c) in dirac_die_roll3() {
        let mut next_state = state;
        next_state.p1_pos += a + b + c;
        next_state.p1_score += (next_state.p1_pos - 1) % 10 + 1;
        if next_state.p1_score >= 21 {
            wins.0 += 1;
            continue;
        }

        for (a, b, c) in dirac_die_roll3() {
            let mut next_state = next_state;
            next_state.p2_pos += a + b + c;
            next_state.p2_score += (next_state.p2_pos - 1) % 10 + 1;
            if next_state.p2_score >= 21 {
                wins.1 += 1;
                continue;
            }

            if let Some(entry) = state_cache.get(&next_state) {
                wins.0 += entry.0;
                wins.1 += entry.1;
            } else {
                let entry = part2(state_cache, next_state);
                wins.0 += entry.0;
                wins.1 += entry.1;
            }
        }
    }

    state_cache.insert(state, wins);
    wins
}

pub fn run(input: &str) {
    let (p1_start, p2_start) = input
        .lines()
        .map(|line| line.split_once(": ").unwrap().1.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();

    let result1 = {
        let mut die = (1..=100).cycle();
        let mut p1_pos = p1_start;
        let mut p2_pos = p2_start;
        let mut p1_score = 0;
        let mut p2_score = 0;

        let mut rolls = 0;
        loop {
            p1_pos += die.by_ref().take(3).sum::<usize>();
            p1_pos = (p1_pos - 1) % 10 + 1;
            rolls += 3;
            p1_score += p1_pos;
            if p1_score >= 1000 {
                break;
            }

            p2_pos += die.by_ref().take(3).sum::<usize>();
            p2_pos = (p2_pos - 1) % 10 + 1;
            rolls += 3;
            p2_score += p2_pos;
            if p2_score >= 1000 {
                break;
            }
        }

        let losing = std::cmp::min(p1_score, p2_score);
        losing * rolls
    };

    println!("Part 1: {}", result1);

    let result2 = {
        let state = State {
            p1_pos: p1_start,
            p1_score: 0,
            p2_pos: p2_start,
            p2_score: 0,
        };

        let mut cache = HashMap::new();

        let wins = part2(&mut cache, state);
        std::cmp::max(wins.0, wins.1)
    };

    println!("Part 2: {}", result2);
}
