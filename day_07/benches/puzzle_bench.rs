use criterion::{criterion_group, criterion_main, Criterion};
use day_07::process_part_two;

fn bench_part_two(c: &mut Criterion) {
    c.bench_function("day_07 part two", |b| {
        b.iter(|| process_part_two(include_str!("../input.txt")))
    });
}

criterion_group!(benches, bench_part_two);
criterion_main!(benches);
