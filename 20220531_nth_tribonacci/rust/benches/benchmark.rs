use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rust::*;

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Comparison");
    let seed = [0, 0, 1];
    for i in [5u64, 10, 20, 40, 80].iter() {
        group.bench_with_input(BenchmarkId::new("Vector-Backed", i), i, |b, i| {
            b.iter(|| nth_tribonacci_vec(*i, seed))
        });
        group.bench_with_input(BenchmarkId::new("Array-Backed", i), i, |b, i| {
            b.iter(|| nth_tribonacci_arr(*i, seed))
        });
        group.bench_with_input(BenchmarkId::new("Primitives", i), i, |b, i| {
            b.iter(|| nth_tribonacci_primitive(*i, seed))
        });
        group.bench_with_input(BenchmarkId::new("In-Place Insert", i), i, |b, i| {
            b.iter(|| nth_tribonacci_inplace(*i, seed))
        });
        group.bench_with_input(BenchmarkId::new("In-Place Primitives", i), i, |b, i| {
            b.iter(|| nth_tribonacci_primitive_mod(*i, seed))
        });
    }
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
