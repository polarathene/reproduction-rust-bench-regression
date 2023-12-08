use divan::{Divan, Bencher};

fn main() {
    // divan::main();

    // This will lower and better stablize `fastest` timings,
    // However the regression is still relative (consistently about 20-30ns higher timings)
    Divan::default()
        .sample_count(1000)
        .sample_size(128)
        .run_benches();
}

#[divan::bench]
fn into_string_fold(b: Bencher) {
    b.with_inputs(|| {
        bench_regression::Map::new()
    }).bench_refs(|data| {
        // Only reproduces when this logic lives in a separate scope (crate)
        bench_regression::fold(&data);
    });
}
