use criterion::{black_box, criterion_group, criterion_main, Criterion};
use psyche_core::FixedVec;

fn fixed_vec_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("fixed_vec");

    let data: Vec<u64> = (0..1000).collect();

    group.bench_function("extend_1k", |b| {
        b.iter(|| {
            let mut vec: FixedVec<u64, 1024> = FixedVec::new();
            vec.extend(black_box(data.iter().copied())).unwrap();
            vec
        })
    });

    group.bench_function("extend_from_slice_1k", |b| {
        b.iter(|| {
            let mut vec: FixedVec<u64, 1024> = FixedVec::new();
            vec.extend_from_slice(black_box(&data)).unwrap();
            vec
        })
    });

    // Comparison with standard Vec (though FixedVec is on stack/fixed size, meaningful as baseline)
    group.bench_function("std_vec_extend_1k", |b| {
        b.iter(|| {
            let mut vec: Vec<u64> = Vec::with_capacity(1024);
            vec.extend(black_box(data.iter().copied()));
            vec
        })
    });

    group.finish();
}

criterion_group!(benches, fixed_vec_benchmark);
criterion_main!(benches);
