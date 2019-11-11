use boringtun::noise::tests::tests::{wireguard_handshake, wireguard_handshake_custom};
use criterion::{black_box, criterion_group, criterion_main, Criterion, ParameterizedBenchmark};

fn bench_handshake(c: &mut Criterion) {
    c.bench(
        "Level 3",
        ParameterizedBenchmark::new(
            "Handshake",
            |b, param| b.iter(|| {
                black_box(wireguard_handshake(*param));
            }),
            vec![0u32, 1u32, 10u32, 20u32, 40u32, 60u32, 80u32, 100u32],
        )
    );

    c.bench_function("Custom Handshake", |b| b.iter_custom(|iters| {
        black_box(wireguard_handshake_custom(iters))
    }));
}

criterion_group!(benches, bench_handshake);
criterion_main!(benches);
