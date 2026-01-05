use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use psyche_core::MerkleTree;
use rand::Rng;

fn merkle_tree_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("merkle_tree");
    let mut rng = rand::rng();

    // Benchmark creation of Merkle Trees with different leaf counts
    for size in [100, 1000, 10_000, 100_000].iter() {
        let count = *size;
        let input: Vec<Vec<u8>> = (0..count)
            .map(|_| (0..32).map(|_| rng.random()).collect())
            .collect();

        group.bench_with_input(BenchmarkId::new("new", count), &input, |b, input| {
            b.iter(|| MerkleTree::new(black_box(input)))
        });
    }

    // Benchmark path finding (generating proofs)
    let size = 10_000;
    let input: Vec<Vec<u8>> = (0..size)
        .map(|_| (0..32).map(|_| rng.random()).collect())
        .collect();
    let tree = MerkleTree::new(&input);

    group.bench_function("find_path_10k_middle", |b| {
        b.iter(|| tree.find_path(black_box(5000)))
    });

    group.finish();
}

criterion_group!(benches, merkle_tree_benchmark);
criterion_main!(benches);
