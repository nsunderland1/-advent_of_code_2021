use std::ops::RangeInclusive;

use itertools::{iproduct, Itertools};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::i64 as parse_i64,
    combinator::{map, value},
    sequence::{pair, preceded, separated_pair, tuple},
    IResult,
};

#[derive(Debug, Clone)]
struct Step {
    state: bool,
    x: RangeInclusive<isize>,
    y: RangeInclusive<isize>,
    z: RangeInclusive<isize>,
}

fn parse_state(input: &str) -> IResult<&str, bool> {
    alt((value(true, tag("on")), value(false, tag("off"))))(input)
}

fn parse_range<'a>(
    label: &'static str,
) -> impl FnMut(&'a str) -> IResult<&'a str, RangeInclusive<isize>> {
    preceded(
        pair(tag(label), tag("=")),
        map(
            separated_pair(parse_i64, tag(".."), parse_i64),
            |(start, end)| start as isize..=end as isize,
        ),
    )
}

fn parse_step(input: &str) -> IResult<&str, Step> {
    map(
        separated_pair(
            parse_state,
            tag(" "),
            tuple((
                parse_range("x"),
                tag(","),
                parse_range("y"),
                tag(","),
                parse_range("z"),
            )),
        ),
        |(state, (x, _, y, _, z))| Step { state, x, y, z },
    )(input)
}

fn index((x, y, z): (isize, isize, isize)) -> Option<(usize, usize, usize)> {
    let (x, y, z) = (x + 50, y + 50, z + 50);
    if x >= 0 && y >= 0 && z >= 0 && x <= 100 && y <= 100 && z <= 100 {
        Some((x as usize, y as usize, z as usize))
    } else {
        None
    }
}

#[derive(Debug, Clone)]
enum EventKind {
    Start,
    End,
}

#[derive(Debug, Clone)]
struct Event {
    kind: EventKind,
    position: isize,
    step_id: usize,
}

pub fn run(input: &str) {
    let steps: Vec<_> = input
        .lines()
        .map(|line| parse_step(&line).unwrap().1)
        .collect();

    let mut grid = vec![vec![vec![false; 101]; 101]; 101];

    let result1 = {
        for step in steps.iter() {
            if *step.x.start() > 50
                || *step.x.end() < -50
                || *step.y.start() > 50
                || *step.y.end() < -50
                || *step.z.start() > 50
                || *step.z.end() < -50
            {
                continue;
            }
            for (x, y, z) in
                iproduct!(step.x.clone(), step.y.clone(), step.z.clone()).filter_map(index)
            {
                grid[x][y][z] = step.state;
            }
        }

        grid.iter()
            .flat_map(|plane| plane.iter().map(|row| row.iter().filter(|v| **v).count()))
            .sum::<usize>()
    };

    println!("Part 1: {}", result1);

    // TODO: works for my input, is somehow off by 10 for the example input
    let result2 = {
        let x_events = steps
            .iter()
            .enumerate()
            .flat_map(|(i, step)| {
                [
                    Event {
                        kind: EventKind::Start,
                        position: *step.x.start(),
                        step_id: i,
                    },
                    Event {
                        kind: EventKind::End,
                        position: *step.x.end(),
                        step_id: i,
                    },
                ]
            })
            .sorted_by_key(|event| event.position);

        let y_events = steps
            .iter()
            .enumerate()
            .map(|(i, step)| {
                [
                    Event {
                        kind: EventKind::Start,
                        position: *step.y.start(),
                        step_id: i,
                    },
                    Event {
                        kind: EventKind::End,
                        position: *step.y.end(),
                        step_id: i,
                    },
                ]
            })
            .collect_vec();

        let z_events = steps
            .iter()
            .enumerate()
            .map(|(i, step)| {
                [
                    Event {
                        kind: EventKind::Start,
                        position: *step.z.start(),
                        step_id: i,
                    },
                    Event {
                        kind: EventKind::End,
                        position: *step.z.end(),
                        step_id: i,
                    },
                ]
            })
            .collect_vec();

        let mut active_steps: Vec<usize> = Vec::with_capacity(steps.len());
        let mut area_start = isize::MIN;
        let mut total_cubes = 0usize;

        for event in x_events {
            let y_events = active_steps
                .iter()
                .flat_map(|&i| y_events[i].clone())
                // TODO: be smarter and avoid re-sorting every time?
                .sorted_by_key(|event| event.position);

            let mut y_cubes = 0;
            {
                let mut active_steps: Vec<usize> = Vec::with_capacity(active_steps.len());
                let mut area_start = isize::MIN;

                for event in y_events {
                    let z_events = active_steps
                        .iter()
                        .flat_map(|&i| z_events[i].clone())
                        // TODO: be smarter and avoid re-sorting every time?
                        .sorted_by_key(|event| event.position);

                    let mut z_cubes = 0;
                    {
                        let mut active_steps: Vec<usize> = Vec::with_capacity(active_steps.len());
                        let mut area_start = isize::MIN;

                        for event in z_events {
                            if let Some(max) = active_steps.iter().copied().max() {
                                if steps[max].state {
                                    z_cubes += match event.kind {
                                        EventKind::Start => event.position - area_start,
                                        EventKind::End => event.position - area_start + 1,
                                    }
                                }
                            }

                            match event.kind {
                                EventKind::Start => {
                                    active_steps.push(event.step_id);
                                    area_start = event.position;
                                }
                                EventKind::End => {
                                    active_steps.swap_remove(
                                        active_steps
                                            .iter()
                                            .position(|&step| step == event.step_id)
                                            .unwrap(),
                                    );
                                    area_start = event.position + 1;
                                }
                            };
                        }
                    }

                    match event.kind {
                        EventKind::Start => {
                            active_steps.push(event.step_id);
                            z_cubes *= event.position - area_start;
                            area_start = event.position;
                        }
                        EventKind::End => {
                            active_steps.swap_remove(
                                active_steps
                                    .iter()
                                    .position(|&step| step == event.step_id)
                                    .unwrap(),
                            );
                            z_cubes *= event.position - area_start + 1;
                            area_start = event.position + 1;
                        }
                    };

                    y_cubes += z_cubes;
                }
            }

            match event.kind {
                EventKind::Start => {
                    active_steps.push(event.step_id);
                    y_cubes *= event.position - area_start;
                    area_start = event.position;
                }
                EventKind::End => {
                    active_steps.swap_remove(
                        active_steps
                            .iter()
                            .position(|&step| step == event.step_id)
                            .unwrap(),
                    );
                    y_cubes *= event.position - area_start + 1;
                    area_start = event.position + 1;
                }
            };

            total_cubes += y_cubes as usize;
        }

        total_cubes
    };

    println!("Part 2: {}", result2);
}
