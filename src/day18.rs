use std::ops::Add;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, Clone)]
enum Number {
    Regular(u64),
    Pair(Box<Number>, Box<Number>),
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Pair(Box::new(self), Box::new(rhs))
    }
}

impl Number {
    fn reduce(&mut self) {
        loop {
            let res = self.explode(0);
            if res.is_some() {
                continue;
            };
            let res = self.split();
            if !res {
                break;
            }
        }
    }

    fn explode(&mut self, depth: usize) -> Option<(Option<u64>, Option<u64>)> {
        match self {
            Self::Regular(_) => None,
            Self::Pair(l, r) => match (&mut **l, &mut **r) {
                (Self::Regular(nl), Self::Regular(nr)) if depth >= 4 => {
                    // We need to get the values out before mutating
                    let nl = *nl;
                    let nr = *nr;
                    *self = Self::Regular(0);
                    Some((Some(nl), Some(nr)))
                }
                (l, r) => match l.explode(depth + 1) {
                    Some((explode_left, explode_right)) => {
                        if let Some(explode_right) = explode_right {
                            r.add_to_leftmost(explode_right);
                        }
                        Some((explode_left, None))
                    }
                    None => match r.explode(depth + 1) {
                        Some((explode_left, explode_right)) => {
                            if let Some(explode_left) = explode_left {
                                l.add_to_rightmost(explode_left)
                            }
                            Some((None, explode_right))
                        }
                        None => None,
                    },
                },
            },
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Self::Regular(n) if *n >= 10 => {
                *self = Self::Regular(*n / 2)
                    + Self::Regular(if *n % 2 == 0 { *n / 2 } else { *n / 2 + 1 });
                true
            }
            Self::Regular(_) => false,
            Self::Pair(l, r) => l.split() || r.split(), // takes advantage of short-circuiting
        }
    }

    fn add_to_leftmost(&mut self, val: u64) {
        match self {
            Number::Regular(n) => *n += val,
            Number::Pair(l, _) => l.add_to_leftmost(val),
        }
    }

    fn add_to_rightmost(&mut self, val: u64) {
        match self {
            Number::Regular(n) => *n += val,
            Number::Pair(_, r) => r.add_to_rightmost(val),
        }
    }

    fn magnitude(&self) -> u64 {
        match self {
            Self::Regular(n) => *n,
            Self::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }
}

fn parse_number(input: &str) -> IResult<&str, Number> {
    alt((
        map(nom::character::complete::u64, Number::Regular),
        map(
            delimited(
                tag("["),
                separated_pair(parse_number, tag(","), parse_number),
                tag("]"),
            ),
            |(l, r)| Number::Pair(Box::new(l), Box::new(r)),
        ),
    ))(input)
}

pub fn run(input: &str) {
    let input: Vec<_> = input
        .lines()
        .map(|line| parse_number(&line).unwrap().1)
        .collect();

    let result1 = {
        let sum = input
            .clone()
            .into_iter()
            .reduce(|l, r| {
                let mut res = l + r;
                res.reduce();
                res
            })
            .unwrap();
        sum.magnitude()
    };

    println!("Part 1: {}", result1);

    let result2 = {
        input
            .into_iter()
            .permutations(2)
            .map(|permutation| {
                permutation
                    .into_iter()
                    .reduce(|l, r| {
                        let mut res = l + r;
                        res.reduce();
                        res
                    })
                    .unwrap()
                    .magnitude()
            })
            .max()
            .unwrap()
    };

    println!("Part 2: {}", result2);
}
