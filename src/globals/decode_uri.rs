use crate::globals::utils::{decode_two_octets, hex_value};
use crate::globals::UriError;

/// Decodes a UTF-8 encoded URI, reserving any character in the set `#$&+,/:;=?@`.
///
/// # Notes
/// This function functionally behaves the same as
/// [JavaScript's decodeURI](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/decodeURI).
pub fn decode_uri(bytes: &[u8], decoded: &mut Vec<u8>) -> Result<(), UriError> {
  const RESERVED: &[u8] = b"#$&+,/:;=?@";

  decode_uri_inner(bytes, RESERVED, decoded)
}

/// Decodes a UTF-8 encoded URI.
///
/// # Notes
/// This function functionally behaves the same as
/// [JavaScript's decodeURIComponent](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/decodeURIComponent).
pub fn decode_uri_component(bytes: &[u8], decoded: &mut Vec<u8>) -> Result<(), UriError> {
  decode_uri_inner(bytes, &[], decoded)
}

/// Fast UTF-8 uri decode function with multi-byte and reserved character support.
///
/// This function follows the [EMCA-262 spec](https://262.ecma-international.org/11.0/#sec-decode)
/// for decoding URIs.
pub fn decode_uri_inner(
  bytes: &[u8],
  reserved: &[u8],
  decoded: &mut Vec<u8>,
) -> Result<(), UriError> {
  const UTF8_SURROGATE: u32 = 0x10000;
  // In a 4 byte utf-8 series, 10 bits are reserved for determining the number of octets there
  // actually are.
  const RESERVED_OCTETS_BITS: u32 = 10;

  let mut idx = 0;

  while idx < bytes.len() {
    // SAFETY: We'd have at least a single byte here.
    let byte = unsafe { *bytes.get_unchecked(idx) };

    if byte == b'%' {
      const NEXT: usize = 2;

      if idx + NEXT >= bytes.len() {
        return Err(UriError::InvalidUri);
      }

      // SAFETY: We error if there's not enough bytes remaining
      let next_byte = unsafe {
        let first = *bytes.get_unchecked(idx + 1);
        let second = *bytes.get_unchecked(idx + 2);

        hex_value(first, second)
      };

      idx += NEXT;

      // Gets the number of byte pairs
      let (bytes_needed, mask) = match next_byte {
        // One byte - We check for one byte here for perf reasons (avoids extra branching)
        // For one byte we can use the byte itself as the mask because the bits are the same
        x if x < 0x80 => (0, x),
        // Two bytes
        x if x < 0xE0 => (3, 0x1F),
        // Three bytes
        x if x < 0xF0 => (6, 0x0F),
        // Four bytes
        x if x < 0xF8 => (9, 0x07),
        // Invalid
        _ => return Err(UriError::InvalidUtf8Character),
      };

      if idx + bytes_needed >= bytes.len() {
        return Err(UriError::InvalidUri);
      }

      const CHUNKS: usize = 3;

      let mut total = next_byte & mask;

      for i in 0..bytes_needed / CHUNKS {
        // SAFETY: We check if we have enough bytes beforehand which also allows to
        // avoid branching here
        let first = unsafe { *bytes.get_unchecked(idx + (i * CHUNKS + NEXT)) };
        let second = unsafe { *bytes.get_unchecked(idx + (i * CHUNKS + NEXT + 1)) };
        let upcoming_byte = hex_value(first, second);

        if upcoming_byte & 0xC0 != 0x80 {
          return Err(UriError::InvalidUtf8Character);
        }

        total = (total << 6) + (upcoming_byte & 0x3F)
      }

      // Invalid unicode scalar value
      // Either a high or low surrogate or not a valid unicode character
      if total > 0xD7FF && total < 0xE000 || total > 0x10FFFF {
        return Err(UriError::InvalidUtf8Character);
      }

      idx += bytes_needed;

      // Get the high and low bits if needed
      if total < UTF8_SURROGATE {
        if reserved.iter().any(|&x| x as u32 == total) {
          decoded.extend_from_slice(&[b'%', unsafe { *bytes.get_unchecked(idx - 1) }, unsafe {
            *bytes.get_unchecked(idx)
          }]);
        } else {
          decoded.extend_from_slice(&decode_two_octets(total as u16));
        }
      } else {
        let low = ((total - UTF8_SURROGATE) & 0x3FF) + 0xDC00;
        let high = (((total - UTF8_SURROGATE) >> RESERVED_OCTETS_BITS) & 0x3FF) + 0xD800;
        let [hi_0, hi_1] = decode_two_octets(high as u16);
        let [lo_0, lo_1] = decode_two_octets(low as u16);

        decoded.extend_from_slice(&[hi_0, hi_1, lo_0, lo_1]);
      }
    } else {
      decoded.push(byte);
    }

    idx += 1;
  }

  Ok(())
}
