use itertools::Itertools;

fn parse_line(s: &str) -> u32 {
    s.parse().unwrap()
}

fn part1(input: &Vec<u32>) -> usize {
    input.iter().tuple_windows().filter(|(l, r)| l < r).count()
}

fn part2(input: &Vec<u32>) -> usize {
    input
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(l, r)| l < r)
        .count()
}

pub fn run(input: &str) {
    let input = input.lines().map(parse_line).collect();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
