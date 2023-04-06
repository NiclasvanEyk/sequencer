use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use sequencer::builders::vector_with_holes;
use sequencer::sequential_search::fill_sequence;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sequential_search");
    for sequence in [
        vector_with_holes(10, 1),
        vector_with_holes(100, 1),
        vector_with_holes(1000, 1),
        vector_with_holes(10000, 1),
        vector_with_holes(100000, 1),
        vector_with_holes(1000000, 1),
        vector_with_holes(10000000, 1),
    ]
    .iter()
    {
        group.bench_with_input(
            BenchmarkId::from_parameter(sequence.len()),
            sequence,
            |b, s| b.iter(|| fill_sequence(&mut s.clone())),
        );
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
