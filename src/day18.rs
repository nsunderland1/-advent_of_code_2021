use std::ops::Add;

#[allow(unused)]
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug)]
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

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Action {
    Exploded(Option<u64>, Option<u64>),
    Split,
    Kept,
}

impl Number {
    fn reduce(self) -> Self {
        let mut number = self;
        loop {
            let (next_number, action) = number.reduce_aux(0);
            number = next_number;
            println!("{}", number.pretty_print());
            if action == Action::Kept {
                break;
            }
        }
        number
    }

    fn reduce_aux(self, depth: usize) -> (Self, Action) {
        match self {
            Self::Regular(n) if n >= 10 => (Self::split(n), Action::Split),
            Self::Regular(_) => (self, Action::Kept),
            Self::Pair(l, r) => match (*l, *r) {
                (Self::Regular(nl), Self::Regular(nr)) if depth >= 4 => {
                    (Self::Regular(0), Action::Exploded(Some(nl), Some(nr)))
                }
                (l, r) => match l.reduce_aux(depth + 1) {
                    (l_reduced, Action::Kept) => match r.reduce_aux(depth + 1) {
                        (r_reduced, Action::Exploded(explode_left, explode_right)) => {
                            let l_added = if let Some(explode_left) = explode_left {
                                l_reduced.add_to_rightmost(explode_left)
                            } else {
                                l_reduced
                            };
                            (
                                Self::Pair(Box::new(l_added), Box::new(r_reduced)),
                                Action::Exploded(None, explode_right),
                            )
                        }
                        (r_reduced, action) => {
                            (Self::Pair(Box::new(l_reduced), Box::new(r_reduced)), action)
                        }
                    },
                    (l_reduced, Action::Split) => {
                        (Self::Pair(Box::new(l_reduced), Box::new(r)), Action::Split)
                    }
                    (l_reduced, Action::Exploded(explode_left, explode_right)) => {
                        let r_added = if let Some(explode_right) = explode_right {
                            r.add_to_leftmost(explode_right)
                        } else {
                            r
                        };
                        (
                            Self::Pair(Box::new(l_reduced), Box::new(r_added)),
                            Action::Exploded(explode_left, None),
                        )
                    }
                },
            },
        }
    }

    fn add_to_leftmost(self, val: u64) -> Self {
        match self {
            Number::Regular(n) => Number::Regular(n + val),
            Number::Pair(l, r) => Number::Pair(Box::new(l.add_to_leftmost(val)), r),
        }
    }

    fn add_to_rightmost(self, val: u64) -> Self {
        match self {
            Number::Regular(n) => Number::Regular(n + val),
            Number::Pair(l, r) => Number::Pair(l, Box::new(r.add_to_rightmost(val))),
        }
    }

    fn split(n: u64) -> Self {
        Self::Pair(
            Box::new(Self::Regular(n / 2)),
            Box::new(Self::Regular(if n % 2 == 0 { n / 2 } else { n / 2 + 1 })),
        )
    }

    fn magnitude(&self) -> u64 {
        match self {
            Self::Regular(n) => *n,
            Self::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }

    fn pretty_print(&self) -> String {
        match self {
            Self::Regular(n) => n.to_string(),
            Self::Pair(l, r) => format!("[{},{}]", l.pretty_print(), r.pretty_print()),
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
        let sum = input.into_iter().reduce(|l, r| (l + r).reduce()).unwrap();
        sum.magnitude()
    };

    println!("Part 1: {}", result1);

    let result2 = {
        // Part 2
        0
    };

    println!("Part 2: {}", result2);
}
