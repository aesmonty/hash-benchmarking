use criterion::{black_box, criterion_group, criterion_main, Criterion};
use uniform_rand::{rn_gen, rn_gen_blake, TIME, UPPER, rn_gen_blake3};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Blake3", |b| b.iter(|| rn_gen_blake3(black_box(TIME),black_box(UPPER))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);