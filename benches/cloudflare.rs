// Copyright (c) 2019 Cloudflare, Inc. All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause

use boringtun::crypto::blake2s::*;
use boringtun::crypto::chacha20poly1305::*;
use boringtun::crypto::pqcrypto::*;
use boringtun::crypto::x25519::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_x25519_public_key(c: &mut Criterion) {
    let secret_key = X25519SecretKey::new();

    c.bench_function("x25519 Public Key", |b| b.iter(|| {
        black_box(secret_key.public_key());
    }));
}

fn bench_x25519_shared_key(c: &mut Criterion) {
    let secret_key = X25519SecretKey::new();
    let public_key = X25519SecretKey::new().public_key();

    c.bench_function("x25519 Shared Key", |b| b.iter(|| {
        black_box(secret_key.shared_key(&public_key));
    }));
}

fn bench_pq_keypair(c: &mut Criterion) {
    c.bench_function("PQ Key Pair", |b| b.iter(|| {
        let keypair = black_box(PQKeyPair::new());
    }));
}

fn bench_pq_encaps(c: &mut Criterion) {
    let keypair = PQKeyPair::new();
    let public_key = keypair.public_key;

    c.bench_function("PQ Encapsulate", |b| b.iter(|| {
        black_box(public_key.encaps());
    }));
}

fn bench_pq_decaps(c: &mut Criterion) {
    let keypair = PQKeyPair::new();
    let secret_key = keypair.secret_key;
    let public_key = keypair.public_key;
    let (ciphertext, _) = public_key.encaps();

    c.bench_function("PQ Decapsulate", |b| b.iter(|| {
        black_box(ciphertext.decaps(&secret_key));
    }));
}

fn bench_blake2s_hash_128b(c: &mut Criterion) {
    let data = [0_u8; 128];
    c.bench_function("Blake2s Hash 128b", |b| b.iter(|| black_box(
                Blake2s::new_hash().hash(&data).finalize())));
}

fn bench_blake2s_hash_1024b(c: &mut Criterion) {
    let data = [0_u8; 1024];
    c.bench_function("Blake2s Hash 1024b", |b| b.iter(|| black_box(
                Blake2s::new_hash().hash(&data).finalize())));
}

fn bench_chacha20poly1305_seal_192b(c: &mut Criterion) {
    let pc = ChaCha20Poly1305::new_aead(&[
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32,
    ]);
    let pt = [0_u8; 192];
    let mut ct = [0_u8; 192 + 16];
    c.bench_function("Chacha20Poly1305 Seal 192b", |b| b.iter(|| {
        black_box(pc.seal_wg(0, &[], &pt, &mut ct));
    }));
}

fn bench_chacha20poly1305_open_192b(c: &mut Criterion) {
    let pc = ChaCha20Poly1305::new_aead(&[
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32,
    ]);
    let mut pt = [0_u8; 192];
    let mut ct = [0_u8; 192 + 16];

    pc.seal_wg(0, &[], &pt, &mut ct);

    c.bench_function("Chacha20Poly1305 Open 192b", |b| b.iter(|| {
        black_box(pc.open_wg(0, &[], &ct, &mut pt).unwrap());
    }));
}

fn bench_chacha20poly1305_seal_512b(c: &mut Criterion) {
    let pc = ChaCha20Poly1305::new_aead(&[
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32,
    ]);
    let pt = [0_u8; 512];
    let mut ct = [0_u8; 512 + 16];
    c.bench_function("Chacha20Poly1305 Seal 512b", |b| b.iter(|| {
        black_box(pc.seal_wg(0, &[], &pt, &mut ct));
    }));
}

fn bench_chacha20poly1305_seal_8192b(c: &mut Criterion) {
    let pc = ChaCha20Poly1305::new_aead(&[
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32,
    ]);
    let pt = [0_u8; 8192];
    let mut ct = [0_u8; 8192 + 16];
    c.bench_function("Chacha20Poly1305 Seal 8192b", |b| b.iter(|| {
        black_box(pc.seal_wg(0, &[], &pt, &mut ct));
    }));
}

criterion_group!(benches, bench_x25519_public_key, bench_x25519_shared_key,
                 bench_pq_keypair, bench_pq_encaps, bench_pq_decaps,
                 bench_blake2s_hash_128b, bench_blake2s_hash_1024b,
                 bench_chacha20poly1305_seal_192b,
                 bench_chacha20poly1305_open_192b,
                 bench_chacha20poly1305_seal_512b,
                 bench_chacha20poly1305_seal_8192b);
criterion_main!(benches);
