use criterion::*;
use rust::*;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn benchmark_pre_sorted(c: &mut Criterion) {
    let mut group = c.benchmark_group("Pre-Sorted Input");
    let base_input: Vec<_> = (1..100_000).into_iter().collect();

    let mut input = base_input.clone();
    group.bench_function(
        "Standard Sort",
        |b| b.iter(|| find_two_max_std_sort(black_box(&mut input)))
    );

    let mut input = base_input.clone();
    group.bench_function(
        "Unstable Sort",
        |b| b.iter(|| find_two_max_unstable_sort(black_box(&mut input)))
    );

    let mut input = base_input.clone();
    group.bench_function(
        "Naive Swap",
        |b| b.iter(|| find_two_max_naive_swap(black_box(&mut input)))
    );

    let mut input = base_input.clone();
    group.bench_function(
        "Memory Swap",
        |b| b.iter(|| find_two_max_mem_swap(black_box(&mut input)))
    );
    group.finish();
}

fn benchmark_rev_sorted(c: &mut Criterion) {
    let mut group = c.benchmark_group("Reverse Sorted Input");
    let base_input: Vec<_> = (1..100_000).into_iter().rev().collect();

    let mut input = base_input.clone();
    group.bench_function(
        "Standard Sort",
        |b| b.iter(|| find_two_max_std_sort(black_box(&mut input)))
    );

    let mut input = base_input.clone();
    group.bench_function(
        "Unstable Sort",
        |b| b.iter(|| find_two_max_unstable_sort(black_box(&mut input)))
    );

    let mut input = base_input.clone();
    group.bench_function(
        "Naive Swap",
        |b| b.iter(|| find_two_max_naive_swap(black_box(&mut input)))
    );

    let mut input = base_input.clone();
    group.bench_function(
        "Memory Swap",
        |b| b.iter(|| find_two_max_mem_swap(black_box(&mut input)))
    );
    group.finish();
}

fn benchmark_random_sorted(c: &mut Criterion) {
    let mut group = c.benchmark_group("Random Sorted Input");
    let mut base_input: Vec<_> = (1..100_000).into_iter().rev().collect();
    base_input.shuffle(&mut thread_rng());
    
    let mut input = base_input.clone();
    group.bench_function(
        "Naive Swap",
        |b| b.iter(|| find_two_max_naive_swap(black_box(&mut input)))
    );
    
    let mut input = base_input.clone();
    group.bench_function(
        "Memory Swap",
        |b| b.iter(|| find_two_max_mem_swap(black_box(&mut input)))
    );
    
    let mut input = base_input.clone();
    group.bench_function(
        "Standard Sort",
        |b| b.iter(|| find_two_max_std_sort(black_box(&mut input)))
    );
    
    let mut input = base_input.clone();
    group.bench_function(
        "Unstable Sort",
        |b| b.iter(|| find_two_max_unstable_sort(black_box(&mut input)))
    );
    group.finish();
}

pub fn sort_vs_swap(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sorting vs Swapping");
    let rep_steps = [1_000, 10_000, 100_000, 1_000_000, 10_000_000];

    for reps in rep_steps.iter() {
        let mut base_input: Vec<_> = (1..*reps).into_iter().collect();
        base_input.shuffle(&mut thread_rng());

        let mut input = base_input.clone();
        group.bench_function(
             BenchmarkId::new("Unstable Sort", reps),
             |b| b.iter(|| find_two_max_unstable_sort(black_box(&mut input)))
        );

        let mut input = base_input.clone();
        group.bench_function(
             BenchmarkId::new("Memory Swap", reps),
             |b| b.iter(|| find_two_max_mem_swap(black_box(&mut input)))
        );
    }
}

criterion_group!(benches,
    benchmark_pre_sorted,
    benchmark_rev_sorted,
    benchmark_random_sorted,
    sort_vs_swap);
criterion_main!(benches);

