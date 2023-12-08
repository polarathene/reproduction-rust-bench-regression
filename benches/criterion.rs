use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

fn into_string_fold(c: &mut Criterion) {
    let id = BenchmarkId::new("criterion", "into_string_fold");
    let data = bench_regression::Map::new();

    c.bench_with_input(id, &data, |b, d| {
        b.iter(|| bench_regression::fold(&d));
    });
}

criterion_group!(benches, into_string_fold);
criterion_main!(benches);
