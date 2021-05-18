mod custom_radix_float;
/// Number related utilities.
pub mod utils;

pub use custom_radix_float::float_to_custom_radix;
#[doc(inline)]
pub use utils::*;

/// Base 36 lookup table.
pub static BASE_36_LUT: &[u8; radii::HEXATRIDECIMAL_RADIX as usize] =
  b"0123456789ABCDEFGHIJKLMNOPQRSTUWVXYZ";

/// Commonly used radii.
pub mod radii {
  /// The radix used for converting to binary.
  pub const BINARY_RADIX: u8 = 2;
  /// The radix used for converting to octal.
  pub const OCTAL_RADIX: u8 = 8;
  /// The radix used for converting to decimal.
  pub const DECIMAL_RADIX: u8 = 10;
  /// The radix used for converting to hexadecimal.
  pub const HEXADECIMAL_RADIX: u8 = 16;
  /// The radix used for converting to hexatridecimal.
  pub const HEXATRIDECIMAL_RADIX: u8 = 36;
}
