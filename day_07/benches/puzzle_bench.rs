use criterion::{criterion_group, criterion_main, Criterion};
use day_07::process_part_two;
use std::fs;

fn run_part_two() {
    let input = fs::read_to_string("input.txt").expect("error loading the input file");
    process_part_two(&input);
}

fn bench_part_two(c: &mut Criterion) {
    c.bench_function("day_07 part two", |b| b.iter(|| run_part_two()));
}

criterion_group!(benches, bench_part_two);
criterion_main!(benches);
