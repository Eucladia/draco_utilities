use std::ops::{Bound, RangeBounds};

const NUMS_0: u64 = 0x9E377A00;
const NUMS_1: u64 = 0x5851F42D4C957F2D;
const NUMS_2: u64 = 0xA0761D6478BD642F;
const NUMS_3: u64 = 0xE7037ED1A0B428DB;

/// The pseudorandom number generator.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Rng {
  state: u64,
}

impl Rng {
  /// Creates a [`Rng`] instance with the `seed` set to `0`.
  #[inline]
  pub const fn new() -> Self {
    Rng::with_seed(0)
  }

  /// Creates a [`Rng`] instance with the given seed.
  #[inline]
  pub const fn with_seed(seed: u64) -> Self {
    let state = seed
      .wrapping_sub(NUMS_0)
      .rotate_right(7)
      .wrapping_mul(NUMS_1);

    Rng { state }
  }

  /// Generates an unsigned 128 bit integer.
  #[inline]
  pub fn gen_u128(&mut self) -> u128 {
    ((self.gen_u64() as u128) << 64) | self.gen_u64() as u128
  }

  /// Generates an unsigned 64 bit integer.
  #[inline]
  pub fn gen_u64(&mut self) -> u64 {
    // We could just have the generating code in the u32 implementation and do
    // `((gen_u32() as u64) << 32) | gen_u32() as u64)` but that reduces performance by about 50%.
    self.state = self.state.wrapping_add(NUMS_2);

    let b = self.state.wrapping_mul(NUMS_3);
    let t = (self.state as u128).wrapping_mul(b as u128);

    ((t >> 64) ^ t) as u64
  }

  /// Generates an unsigned 32 bit integer.
  #[inline]
  pub fn gen_u32(&mut self) -> u32 {
    self.gen_u64() as u32
  }

  /// Generates an unsigned 32 bit integer that is within the range `[low, high)`.
  ///
  /// # Notes
  /// This function panics if the starting bound is larger than the end bound.
  pub fn gen_u32_in_range(&mut self, bounds: impl RangeBounds<u32>) -> u32 {
    #[cold]
    fn invalid_range(s: u32, e: u32) -> ! {
      panic!("start ({}) is greater than end ({})", s, e);
    }

    let start = match bounds.start_bound() {
      Bound::Unbounded => u32::MIN,
      Bound::Included(x) => *x,
      Bound::Excluded(x) => x.wrapping_add(1),
    };
    let end = match bounds.end_bound() {
      Bound::Unbounded => u32::MAX,
      Bound::Included(x) => *x,
      Bound::Excluded(x) => x.wrapping_sub(1),
    };

    if start > end {
      invalid_range(start, end);
    }

    let len = end.wrapping_sub(start).wrapping_add(1);

    start.wrapping_add(self.gen_capped_u32(len))
  }

  /// Generates an unsigned 32 bit integer that is in the range `[0, cap)`.
  // Uses a (nearly) division-less implementation, created by Daniel Lemire,
  // https://lemire.me/blog/2019/06/06/nearly-divisionless-random-integer-generation-on-various-systems/.
  #[inline]
  pub fn gen_capped_u32(&mut self, cap: u32) -> u32 {
    let mut rand = self.gen_u32();
    let mut high = mul_high_u32(rand, cap);
    let mut low = rand.wrapping_mul(cap);

    if low < cap {
      let threshold = cap.wrapping_neg() % cap;

      while low < threshold {
        rand = self.gen_u32();
        high = mul_high_u32(rand, cap);
        low = rand.wrapping_mul(cap);
      }
    }

    high
  }

  /// Generates a 32 bit float in the range `[0, 1)`.
  #[inline]
  pub fn gen_f32(&mut self) -> f32 {
    const EXPONENT_WIDTH: u32 = 8;
    const SIGN_WIDTH: u32 = 1;
    let exp_mask = 0x3F800000;
    let rand_u32 = self.gen_u32();

    f32::from_bits(exp_mask | (rand_u32 >> (EXPONENT_WIDTH + SIGN_WIDTH))) - 1.0
  }

  /// Generates a 64 bit float in the range `[0, 1)`.
  #[inline]
  pub fn gen_f64(&mut self) -> f64 {
    const EXPONENT_WIDTH: u64 = 11;
    const SIGN_WIDTH: u64 = 1;
    let exp_mask = 0x3FF0000000000000;
    let rand_u64 = self.gen_u64();

    f64::from_bits(exp_mask | (rand_u64 >> (EXPONENT_WIDTH + SIGN_WIDTH))) - 1.0
  }
}

#[inline]
const fn mul_high_u32(a: u32, b: u32) -> u32 {
  (((a as u64) * (b as u64)) >> 32) as u32
}
