// Adapted from https://github.com/sebastinas/ascon-aead/blob/main/ascon-aead/benches/asconbenches.rs
// Copyright 2022 Sebastian Ramacher
// SPDX-License-Identifier: MIT

use ascon_jasmin::{ascon128a_encrypt, ascon128_encrypt};
use criterion::{
    black_box, criterion_group, criterion_main, Bencher, BenchmarkId, Criterion, Throughput,
};
use criterion_cycles_per_byte::CyclesPerByte;
use rand::{rngs::StdRng, RngCore, SeedableRng};

const SIZES: [usize; 12] = [1, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 1536, 16384];

fn bench_for_size_ascon_128(b: &mut Bencher, rng: &mut dyn RngCore, size: usize) {
    let mut key = [0; 16];
    rng.fill_bytes(key.as_mut_slice());
    let mut nonce = [0; 16];
    rng.fill_bytes(nonce.as_mut_slice());
    let mut plaintext = vec![0u8; size];
    rng.fill_bytes(plaintext.as_mut_slice());
    let ad = vec![0u8; 0];

    b.iter(|| black_box(ascon128_encrypt(plaintext.clone(), ad.clone(), nonce, key)));
}

fn bench_for_size_ascon_128_a(b: &mut Bencher, rng: &mut dyn RngCore, size: usize) {
    let mut key = [0; 16];
    rng.fill_bytes(key.as_mut_slice());
    let mut nonce = [0; 16];
    rng.fill_bytes(nonce.as_mut_slice());
    let mut plaintext = vec![0u8; size];
    rng.fill_bytes(plaintext.as_mut_slice());
    let ad = vec![0u8; 0];

    b.iter(|| black_box(ascon128a_encrypt(plaintext.clone(), ad.clone(), nonce, key)));
}

fn criterion_benchmark_ascon_128_a(c: &mut Criterion) {
    let mut rng = StdRng::from_entropy();
    let mut group = c.benchmark_group("Ascon-128a");
    for size in SIZES.iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            bench_for_size_ascon_128_a(b, &mut rng, size)
        });
    }
    group.finish();
}

fn criterion_benchmark_ascon_128(c: &mut Criterion) {
    let mut rng = StdRng::from_entropy();
    let mut group = c.benchmark_group("Ascon-128");
    for size in SIZES.iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            bench_for_size_ascon_128(b, &mut rng, size)
        });
    }
    group.finish();
}

fn bench_for_cycles_ascon_128a(b: &mut Bencher<CyclesPerByte>, rng: &mut dyn RngCore, size: usize) {
    let mut key = [0; 16];
    rng.fill_bytes(key.as_mut_slice());
    let mut nonce = [0; 16];
    rng.fill_bytes(nonce.as_mut_slice());
    let mut plaintext = vec![0u8; size];
    rng.fill_bytes(plaintext.as_mut_slice());
    let ad = vec![0u8; 0];

    b.iter(|| black_box(ascon128a_encrypt(plaintext.clone(), ad.clone(), nonce, key)));
}

fn bench_for_cycles_ascon_128(b: &mut Bencher<CyclesPerByte>, rng: &mut dyn RngCore, size: usize) {
    let mut key = [0; 16];
    rng.fill_bytes(key.as_mut_slice());
    let mut nonce = [0; 16];
    rng.fill_bytes(nonce.as_mut_slice());
    let mut plaintext = vec![0u8; size];
    rng.fill_bytes(plaintext.as_mut_slice());
    let ad = vec![0u8; 0];

    b.iter(|| black_box(ascon128_encrypt(plaintext.clone(), ad.clone(), nonce, key)));
}

fn criterion_cycles_benchmark_ascon_128_a(c: &mut Criterion<CyclesPerByte>) {
    let mut rng = StdRng::from_entropy();
    let mut group = c.benchmark_group("Ascon-128a-Cycles");
    for size in SIZES.iter() {
        group.bench_function(BenchmarkId::from_parameter(size), |b| {
            bench_for_cycles_ascon_128a(b, &mut rng, *size)
        });
    }
    group.finish();
}

fn criterion_cycles_benchmark_ascon_128(c: &mut Criterion<CyclesPerByte>) {
    let mut rng = StdRng::from_entropy();
    let mut group = c.benchmark_group("Ascon-128-Cycles");
    for size in SIZES.iter() {
        group.bench_function(BenchmarkId::from_parameter(size), |b| {
            bench_for_cycles_ascon_128(b, &mut rng, *size)
        });
    }
    group.finish();
}

criterion_group!(
    bench_ascon128,
    criterion_benchmark_ascon_128
);
criterion_group!(
    bench_ascon128a,
    criterion_benchmark_ascon_128_a
);
criterion_group!(
    name = bench_ascon128_cycles;
    config = Criterion::default().with_measurement(CyclesPerByte);
    targets = criterion_cycles_benchmark_ascon_128
);
criterion_group!(
    name = bench_ascon128a_cycles;
    config = Criterion::default().with_measurement(CyclesPerByte);
    targets = criterion_cycles_benchmark_ascon_128_a
);
criterion_main!(bench_ascon128, bench_ascon128a, bench_ascon128_cycles, bench_ascon128a_cycles);
