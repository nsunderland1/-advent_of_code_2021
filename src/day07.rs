use itertools::Itertools;

pub fn run(input: &str) {
    let mut input = input
        .split(',')
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect_vec();

    let result1 = {
        // have to do this because the borrow checker is dumb on the next line
        let len = input.len();
        // dereference so that we can do an immutable borrow below
        let median = *input.select_nth_unstable(len / 2).1;
        input.iter().map(|n| (n - median).abs()).sum::<i32>()
    };

    println!("Part 1: {}", result1);

    // My original solution iterated over `0..=max`, but smart people on the
    // internet realized the optimum is always within 1/2 of the mean. This version
    // is over 10x faster on my machine.
    let result2 = {
        let mean = input.iter().sum::<i32>() / input.len() as i32;
        ((mean - 1)..=(mean + 1))
            .map(|x| {
                input
                    .iter()
                    .map(|pos| {
                        let n = (pos - x).abs();
                        n * (n + 1) / 2
                    })
                    .sum::<i32>()
            })
            .min()
            .unwrap()
    };

    println!("Part 2: {}", result2);
}
