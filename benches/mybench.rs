use boringtun::noise::tests::tests::wireguard_handshake;
use criterion::{black_box, criterion_group, criterion_main, Criterion, ParameterizedBenchmark};

fn bench_handshake(c: &mut Criterion) {
    c.bench(
        "Level 0",
        ParameterizedBenchmark::new(
            "Handshake",
            |b, param| b.iter(|| {
                black_box(wireguard_handshake(*param));
            }),
            vec![0u32, 1u32, 2u32, 10000000u32],
        )
    );
    //c.bench_function("WireGuard Handshake", |b| b.iter(|| {
    //    black_box(wireguard_handshake());
    //}));
}

criterion_group!(benches, bench_handshake);
criterion_main!(benches);
