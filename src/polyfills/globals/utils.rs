use crate::polyfills::number::radii::HEXADECIMAL_RADIX;
use crate::polyfills::number::BASE_36_LUT;

/// Converts a byte to it's zero padded hexadecimal representation.
#[inline]
pub fn byte_to_hex(mut byte: u8) -> [u8; 2] {
  // Pre-emptively pad.
  let mut hex_buffer = [b'0', b'0'];
  let mut index = hex_buffer.len();

  // LLVM unrolls this better than a manually unrolled version lol
  loop {
    let value = byte % HEXADECIMAL_RADIX;
    let hex_byte = BASE_36_LUT[value as usize];

    index -= 1;
    hex_buffer[index] = hex_byte;
    byte = (byte - value) / HEXADECIMAL_RADIX;

    if byte == 0 {
      break;
    }
  }

  hex_buffer
}

/// Decodes a UTF-16 character to 2 UTF-8 characters.
#[inline]
pub fn decode_two_octets(val: u16) -> [u8; 2] {
  [((val >> 6) | 0xC0) as u8, ((val & 0x3F) | 0x80) as u8]
}

/// Converts two bytes into a hex value.
///
/// For performance reasons, this function does no validation on the provided input. However,
/// on invalid hex values, this function will return a number greater than [`u8::MAX`].
pub const fn hex_value(one: u8, two: u8) -> u32 {
  ((HEX_TABLE[one as usize] as u32) << 4) | (HEX_TABLE[two as usize] as u32)
}

// Hex LUT
#[rustfmt::skip]
const HEX_TABLE: [u16; 256] = [
  u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX,
  u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX,
  u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX,
  u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX,
  u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX,
  u16::MAX, u16::MAX, u16::MAX, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, u16::MAX, u16::MAX, u16::MAX,
  u16::MAX, u16::MAX, u16::MAX, u16::MAX, 10, 11, 12, 13, 14, 15, u16::MAX, u16::MAX, u16::MAX,
  u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX,
  u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX,
  u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, 10, 11, 12, 13, 14, 15, u16::MAX, u16::MAX,
  u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX,
  u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX,
  u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX,
  u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX,
  u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX,
  u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX,
  u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX,
  u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX,
  u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX,
  u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX,
  u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX,
  u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX,
  u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX,
  u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX,
  u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX,
  u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX,
  u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX,
];
