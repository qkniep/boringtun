use boringtun::noise::tests::tests::{wireguard_handshake, wireguard_handshake_custom};
use criterion::{black_box, criterion_group, criterion_main, Criterion, ParameterizedBenchmark};
use std::time::Duration;

fn bench_handshake(c: &mut Criterion) {
    c.bench(
        "Level 3",
        ParameterizedBenchmark::new(
            "NPackets",
            |b, param| b.iter(|| {
                black_box(wireguard_handshake(*param));
            }),
            vec![65536u32],
        )
    );

    c.bench_function("Handshake", |b| b.iter_custom(|iters| {
        let mut total_time = Duration::default();
        for _ in 0..iters {
            total_time += black_box(wireguard_handshake_custom());
        }
        total_time
    }));
}

criterion_group!(benches, bench_handshake);
criterion_main!(benches);
