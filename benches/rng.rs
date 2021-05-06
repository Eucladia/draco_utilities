use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, Throughput};

use draco_utilities::rng::Rng;

criterion_group!(benches, gen_rng, gen_rng_group);
criterion_main!(benches);

fn gen_rng(c: &mut Criterion) {
  let mut rng = Rng::new();
  let mut group = c.benchmark_group("rng");

  group.bench_function("gen u64", |b| b.iter(|| rng.gen_u64()));
  group.bench_function("gen u32", |b| b.iter(|| rng.gen_u32()));
  group.bench_function("gen f64", |b| b.iter(|| rng.gen_f64()));
  group.bench_function("gen f32", |b| b.iter(|| rng.gen_f32()));

  group.finish();
}

fn gen_rng_group(c: &mut Criterion) {
  const AMOUNT: u64 = 1_000_000;

  let mut group = c.benchmark_group("rng group");

  group.throughput(Throughput::Bytes(AMOUNT));

  group.bench_function("gen u128", |b| {
    b.iter_batched(
      || Rng::new(),
      |mut rng| {
        for _ in 0..AMOUNT {
          black_box(rng.gen_u128());
        }
      },
      BatchSize::SmallInput,
    )
  });
  group.bench_function("gen u64", |b| {
    b.iter_batched(
      || Rng::new(),
      |mut rng| {
        for _ in 0..AMOUNT {
          black_box(rng.gen_u64());
        }
      },
      BatchSize::SmallInput,
    )
  });
  group.bench_function("gen u32", |b| {
    b.iter_batched(
      || Rng::new(),
      |mut rng| {
        for _ in 0..AMOUNT {
          black_box(rng.gen_u32());
        }
      },
      BatchSize::SmallInput,
    )
  });
  group.bench_function("gen u32 [0, n)", |b| {
    b.iter_batched(
      || Rng::new(),
      |mut rng| {
        for n in 0..AMOUNT {
          black_box(rng.gen_u32_in_range(0..n as u32));
        }
      },
      BatchSize::SmallInput,
    )
  });
  group.bench_function("gen f32", |b| {
    b.iter_batched(
      || Rng::new(),
      |mut rng| {
        for _ in 0..AMOUNT {
          black_box(rng.gen_f32());
        }
      },
      BatchSize::SmallInput,
    )
  });
  group.bench_function("gen f64", |b| {
    b.iter_batched(
      || Rng::new(),
      |mut rng| {
        for _ in 0..AMOUNT {
          black_box(rng.gen_f64());
        }
      },
      BatchSize::SmallInput,
    )
  });

  group.finish();
}
