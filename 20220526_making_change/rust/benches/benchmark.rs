use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rust::*;

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Comparison");
    let test_cases = [
        (1000, vec![1, 5]),
        (1000, vec![1, 5, 10]),
        (1000, vec![1, 5, 10, 20]),
        (1000, vec![1, 5, 10, 20, 40]),
        (1000, vec![1, 5, 10, 20, 40, 80]),
        (1000, vec![1, 5, 10, 20, 40, 80, 160]),
        (1000, vec![1, 5, 10, 20, 40, 80, 160, 320]),
        (1000, vec![1, 5, 10, 20, 40, 80, 160, 320, 640]),
    ];
    for i in test_cases.iter() {
        group.bench_with_input(
            BenchmarkId::new("Recursive", format!("{:?}", i)), i, 
            |b, i| b.iter(|| make_change_rec(i.0, &i.1))
        );
        group.bench_with_input(
            BenchmarkId::new("Depth-First Search", format!("{:?}", i)), i, 
            |b, i| b.iter(|| make_change_rec(i.0, &i.1))
        );
        group.bench_with_input(
            BenchmarkId::new("Dynamic Programming", format!("{:?}", i)), i, 
            |b, i| b.iter(|| make_change_rec(i.0, &i.1))
        );
    }
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
