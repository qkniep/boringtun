use boringtun::noise::tests::tests::wireguard_handshake;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_handshake(c: &mut Criterion) {
    c.bench_function("WireGuard Handshake", |b| b.iter(|| {
        black_box(wireguard_handshake());
    }));
}

criterion_group!(benches, bench_handshake);
criterion_main!(benches);
