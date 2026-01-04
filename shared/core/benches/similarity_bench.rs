use criterion::{black_box, criterion_group, criterion_main, Criterion};
use psyche_core::{hamming_distance, jaccard_distance, manhattan_distance};
use rand::Rng;

fn generate_data(size: usize) -> (Vec<f32>, Vec<f32>) {
    let mut rng = rand::rng();
    let a: Vec<f32> = (0..size).map(|_| rng.random::<f32>()).collect();
    let b: Vec<f32> = (0..size).map(|_| rng.random::<f32>()).collect();
    (a, b)
}

fn similarity_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("similarity");

    // Standard Embeddings Size
    let size = 1024;
    let (a, b) = generate_data(size);

    group.bench_function("manhattan_1k", |bench| {
        bench.iter(|| manhattan_distance(black_box(&a), black_box(&b)))
    });

    group.bench_function("hamming_1k", |bench| {
        bench.iter(|| hamming_distance(black_box(&a), black_box(&b)))
    });

    group.bench_function("jaccard_1k", |bench| {
        bench.iter(|| jaccard_distance(black_box(&a), black_box(&b)))
    });

    // Small vectors
    let small_size = 100;
    let (sa, sb) = generate_data(small_size);
    group.bench_function("jaccard_100", |bench| {
        bench.iter(|| jaccard_distance(black_box(&sa), black_box(&sb)))
    });

    group.finish();
}

criterion_group!(benches, similarity_benchmark);
criterion_main!(benches);
