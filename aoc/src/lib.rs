use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

/// Gets the lines of the file `filename` as a series of strings
pub fn read_lines<P>(filename: P) -> impl Iterator<Item = String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().map(Result::unwrap)
}

// Re-export common dependencies so we don't need to compile them for each day
pub use itertools;
pub use nom;
pub use regex;
pub use serde;
pub use serde_json;
