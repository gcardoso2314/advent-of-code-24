use criterion::{criterion_group, criterion_main, Criterion};

use day_03::{process_part_one, process_part_two};

fn bench_part_one(c: &mut Criterion) {
    c.bench_function("day_03 part one", |b| {
        b.iter(|| process_part_one(include_str!("../input.txt")))
    });
}

fn bench_part_two(c: &mut Criterion) {
    c.bench_function("day_03 part two", |b| {
        b.iter(|| process_part_two(include_str!("../input.txt")))
    });
}

criterion_group!(benches, bench_part_one, bench_part_two);
criterion_main!(benches);
