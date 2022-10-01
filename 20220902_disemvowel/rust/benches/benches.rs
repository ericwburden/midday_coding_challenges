use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rust::{disemvowel_build, disemvowel_join_str, disemvowel_join_vec};

fn bench_disemvowel(c: &mut Criterion) {
    let mut group = c.benchmark_group("Disemvowelling");
    let start_string = String::from("A good place to start.");
    for i in (0..=5000).step_by(1000).skip(1) {
        let s = start_string.repeat(i);
        group.bench_with_input(BenchmarkId::new("Builder Pattern", i), &s, 
            |b, s| b.iter(|| disemvowel_build(s)));
        group.bench_with_input(BenchmarkId::new("Join to String", i), &s, 
            |b, s| b.iter(|| disemvowel_join_str(s)));
        group.bench_with_input(BenchmarkId::new("Join Vector", i), &s, 
            |b, s| b.iter(|| disemvowel_join_vec(s)));
    }
    group.finish();
}

criterion_group!(benches, bench_disemvowel);
criterion_main!(benches);