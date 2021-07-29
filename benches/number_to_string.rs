use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

use draco_utilities::number;

criterion_group!(
  benches,
  singular_radix,
  singular_radix_large_fractionless_num,
  singular_radix_large_num,
  backtrack,
  batched_radix,
);
criterion_main!(benches);

// Due to the inherent nature of optimizations/optimizers & micro-benchmarks, the
// benchmarks that use `Bencher::iter_batched` impact the other benchmarks, resulting in what
// appears to be "slower" execution time for the `singular_radix` and `singular_radix_batched`
// benchmarks.
//
// See:
//  - https://github.com/bheisler/criterion.rs/issues/300#issuecomment-498487813
//  - https://github.com/bheisler/criterion.rs/issues/290
fn singular_radix(c: &mut Criterion) {
  let mut bytes = Vec::with_capacity(2200);

  c.bench_function("float_to_radix (hex + singular)", |b| {
    b.iter(|| {
      black_box(number::to_string::<16>(2102.3230, &mut bytes));
    })
  });
}

fn singular_radix_large_fractionless_num(c: &mut Criterion) {
  let mut bytes = Vec::with_capacity(2200);

  c.bench_function(
    "float_to_radix (hex + singular + large fractionless number)",
    |b| {
      b.iter(|| {
        black_box(number::to_string::<16>(21023123232132.0, &mut bytes));
      })
    },
  );
}
fn singular_radix_large_num(c: &mut Criterion) {
  let mut bytes = Vec::with_capacity(2200);

  c.bench_function("float_to_radix (hex + singular + large number)", |b| {
    b.iter(|| {
      black_box(number::to_string::<16>(
        2102238732932.3231323930,
        &mut bytes,
      ));
    })
  });
}

fn backtrack(c: &mut Criterion) {
  let bytes = Vec::with_capacity(2200);

  c.bench_function("float_to_radix (backtrack)", |b| {
    b.iter_batched(
      || bytes.clone(),
      |mut bytes| {
        black_box(number::to_string::<3>(2.0 / 7.0, &mut bytes));
      },
      BatchSize::SmallInput,
    )
  });
}

fn batched_radix(c: &mut Criterion) {
  let bytes = Vec::with_capacity(2200);

  c.bench_function("float_to_radix (hex + batched)", |b| {
    b.iter_batched(
      || bytes.clone(),
      |mut bytes| {
        black_box(number::to_string::<16>(2102.3230, &mut bytes));
      },
      BatchSize::SmallInput,
    )
  });
}
