mod to_string;
/// Number related utilities.
pub mod utils;

pub use to_string::to_string;
#[doc(inline)]
pub use utils::*;

/// Base 36 lookup table.
pub static BASE_36_LUT: &[u8; radii::HEXATRIDECIMAL_RADIX as usize] =
  b"0123456789ABCDEFGHIJKLMNOPQRSTUWVXYZ";
