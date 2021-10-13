use rust::{is_sqrt_robust, is_sqrt_naive};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_naive_approach(c: &mut Criterion) {
    c.bench_function("is_sqrt_naive 1_000_000", |b| b.iter(|| is_sqrt_naive(black_box(1_000_000))));
}

fn benchmark_robust_approach(c: &mut Criterion) {
    c.bench_function("is_sqrt_robust 1_000_000", |b| b.iter(|| is_sqrt_robust(black_box(1_000_000))));
}

criterion_group!(benches, benchmark_naive_approach, benchmark_robust_approach);
criterion_main!(benches);
