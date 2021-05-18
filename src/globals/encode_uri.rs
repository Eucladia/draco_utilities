use crate::globals::utils::byte_to_hex;
use crate::globals::UriError;

/// Encodes a UTF-8 URI, reserving any character in the set
/// `ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!#$&'()*+,-./:;=?@_~`.
///
/// # Notes
/// This function functionally behaves the same as
/// [JavaScript's encodeURI](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURI).
pub fn encode_uri(bytes: &[u8], encoded: &mut Vec<u8>) -> Result<(), UriError> {
  const RESERVED: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!#$&'()*+,-./:;=?@_~";

  encode_inner(bytes, RESERVED, encoded)
}

/// Encodes a UTF-8 URI, reserving any character in the set
/// `ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!'()*-._~`.
///
/// # Notes
/// This function functionally behaves the same as
/// [JavaScript's encodeURI](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURIComponent).
pub fn encode_uri_component(bytes: &[u8], encoded: &mut Vec<u8>) -> Result<(), UriError> {
  const RESERVED: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!'()*-._~";

  encode_inner(bytes, RESERVED, encoded)
}

/// Fast UTF-8 uri encode function with multi-byte and reserved character support.
///
/// This function follows the [EMCA-262 spec](https://tc39.es/ecma262/#sec-encode)
/// for encoding URIs.
pub fn encode_inner(bytes: &[u8], reserved: &[u8], encoded: &mut Vec<u8>) -> Result<(), UriError> {
  let mut index = 0;

  while index < bytes.len() {
    // SAFETY: Guaranteed to be valid because of `index < len`.
    let current = unsafe { *bytes.get_unchecked(index) };

    if reserved.contains(&current) {
      encoded.push(current);
      index += 1;
      continue;
    }

    let bytes_needed = match current {
      // One octet
      x if x < 0x80 => {
        // ASCII fast path
        encoded.extend_from_slice(&percent_hex(x));
        index += 1;
        continue;
      }
      // Two octets
      x if x < 0xE0 => 1,
      // Three octets
      x if x < 0xF0 => 2,
      // Four octets
      x if x < 0xF8 => 3,
      // Invalid
      _ => return Err(UriError::InvalidUtf8Character),
    };

    if index + bytes_needed >= bytes.len() {
      return Err(UriError::InvalidUri);
    }

    encoded.extend_from_slice(&percent_hex(current));

    for i in 0..bytes_needed {
      // SAFETY: We check if we have enough bytes beforehand.
      let byte = unsafe { *bytes.get_unchecked(index + i + 1) };

      // It should be a continuation byte
      if byte & 0xC0 != 0x80 {
        return Err(UriError::InvalidUtf8Character);
      }

      encoded.extend_from_slice(&percent_hex(byte));
      index += 1;
    }

    index += 1;
  }

  Ok(())
}

#[inline]
fn percent_hex(byte: u8) -> [u8; 3] {
  let [one, two] = byte_to_hex(byte);

  [b'%', one, two]
}
