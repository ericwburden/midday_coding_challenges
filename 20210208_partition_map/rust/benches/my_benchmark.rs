use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust::partition::find_partition_by_memory;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("find_partition big", |b| {
        b.iter(|| (0..200_000).map(|n| find_partition_by_memory(black_box(n))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
