use std::{
    fs,
    path::{Path, PathBuf},
};

use chrono::{Datelike, FixedOffset, Utc};
use structopt::StructOpt;

mod day01;
mod day02;
mod day03;
mod day04;

#[derive(StructOpt)]
enum Options {
    All,
    Today,
    Day { day: u32 },
}

fn current_day_december_2021() -> Option<u32> {
    let eastern_tz = FixedOffset::west(5 * 3600); // Eastern Canada / US
    let date = Utc::now().with_timezone(&eastern_tz).date();
    if date.year() == 2021 && date.month() == 12 && date.day() <= 25 {
        Some(date.day())
    } else {
        None
    }
}

impl Options {
    fn days(&self) -> Vec<u32> {
        let today = current_day_december_2021();
        match self {
            Options::All => (1..=today.unwrap_or(25)).collect(),
            Options::Today => {
                let today =
                    today.expect("This option only works from December 1st through 25th, 2021");
                vec![today]
            }
            Options::Day { day } => {
                assert!(*day <= today.unwrap_or(25), "You can't run a future day!");
                vec![*day]
            }
        }
    }
}

// Just use a hardcoded table for now
const DAY_TABLE: [fn(&str); 4] = [day01::run, day02::run, day03::run, day04::run];

pub fn input_file(day: u32) -> PathBuf {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    crate_root.join("input").join(format!("{}.input", day))
}

fn main() {
    let options = Options::from_args();
    let days = options.days();

    for day in days {
        let input = fs::read_to_string(input_file(day)).expect("Failed to read input file");
        println!("Day {}", day);
        DAY_TABLE[day as usize - 1](&input);
    }
}
