use itertools::Itertools;

fn parse_line(s: &str) -> Vec<char> {
    s.chars().collect()
}

fn closes(open: char, close: char) -> bool {
    close == closer_for(open)
}

fn closer_for(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!("These are the only options"),
    }
}

fn is_opener(c: char) -> bool {
    matches!(c, '(' | '[' | '{' | '<')
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
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!("These are the only options"),
    }
}

fn check_syntax(instructions: Vec<char>) -> Result<Vec<char>, char> {
    let mut stack = Vec::with_capacity(instructions.len());
    let mut remaining = &instructions[..];
    while let [first, rest @ ..] = remaining {
        match stack.last() {
            Some(&opener) if closes(opener, *first) => {
                stack.pop();
            }
            _ if is_opener(*first) => {
                stack.push(*first);
            }
            _ => return Err(*first),
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
                .fold(0, |acc, c| 5 * acc + incomplete_score(closer_for(c)))
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
