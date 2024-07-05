use criterion::{criterion_group, criterion_main, Criterion};
use rust_benchmarking_example::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("distance", |bench| {
        bench.iter(|| {
            let a = Vector2::new(0.0, 0.0);
            let b = Vector2::new(3.0, 4.0);

            a.distance(&b);
            // a.distance_squared(&b);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
