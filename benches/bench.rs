use std::fs::read_to_string;

use advent_of_code_2021::{get_input_file, run_day};
use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    c.bench_function("day 9", |b| {
        let input = read_to_string(get_input_file(9)).unwrap();
        b.iter(|| run_day(9, &input));
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
