use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct State {
    p1_score: usize,
    p1_pos: usize,
    p2_score: usize,
    p2_pos: usize,
}

impl State {
    fn cache_index(&self) -> usize {
        self.p1_score * 10 * 21 * 10
            + (self.p1_pos - 1) * 21 * 10
            + self.p2_score * 10
            + (self.p2_pos - 1)
    }
}

const DIRAC_DIE_COMBINATIONS: [(usize, usize); 7] =
    [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn part2(state_cache: &mut Vec<(usize, usize)>, state: State) -> (usize, usize) {
    if state.p1_score >= 21 {
        return (1, 0);
    }
    if state.p2_score >= 21 {
        return (0, 1);
    }

    let cached = state_cache[state.cache_index()];
    if cached.0 > 0 || cached.1 > 0 {
        return cached;
    }

    let mut wins = (0, 0);

    for (roll, count) in DIRAC_DIE_COMBINATIONS {
        let mut next_state = state;
        next_state.p1_pos += roll;
        next_state.p1_pos = (next_state.p1_pos - 1) % 10 + 1;
        next_state.p1_score += next_state.p1_pos;

        let (p2_wins, p1_wins) = part2(
            state_cache,
            State {
                p1_pos: next_state.p2_pos,
                p1_score: next_state.p2_score,
                p2_pos: next_state.p1_pos,
                p2_score: next_state.p1_score,
            },
        );

        wins.0 += count * p1_wins;
        wins.1 += count * p2_wins;
    }

    state_cache[state.cache_index()] = wins;
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

        let mut cache = vec![(0, 0); 21 * 10 * 21 * 10];

        let wins = part2(&mut cache, state);
        std::cmp::max(wins.0, wins.1)
    };

    println!("Part 2: {}", result2);
}
