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

impl Area {
    fn contains(&self, (x, y): (i32, i32)) -> bool {
        self.x_range.contains(&x) && self.y_range.contains(&y)
    }
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

    let candidate_xs = (0..=*area.x_range.end()).filter(|&x| {
        let mut x_step = x;
        let mut x_pos = 0;
        while x_pos <= *area.x_range.end() {
            if area.x_range.contains(&x_pos) {
                return true;
            }
            if x_step == 0 {
                break;
            }
            x_pos += x_step;
            x_step -= 1;
        }
        false
    });

    let candidate_ys = *area.y_range.start()..area.y_range.start().abs();

    let hits = Itertools::cartesian_product(candidate_xs.clone(), candidate_ys.clone())
        .filter_map(|(mut xstep, mut ystep)| {
            let mut pos = (0, 0);
            let mut max_height = i32::MIN;
            let mut landed_inside = false;
            while pos.0 <= *area.x_range.end() && pos.1 >= *area.y_range.start() {
                if area.contains(pos) {
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
        .collect_vec();

    println!("Part 1: {}", hits.iter().max().unwrap());
    println!("Part 2: {}", hits.into_iter().count());
}
