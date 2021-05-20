use crate::number::radii::{BINARY_RADIX, HEXATRIDECIMAL_RADIX};
use crate::number::{from_character_byte, next_floating_point, BASE_36_LUT};

use std::mem::MaybeUninit;

/// Static assertion, particularly useful for constraining a min_const_generic
macro_rules! static_assert {
  ($imm:ident : $ty:ty where $e:expr) => {
    struct Validate<const $imm: $ty>();
    impl<const $imm: $ty> Validate<$imm> {
      const VALID: () = {
        let _ = 1 / ($e as usize);
      };
    }
    let _ = Validate::<$imm>::VALID;
  };
}

const ROUNDING_ERROR: f64 = 0.5;
const DEFAULT_BUFFER_SIZE: usize = 2200;

/// Converts a [`f64`] into bytes of the string representation in different radix.
///
/// # Examples
///
/// ```
/// use draco_utilities::number::float_to_custom_radix;
///
/// const RADIX: u8 = 16;
///
/// let mut bytes = Vec::new();
/// let float = 0.141592653589793;
///
/// float_to_custom_radix::<RADIX>(float, &mut bytes);
///
/// let string = std::str::from_utf8(&bytes).unwrap();
///
/// assert_eq!(string, "0.243f6a8885a2f8");
/// ```
///
/// # Notes
/// This function uses a static assertion and will fail at compile time if the radix
/// is not within [2, 36].
///
/// This function is an implementation of
/// [Number.prototype.toString](https://tc39.es/ecma262/#sec-number.prototype.tostring)
/// and uses a similar implementation as
/// [v8's](https://github.com/v8/v8/blob/master/src/numbers/conversions.cc#L1269).
pub fn float_to_custom_radix<const RADIX: u8>(num: f64, bytes: &mut Vec<u8>) {
  static_assert!(RADIX: u8 where RADIX >= BINARY_RADIX && RADIX <= HEXATRIDECIMAL_RADIX);

  if num.is_nan() {
    return bytes.extend_from_slice(b"NaN");
  }

  // IEEE-754 spec states that positive 0 and negative 0 should be equal, so we don't need to
  // explicitly check for -0.0
  if num == 0.0 {
    return bytes.extend_from_slice(b"0");
  }

  let is_negative = num.is_sign_negative();

  if num.is_infinite() {
    let inf_bytes: &[u8] = if is_negative {
      b"-Infinity"
    } else {
      b"Infinity"
    };

    return bytes.extend_from_slice(inf_bytes);
  }

  let abs_value = num.abs();
  // We can't cast to a u64 because f64 -> u64 is lossy, but we can use a u128 and the performance
  // on x64 is similar enough.
  // NOTE: f64::{floor,trunc} have function calls that aren't inlined (as of rustc 1.51.0). :/
  let mut integral = abs_value.floor() as u128;
  let mut fraction = abs_value.fract();
  // Multiply by `0.5` in order to determine later on whether the number should be rounded up.
  let mut delta =
    (ROUNDING_ERROR * (next_floating_point(abs_value) - abs_value)).max(next_floating_point(0.0));

  // TODO: Since we force the radix to be a const, we can compute a better estimate once we can use
  // generics from outer functions.

  // SAFETY: `MaybeUninit`s do not require initialization
  let mut temp_buffer: [MaybeUninit<u8>; DEFAULT_BUFFER_SIZE] =
    unsafe { MaybeUninit::uninit().assume_init() };
  // The number of starting bytes not written to.
  let mut count = DEFAULT_BUFFER_SIZE;
  // We need to store the end count for backtracking, this is different from `count` because this
  // gets decremented when there needs to be backtracking so that we don't add the no longer
  // needed bytes.
  let mut end_count = DEFAULT_BUFFER_SIZE;
  // We need to store the start of the fractional digits so we can reverse the order of them. It's
  // initialized to the current buffer length which will make the later `slice::reverse` call
  // not change anything if there were no fractional digits, plus there is no additional branch :D
  let mut fraction_start = DEFAULT_BUFFER_SIZE;

  if fraction >= delta {
    loop {
      fraction *= RADIX as f64;
      delta *= RADIX as f64;

      let byte = fraction as u8;
      let radix = get_radix_byte::<RADIX>(byte);

      count -= 1;
      temp_buffer[count] = MaybeUninit::new(radix);
      fraction -= byte as f64;

      // If the remainder is exactly 0.5 or the value is odd, then it needs to be rounded towards
      // even.
      #[allow(clippy::float_cmp)]
      if fraction > ROUNDING_ERROR || (fraction == ROUNDING_ERROR && (byte & 1) == 1) {
        // If it's greater than 1 then we are forced to carry over the remainder to the proceeding
        // digits.
        if fraction + delta > 1.0 {
          let mut backtrack_idx = count;
          let last_in_radix = get_radix_byte::<RADIX>(RADIX - 1);

          // We need to backtrace while the last digit is the largest value for that specific radix.
          // SAFETY: This would always be a valid index that has been initialized.
          while unsafe {
            backtrack_idx != DEFAULT_BUFFER_SIZE
              && *temp_buffer.get_unchecked(backtrack_idx).as_ptr() == last_in_radix
          } {
            backtrack_idx += 1;
            end_count -= 1;
          }

          // We've backtracked to as far as the theoretical decimal point so we need to carry over
          // to the integral.
          //
          // Eg: base 10 of `1.9999999999999999` to base 16 would become `2`.
          if backtrack_idx == DEFAULT_BUFFER_SIZE {
            // Need to decrease the end bound by 1 because the decimal point is *always* inserted
            end_count -= 1;
            integral += 1;
          } else {
            // SAFETY: This would always be a valid index that has been initialized.
            let current_byte = unsafe { *temp_buffer.get_unchecked(backtrack_idx).as_ptr() };
            let actual_value = from_character_byte(current_byte);
            let incremented_byte = get_radix_byte::<RADIX>(actual_value + 1);

            temp_buffer[backtrack_idx] = MaybeUninit::new(incremented_byte);
          }
          break;
        }
      } else if fraction < delta {
        break;
      }
    }

    fraction_start = count;
    count -= 1;
    temp_buffer[count] = MaybeUninit::new(b'.');
  }

  loop {
    let value = integral % RADIX as u128;
    let radix = get_radix_byte::<RADIX>(value as u8);

    count -= 1;
    temp_buffer[count] = MaybeUninit::new(radix);
    integral = (integral - value) / RADIX as u128;

    if integral == 0 {
      break;
    }
  }

  if is_negative {
    count -= 1;
    temp_buffer[count] = MaybeUninit::new(b'-');
  }

  // We don't need to have an end bound because of all of the bytes after this would
  // have been initialized already.
  // SAFETY: The starting index won't be greater than the maximum amount
  unsafe { temp_buffer.get_unchecked_mut(fraction_start..).reverse() };

  // SAFETY: MaybeUninit<u8> has the same layout and alignment as `u8` and the memory has been
  // initialized.
  let set_slice =
    unsafe { &*(temp_buffer.get_unchecked(count..end_count) as *const _ as *const [u8]) };

  bytes.extend_from_slice(set_slice);
}

fn get_radix_byte<const BASE: u8>(byte: u8) -> u8 {
  BASE_36_LUT[(byte % BASE) as usize] | (1 << 5)
}
