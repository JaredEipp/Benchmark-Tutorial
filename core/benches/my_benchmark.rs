use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use test_bench_core::{fibonacci, fibonacci_iter};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci");
    for i in [20u64, 21u64].iter() {
        group.bench_with_input(BenchmarkId::new("Recursive", i), i, |b,i| b.iter(|| fibonacci(black_box(*i))));
        group.bench_with_input(BenchmarkId::new("Iterative", i), i, |b,i| b.iter(|| fibonacci_iter(black_box(*i))));

    }
    group.finish();
    
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);