use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, Throughput};

use draco_utilities::globals::encode_uri;

criterion_group!(benches, decode_group);
criterion_main!(benches);

static ENCODED_STRING: &str = "https://developer.mozilla.org/ru/docs/JavaScript_шеллы";

fn decode_group(c: &mut Criterion) {
  let mut group = c.benchmark_group("encode string");

  group.throughput(Throughput::Bytes(ENCODED_STRING.len() as u64));
  group.bench_function("encode_uri", |b| {
    b.iter_batched(
      || Vec::with_capacity(200),
      |mut bytes| black_box(encode_uri(ENCODED_STRING.as_bytes(), &mut bytes)),
      BatchSize::SmallInput,
    )
  });

  group.finish();
}
