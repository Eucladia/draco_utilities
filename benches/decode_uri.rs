use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, Throughput};

use draco_utilities::polyfills::globals::decode_uri;

criterion_group!(benches, decode_group);
criterion_main!(benches);

static ENCODED_STRING: &str =
  "https://developer.mozilla.org/ru/docs/JavaScript_%D1%88%D0%B5%D0%BB%D0%BB%D1%8B";

fn decode_group(c: &mut Criterion) {
  let mut group = c.benchmark_group("decode string");

  group.throughput(Throughput::Bytes(ENCODED_STRING.len() as u64));
  group.bench_function("decode_uri", |b| {
    b.iter_batched(
      || Vec::with_capacity(150),
      |mut bytes| black_box(decode_uri(ENCODED_STRING.as_bytes(), &mut bytes)),
      BatchSize::SmallInput,
    )
  });

  group.finish();
}
