#[allow(unused)]
use itertools::Itertools;

pub fn run(input: &str) {
    let (x_range, y_range) = input
        .split_once(": ")
        .unwrap()
        .1
        .split(", ")
        .map(|dimension| {
            let (start, end) = dimension
                .split_once('=')
                .unwrap()
                .1
                .split_once("..")
                .unwrap();
            (start.parse::<i64>().unwrap(), end.parse::<i64>().unwrap())
        })
        .collect_tuple()
        .unwrap();

    let candidate_xs = 0..=x_range.1;
    let candidate_ys = -81..5000; // this'll probably work

    let result1 = {
        Itertools::cartesian_product(candidate_xs.clone(), candidate_ys.clone())
            .filter_map(|(mut xstep, mut ystep)| {
                let mut pos = (0, 0);
                let mut max_height = i64::MIN;
                let mut landed_inside = false;
                while pos.0 <= x_range.1 && pos.1 >= y_range.0 {
                    if (x_range.0..=x_range.1).contains(&pos.0)
                        && (y_range.0..=y_range.1).contains(&pos.1)
                    {
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
                let mut max_height = i64::MIN;
                let mut landed_inside = false;
                while pos.0 <= x_range.1 && pos.1 >= y_range.0 {
                    if (x_range.0..=x_range.1).contains(&pos.0)
                        && (y_range.0..=y_range.1).contains(&pos.1)
                    {
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
