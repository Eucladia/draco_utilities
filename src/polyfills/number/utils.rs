/// The width of the mantissa in a 64 bit floating point number.
pub const DOUBLE_MANTISSA_WIDTH: u64 = 52;
/// The width of the exponent in a 64 bit floating point number.
pub const DOUBLE_EXPONENT_WIDTH: u64 = 11;
/// The width of the sign bit in a 64 bit floating point number.
pub const DOUBLE_SIGN_WIDTH: u64 = 1;
/// The position of the sign bit in 64 bit floating point number.
pub const DOUBLE_SIGN_BIT_POSITION: u64 = DOUBLE_MANTISSA_WIDTH + DOUBLE_EXPONENT_WIDTH;
/// The bias in a 64 bit floating point number.
pub const DOUBLE_BIAS: u64 = (1 << (DOUBLE_EXPONENT_WIDTH - 1)) - 1;
/// The implicit bit in the mantissa in a 64 bit floating point number.
pub const DOUBLE_IMPLICIT_BIT: u64 = 1 << DOUBLE_MANTISSA_WIDTH;

/// Breaks down a [`f64`] into it's sign, exponent, and mantissa bits.
///
/// # Notes
/// The sign bit will be `0` for positive numbers and `1` for negative numbers.
/// The exponent bits will not have the [bias](https://en.wikipedia.org/wiki/Exponent_bias).
/// The mantissa bits will have the [implicit bit](https://en.wikipedia.org/wiki/IEEE_754#Representation_and_encoding_in_memory) set.
///
/// This does not properly handle input that is
/// `NaN`, `+-Infinity`, or [Subnormal](https://en.wikipedia.org/wiki/Denormal_number)!
#[inline]
pub fn floating_point_parts(num: f64) -> (u64, u64, u64) {
  // Floats are roughly (1 + mantissa) * 2^exponent
  //
  // 3.14 => 0 10000000000 1001000111101011100001010001111010111000010100011111
  // ----------------------------------------------------------------------------------
  // | Sign | Exponent        | Mantissa                                              |
  // ----------------------------------------------------------------------------------
  // | 63   | 62           52 | 51                                                  0 |
  // ---------------------------------------------------------------------------------|
  // | 0    | 10000000000     | 1001000111101011100001010001111010111000010100011111  |
  // ----------------------------------------------------------------------------------
  let bits = num.to_bits();

  let sign_bit = bits >> DOUBLE_SIGN_BIT_POSITION;

  let exponent_mask = (1 << DOUBLE_EXPONENT_WIDTH) - 1;
  let exponent_bits = ((bits >> DOUBLE_MANTISSA_WIDTH) & exponent_mask) - DOUBLE_BIAS;

  let mantissa_mask = (1 << DOUBLE_MANTISSA_WIDTH) - 1;
  let mantissa_bits = (bits & mantissa_mask) | DOUBLE_IMPLICIT_BIT;

  (sign_bit, exponent_bits, mantissa_bits)
}

/// Converts the sign bit, exponent bits, and the mantissa bits into a [`f64`].
/// This will convert the result of [`floating_point_parts`] back to the original [`f64`].
///
/// # Notes
/// The bias bits will be added to the exponent bits and the implicit bit will be removed
/// from the mantissa bits.
#[inline]
pub fn from_floating_point_parts(sign: u64, exponent: u64, mantissa: u64) -> f64 {
  let mant = mantissa & !(DOUBLE_IMPLICIT_BIT);
  let exp = (exponent + DOUBLE_BIAS) << DOUBLE_MANTISSA_WIDTH;
  let sig = sign << DOUBLE_SIGN_BIT_POSITION;

  f64::from_bits(sig | exp | mant)
}

// With the way the ASCII table was made, we can easily convert a byte to the
// appropriate byte in the desired radix.
/// Converts a byte, in the range of `[0, 36)`, where a value of `0` represents `0` and a value
/// of `35` represents `z`.
///
/// # Notes
/// For alphabetical characters it is case insensitive, that is, both `b'z'` and `b'Z'` will
/// return 35.
#[inline]
pub const fn to_character_byte(mut byte: u8) -> u8 {
  // Since 0-9 and A-Z aren't continuous, we need to add the difference of
  // the characters that need to be ignored.
  if byte > 9 {
    byte += b'@' - b'9';
  }

  // Casing is determined by the 6th bit of the byte. With this in mind, we can normalize all bytes
  // to be lowercase by simply setting the 6th bit.
  (b'0' + byte) | (1 << 5)
}

/// Converts an alphanumeric character value to a byte in the range of `[0, 36)`, where a value of
/// `0` represents `0` and a value of `35` represents `z`.
///
/// # Notes
/// This function is the opposite of [`to_character_byte`].
#[inline]
pub const fn from_character_byte(mut byte: u8) -> u8 {
  byte |= 1 << 5;

  if byte > b'9' {
    byte -= b'a' - (b'9' + 1);
  }

  byte - b'0'
}

/// Gets the next  IEEE-754 representable [`f64`] after `num`.
///
/// # Notes
/// This will return [`f64::NAN`] on +-Infinity and `0.0` for negative subnormal numbers.
///
/// This relies on [Two's Complement](https://en.wikipedia.org/wiki/Two%27s_complement)!
// TODO: Mark as const once these FP operations are stabilized
#[inline]
pub fn next_floating_point(num: f64) -> f64 {
  if num.is_infinite() {
    return f64::NAN;
  }

  use std::num::FpCategory::*;

  // We need an arm for `Zero` because in the number `0.0`, all of the exponent bits are `0`
  // and the match in `f64::classify` checks against both a mantissa mask and an exponent mask,
  // so the arm that returns `Zero` gets caught first.
  //
  // It should be fine here since we check the sign bit as well
  if num.is_sign_negative() && matches!(num.classify(), Zero | Subnormal) {
    0.0
  } else {
    let bits = num.to_bits();
    // Instead of adding or subtracting 1 from the bits, we can achieve the same functionality by
    // adding with wrapping behavior. Two's complement is used on most modern hardware used today
    // so this *should* be fine. It's also a tad bit faster on x86.
    let sign: i64 = if num.is_sign_positive() { 1 } else { -1 };
    let next_number = bits.wrapping_add(sign as u64);

    f64::from_bits(next_number)
  }
}
