use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn read_lines<P>(filename: P) -> impl Iterator<Item = String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().map(Result::unwrap)
}

fn input_path() -> PathBuf {
    const crate_root: Path = Path::new(env!("CARGO_MANIFEST_DIR"));
    crate_root.join("input")
}

fn main() {
    let input: Vec<_> = read_lines(input_path()).collect();
}
