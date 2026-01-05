use criterion::{black_box, criterion_group, criterion_main, Criterion};
use psyche_core::{compute_shuffled_index, ShuffleState};

fn bench_shuffling(c: &mut Criterion) {
    let mut group = c.benchmark_group("shuffling");
    let seed = [0u8; 32];

    for size in [100, 1000, 10_000] {
        let index = size / 2; // Middle index

        group.bench_with_input(
            criterion::BenchmarkId::new("single_shot", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    compute_shuffled_index(black_box(index), black_box(size), black_box(&seed))
                })
            },
        );

        group.bench_with_input(
            criterion::BenchmarkId::new("cached_state", size),
            &size,
            |b, &size| {
                let state = ShuffleState::new(size, seed);
                b.iter(|| state.shuffle(black_box(index)))
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_shuffling);
criterion_main!(benches);
