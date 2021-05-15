use crate::polyfills::globals::utils::byte_to_hex;

/// Escapes a string.
///
/// # Notes
/// This function behaves the same as
/// [JavaScript's `escape`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/escape)
pub fn escape(bytes: &[u8], escaped: &mut Vec<u8>) {
  const RESERVED: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789@*_+-./";

  let mut idx = 0;

  while idx < bytes.len() {
    // SAFETY: `idx < bytes.len()` condition guarantees that this is valid.
    let current = unsafe { *bytes.get_unchecked(idx) };

    if RESERVED.contains(&current) {
      escaped.push(current);
      idx += 1;
      continue;
    }

    if current >= 0x80 && idx + 1 < bytes.len() {
      // SAFETY: There would be another byte due to the branch's condition.
      let next = unsafe { *bytes.get_unchecked(idx + 1) };

      // Continuation byte
      if next & 0xC0 == 0x80 {
        let val = ((current as u16 & 0x1F) << 6) | (next as u16 & 0x3F);

        if val >= 0x100 {
          let [one, two] = byte_to_hex((val >> 8) as u8);
          let [three, four] = byte_to_hex((val & 0xFF) as u8);

          escaped.extend_from_slice(&[b'%', b'u', one, two, three, four]);
        } else {
          // Get the LSBs only
          let [one, two] = byte_to_hex((val & 0xFF) as u8);

          escaped.extend_from_slice(&[b'%', one, two]);
        }

        idx += 2;
      } else {
        append_single_byte(current, escaped, &mut idx);
      }
    } else {
      append_single_byte(current, escaped, &mut idx)
    }
  }
}

#[inline]
fn append_single_byte(byte: u8, escaped: &mut Vec<u8>, counter: &mut usize) {
  let [one, two] = byte_to_hex(byte);

  escaped.extend_from_slice(&[b'%', one, two]);
  *counter += 1;
}
