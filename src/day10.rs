use itertools::Itertools;

fn parse_line(s: &str) -> Vec<char> {
    s.chars().collect()
}

fn error_score(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!("These are the only options"),
    }
}

fn incomplete_score(c: char) -> u64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => unreachable!("These are the only options"),
    }
}

fn check_syntax(delimiters: Vec<char>) -> Result<Vec<char>, char> {
    let mut stack = Vec::with_capacity(delimiters.len());
    let mut remaining = &delimiters[..];
    while let [first, rest @ ..] = remaining {
        match (stack.last(), *first) {
            (Some('('), ')') | (Some('['), ']') | (Some('{'), '}') | (Some('<'), '>') => {
                stack.pop();
            }
            (_, c @ ('(' | '[' | '{' | '<')) => {
                stack.push(c);
            }
            (_, c) => return Err(c),
        }
        remaining = rest;
    }
    Ok(stack)
}

pub fn run(input: &str) {
    let input: Vec<_> = input.lines().map(parse_line).collect();

    let (mut incomplete_scores, errors): (Vec<_>, Vec<_>) = input
        .into_iter()
        .map(check_syntax)
        .map_ok(|remaining| {
            remaining
                .into_iter()
                .rev()
                .fold(0, |acc, c| 5 * acc + incomplete_score(c))
        })
        .partition_result();

    let result1 = errors.into_iter().map(error_score).sum::<u64>();

    println!("Part 1: {}", result1);

    let result2 = {
        let len = incomplete_scores.len();
        *incomplete_scores.select_nth_unstable(len / 2).1
    };

    println!("Part 2: {}", result2);
}
