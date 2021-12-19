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
enum Node {
    Regular(u64),
    Pair(Box<Number>, Box<Number>),
}

#[derive(Debug, Clone)]
struct Number {
    max_depth: usize,
    node: Node,
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            max_depth: 1 + std::cmp::max(self.max_depth, rhs.max_depth),
            node: Node::Pair(Box::new(self), Box::new(rhs)),
        }
    }
}

impl Number {
    fn reduce(&mut self) {
        while self.explode(0).is_some() || self.split() {}
    }

    fn explode(&mut self, depth: usize) -> Option<(Option<u64>, Option<u64>)> {
        match &mut self.node {
            Node::Regular(_) => None,
            Node::Pair(l, r) => {
                match (&mut l.node, &mut r.node) {
                    (Node::Regular(nl), Node::Regular(nr)) if depth >= 4 => {
                        // We need to get the values out before mutating
                        let nl = *nl;
                        let nr = *nr;
                        *self = Self {
                            max_depth: 0,
                            node: Node::Regular(0),
                        };
                        Some((Some(nl), Some(nr)))
                    }
                    (Node::Pair(_, _), _) if l.max_depth + depth >= 4 => {
                        let (explode_left, explode_right) = l.explode(depth + 1).unwrap(); // we know this'll succeed based on the depth
                        if let Some(explode_right) = explode_right {
                            r.add_to_leftmost(explode_right);
                        }
                        self.max_depth = 1 + std::cmp::max(l.max_depth, r.max_depth);
                        Some((explode_left, None))
                    }
                    (_, Node::Pair(_, _)) if r.max_depth + depth >= 4 => {
                        let (explode_left, explode_right) = r.explode(depth + 1).unwrap(); // we know this'll succeed based on the depth
                        if let Some(explode_left) = explode_left {
                            l.add_to_rightmost(explode_left);
                        }
                        self.max_depth = 1 + std::cmp::max(l.max_depth, r.max_depth);
                        Some((None, explode_right))
                    }
                    _ => None,
                }
            }
        }
    }

    fn split(&mut self) -> bool {
        match &mut self.node {
            Node::Regular(n) if *n >= 10 => {
                *self = Self {
                    max_depth: 0,
                    node: Node::Regular(*n / 2),
                } + Self {
                    max_depth: 0,
                    node: Node::Regular(if *n % 2 == 0 { *n / 2 } else { *n / 2 + 1 }),
                };
                true
            }
            Node::Regular(_) => false,
            Node::Pair(l, r) => {
                let did_split = l.split() || r.split(); // takes advantage of short-circuiting
                self.max_depth = 1 + std::cmp::max(l.max_depth, r.max_depth);
                did_split
            }
        }
    }

    fn add_to_leftmost(&mut self, val: u64) {
        match &mut self.node {
            Node::Regular(n) => *n += val,
            Node::Pair(l, _) => l.add_to_leftmost(val),
        }
    }

    fn add_to_rightmost(&mut self, val: u64) {
        match &mut self.node {
            Node::Regular(n) => *n += val,
            Node::Pair(_, r) => r.add_to_rightmost(val),
        }
    }

    fn magnitude(&self) -> u64 {
        match &self.node {
            Node::Regular(n) => *n,
            Node::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }
}

fn parse_number(input: &str) -> IResult<&str, Number> {
    alt((
        map(nom::character::complete::u64, |n| Number {
            max_depth: 0,
            node: Node::Regular(n),
        }),
        map(
            delimited(
                tag("["),
                separated_pair(parse_number, tag(","), parse_number),
                tag("]"),
            ),
            |(l, r)| l + r,
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
