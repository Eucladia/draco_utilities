use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, Throughput};

use draco_utilities::globals::escape;

criterion_group!(benches, decode_group);
criterion_main!(benches);

static ENCODED_STRING: &str = "abc123\u{0107}./-2-_2-32%348äöü";

fn decode_group(c: &mut Criterion) {
  let mut group = c.benchmark_group("escape string");

  group.throughput(Throughput::Bytes(ENCODED_STRING.len() as u64));
  group.bench_function("escape", |b| {
    b.iter_batched(
      || Vec::with_capacity(150),
      |mut bytes| black_box(escape(ENCODED_STRING.as_bytes(), &mut bytes)),
      BatchSize::SmallInput,
    )
  });

  group.finish();
}
