use aoc::criterion::{criterion_group, criterion_main, Criterion};
use day01::{parse_input, part1, part2};

pub fn bench(c: &mut Criterion) {
    c.bench_function("part1", |b| b.iter(|| part1(parse_input())));
    c.bench_function("part2", |b| b.iter(|| part2(parse_input())));
}

criterion_group!(benches, bench);
criterion_main!(benches);
