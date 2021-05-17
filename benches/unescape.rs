use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, Throughput};

use draco_utilities::polyfills::globals::unescape;

criterion_group!(benches, decode_group);
criterion_main!(benches);

static ENCODED_STRING: &str = "abc123%u0107./-2-_2-32%25348%E4%F6%FC";

fn decode_group(c: &mut Criterion) {
  let mut group = c.benchmark_group("unescape string");

  group.throughput(Throughput::Bytes(ENCODED_STRING.len() as u64));
  group.bench_function("unescape", |b| {
    b.iter_batched(
      || Vec::with_capacity(150),
      |mut bytes| black_box(unescape(ENCODED_STRING.as_bytes(), &mut bytes)),
      BatchSize::SmallInput,
    )
  });

  group.finish();
}
