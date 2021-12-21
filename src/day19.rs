use itertools::Itertools;
use nalgebra::{Point3, Vector3};
use std::collections::HashSet;

type Section = Vec<Point3<i32>>;

fn _count_matches(v1: &Section, v2: &Section) -> usize {
    let intersection: HashSet<_> = v1.iter().chain(v2.iter()).copied().collect();
    intersection.len()
}

fn _try_align_sections(_v1: &Section, _v2: &mut Section) -> bool {
    false
}

pub fn run(input: &str) {
    let _input: Vec<Section> = input
        .split("\n\n")
        .map(|scanner| {
            scanner
                .lines()
                .skip(1)
                .map(|line| {
                    Point3::from(Vector3::from_iterator(
                        line.split(',').map(str::parse::<i32>).map(Result::unwrap),
                    ))
                })
                .collect_vec()
        })
        .collect_vec();

    let result1 = {
        // Part 1
        0
    };

    println!("Part 1: {}", result1);

    let result2 = {
        // Part 2
        0
    };

    println!("Part 2: {}", result2);
}
