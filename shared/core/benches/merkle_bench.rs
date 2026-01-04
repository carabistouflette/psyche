use criterion::{black_box, criterion_group, criterion_main, Criterion};
use psyche_core::MerkleTree;

fn merkle_tree_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("merkle_tree");

    // Benchmark creation of Merkle Trees with different leaf counts
    for size in [100, 1000, 10_000].iter() {
        let count = *size;
        // Generate simple deterministic data
        let input: Vec<Vec<u8>> = (0..count)
            .map(|i| (i as u64).to_le_bytes().to_vec())
            .collect();

        group.bench_function(format!("new_{}", count), |b| {
            b.iter(|| MerkleTree::new(black_box(&input)))
        });
    }

    // Benchmark path finding (generating proofs)
    let size = 10_000;
    let input: Vec<Vec<u8>> = (0..size)
        .map(|i| (i as u64).to_le_bytes().to_vec())
        .collect();
    let tree = MerkleTree::new(&input);

    group.bench_function("find_path_10k_middle", |b| {
        b.iter(|| tree.find_path(black_box(5000)))
    });

    group.finish();
}

criterion_group!(benches, merkle_tree_benchmark);
criterion_main!(benches);
