fn expand_fish(mut fish_by_days_left: [u64; 9], duration: usize) -> u64 {
    let mut shift_from = 0;
    let mut shift_to = 7;

    for _ in 0..duration {
        fish_by_days_left[shift_to] += fish_by_days_left[shift_from];
        shift_to = (shift_to + 1) % 9;
        shift_from = (shift_from + 1) % 9;
    }
    fish_by_days_left.into_iter().sum()
}

pub fn run(input: &str) {
    let input: Vec<usize> = input
        .split(",")
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let mut fish_by_days_left: [u64; 9] = [0; 9];
    for fish in input {
        fish_by_days_left[fish] += 1;
    }

    println!("Part 1: {}", expand_fish(fish_by_days_left.clone(), 80));
    println!("Part 2: {}", expand_fish(fish_by_days_left, 256));
}
