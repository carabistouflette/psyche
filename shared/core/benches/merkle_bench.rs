use criterion::{black_box, criterion_group, criterion_main, Criterion};
use psyche_core::MerkleTree;
use rand::Rng;

fn bench_merkle_tree(c: &mut Criterion) {
    let mut group = c.benchmark_group("merkle_tree");
    let mut rng = rand::thread_rng();

    // Test different sizes
    for size in [100, 1000, 10_000, 100_000] {
        let items: Vec<Vec<u8>> = (0..size)
            .map(|_| (0..32).map(|_| rng.gen()).collect())
            .collect();

        group.bench_with_input(
            criterion::BenchmarkId::new("new", size),
            &items,
            |b, items| b.iter(|| MerkleTree::new(black_box(items))),
        );
    }

    group.finish();
}

criterion_group!(benches, bench_merkle_tree);
criterion_main!(benches);
