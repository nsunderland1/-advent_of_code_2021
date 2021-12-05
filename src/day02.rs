use itertools::Itertools;

fn parse_line(s: &str) -> (String, u32) {
    let s: (&str, &str) = s.split(" ").collect_tuple().unwrap();
    (s.0.to_string(), s.1.parse().unwrap())
}

pub fn run(input: &str) {
    let input: Vec<_> = input.lines().map(parse_line).collect();

    let result1 = {
        input
            .iter()
            .fold((0, 0), |(x, y), (dir, amt)| match dir.as_str() {
                "forward" => (x + amt, y),
                "down" => (x, y + amt),
                "up" => (x, y - amt),
                _ => unreachable!(),
            })
    };

    println!("Part 1: {}", result1.0 * result1.1);

    let result2 = {
        input
            .iter()
            .fold((0, 0, 0), |(x, y, aim), (dir, amt)| match dir.as_str() {
                "forward" => (x + amt, y + aim * amt, aim),
                "down" => (x, y, aim + amt),
                "up" => (x, y, aim - amt),
                _ => unreachable!(),
            })
    };

    println!("Part 2: {}", result2.0 * result2.1);
}
