use std::ops::RangeInclusive;

#[allow(unused)]
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::i32 as parse_i32,
    combinator::map,
    sequence::{preceded, separated_pair},
    IResult,
};

struct Area {
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
}

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<i32>> {
    map(
        separated_pair(parse_i32, tag(".."), parse_i32),
        |(start, end)| start..=end,
    )(input)
}

fn parse_area(input: &str) -> IResult<&str, Area> {
    preceded(
        tag("target area: "),
        map(
            separated_pair(
                preceded(tag("x="), parse_range),
                tag(", "),
                preceded(tag("y="), parse_range),
            ),
            |(x_range, y_range)| Area { x_range, y_range },
        ),
    )(input)
}

pub fn run(input: &str) {
    let area = parse_area(input).unwrap().1;

    let candidate_xs = 0..=*area.x_range.end();
    let candidate_ys = -81..5000; // this'll probably work

    let result1 = {
        Itertools::cartesian_product(candidate_xs.clone(), candidate_ys.clone())
            .filter_map(|(mut xstep, mut ystep)| {
                let mut pos = (0, 0);
                let mut max_height = i32::MIN;
                let mut landed_inside = false;
                while pos.0 <= *area.x_range.end() && pos.1 >= *area.y_range.start() {
                    if area.x_range.contains(&pos.0) && area.y_range.contains(&pos.1) {
                        landed_inside = true;
                    }
                    max_height = std::cmp::max(max_height, pos.1);
                    pos.0 += xstep;
                    pos.1 += ystep;
                    xstep = std::cmp::max(xstep - 1, 0);
                    ystep -= 1;
                }
                if landed_inside {
                    Some(max_height)
                } else {
                    None
                }
            })
            .max()
            .unwrap()
    };

    println!("Part 1: {}", result1);

    let result2 = {
        Itertools::cartesian_product(candidate_xs, candidate_ys)
            .filter_map(|(mut xstep, mut ystep)| {
                let mut pos = (0, 0);
                let mut max_height = i32::MIN;
                let mut landed_inside = false;
                while pos.0 <= *area.x_range.end() && pos.1 >= *area.y_range.start() {
                    if area.x_range.contains(&pos.0) && area.y_range.contains(&pos.1) {
                        landed_inside = true;
                    }
                    max_height = std::cmp::max(max_height, pos.1);
                    pos.0 += xstep;
                    pos.1 += ystep;
                    xstep = std::cmp::max(xstep - 1, 0);
                    ystep -= 1;
                }
                if landed_inside {
                    Some(max_height)
                } else {
                    None
                }
            })
            .count()
    };

    println!("Part 2: {}", result2);
}
