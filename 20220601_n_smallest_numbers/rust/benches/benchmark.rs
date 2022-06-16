#![feature(result_into_ok_or_err)]
extern crate rand;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rust::*;

const N_SIZES: [usize; 5] = [1, 16, 128, 512, 1024];

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Comparison");
    let mut numbers = Vec::new();
    (0..10_000).for_each(|_| numbers.push(rand::random()));
    for i in N_SIZES {
        group.bench_with_input(BenchmarkId::new("With Sort", i), &i, |b, i| {
            b.iter(|| n_smallest_numbers_sort(*i, &numbers))
        });
        group.bench_with_input(BenchmarkId::new("Without Sort", i), &i, |b, i| {
            b.iter(|| n_smallest_numbers_nosort(*i, &numbers))
        });
    }
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
