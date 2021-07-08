use criterion::{black_box, criterion_group, criterion_main, Criterion};
use uniform_rand::{
    my_blake2b_hash, my_blake2s_hash, my_blake3_hash, my_sha3_hash, rounding_algo, TIME, UPPER,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("my_sha3_hash", |b| {
        b.iter(|| rounding_algo(black_box(TIME), black_box(UPPER), black_box(my_sha3_hash)))
    });
    c.bench_function("my_blake2s_hash", |b| {
        b.iter(|| {
            rounding_algo(
                black_box(TIME),
                black_box(UPPER),
                black_box(my_blake2s_hash),
            )
        })
    });
    c.bench_function("my_blake2b_hash", |b| {
        b.iter(|| {
            rounding_algo(
                black_box(TIME),
                black_box(UPPER),
                black_box(my_blake2b_hash),
            )
        })
    });
    c.bench_function("my_blake3_hash", |b| {
        b.iter(|| rounding_algo(black_box(TIME), black_box(UPPER), black_box(my_blake3_hash)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
