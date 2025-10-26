use criterion::{black_box, criterion_group, criterion_main, Criterion};
use monar::generate_greeting;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("generate_greeting_no_name", |b| {
        b.iter(|| generate_greeting(black_box(None)))
    });

    c.bench_function("generate_greeting_with_name", |b| {
        b.iter(|| generate_greeting(black_box(Some("benchmark"))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
