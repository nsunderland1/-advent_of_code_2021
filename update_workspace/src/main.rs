use std::{fs::OpenOptions, io::Write, path::PathBuf};

use cargo_toml::Manifest;
use serde::Serialize;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    /// The path to the workspace Cargo.toml
    manifest_path: PathBuf,
    /// The name of the crate to add to the workspace
    crate_name: String,
}

// This is crazy verbose for what should be a pretty simple task...
fn main() {
    let options = Options::from_args();
    let mut manifest = Manifest::from_path(&options.manifest_path).expect("Invalid manifest path");
    let members = &mut manifest
        .workspace
        .as_mut()
        .expect("Expected a workspace Cargo.toml, got a crate Cargo.toml")
        .members;

    if members.contains(&options.crate_name) {
        println!("Manifest was already updated!");
        return;
    }
    members.push(options.crate_name);

    let mut manifest_file = OpenOptions::new()
        .write(true)
        .open(options.manifest_path)
        .expect("Someone deleted Cargo.toml while we were running!");

    let mut out_buf = String::with_capacity(4096); // power of 2 so I look smart
    let mut serializer = toml::Serializer::pretty(&mut out_buf);
    let serializer = serializer.pretty_array_indent(2);

    manifest
        .serialize(serializer)
        .expect("This should be valid toml");

    manifest_file
        .write_all(out_buf.as_bytes())
        .expect("Failed to write to Cargo.toml");
}
