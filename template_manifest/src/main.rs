use std::{fs::OpenOptions, io::Write, path::PathBuf};

use structopt::StructOpt;

/// A dumb script to initialize the default Cargo.toml for a day
#[derive(StructOpt)]
struct Options {
    /// The path to the Cargo.toml
    manifest_path: PathBuf,
}

fn main() {
    let options = Options::from_args();

    // Dependencies are at the bottom of the default Cargo.toml, so just stick
    // this at the end of the file. I tried using the manifest crate, but it
    // left a bunch of nasty default values behind. This is also a lot shorter.
    OpenOptions::new()
        .append(true)
        .open(options.manifest_path)
        .expect("Could not open Cargo.toml")
        .write_all(r#"aoc = { path = "../aoc" }"#.as_bytes())
        .expect("Failed to write to Cargo.toml")
}
