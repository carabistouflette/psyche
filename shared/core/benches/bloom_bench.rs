use criterion::{black_box, criterion_group, criterion_main, Criterion};
use psyche_core::Bloom;
use rand::Rng; // Keep this, but also ensure rand crate is available via extern crate if needed (2021 edition shouldn't need it)

fn bench_bloom(c: &mut Criterion) {
    let mut group = c.benchmark_group("bloom");

    // Setup Bloom filter parameters (Standard usage: K=7 for 1% FP rate?)
    // Let's use K=8 as in tests.
    let keys: Vec<u64> = (0..8).map(|_| rand::random()).collect();
    let num_bits = 1024 * 64; // arbitrary size

    // We need strict const generics for Bloom<U, K>.
    // U is array size in u64s. 64KB bits = 8192 bytes = 1024 u64s.
    const SIZE: usize = 1024;
    const K: usize = 8;

    // Pre-generate items
    let items: Vec<Vec<u8>> = (0..1000)
        .map(|_| (0..32).map(|_| rand::random()).collect())
        .collect();

    group.bench_function("add_1000", |b| {
        b.iter(|| {
            let mut bloom = Bloom::<SIZE, K>::new(black_box(num_bits), black_box(&keys));
            for item in &items {
                bloom.add(item);
            }
            bloom
        })
    });

    group.bench_function("contains_1000_hits", |b| {
        let mut bloom = Bloom::<SIZE, K>::new(num_bits, &keys);
        for item in &items {
            bloom.add(item);
        }
        b.iter(|| {
            for item in &items {
                black_box(bloom.contains(item));
            }
        })
    });

    group.finish();
}

criterion_group!(benches, bench_bloom);
criterion_main!(benches);
