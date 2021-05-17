use crate::polyfills::globals::utils::{decode_two_octets, hex_value};

/// Unescapes a string.
///
/// # Notes
/// This function is the opposite of the [`escape`] function.
///
/// This function is functionally equivalent to JavaScript's
/// [unescape](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/unescape)
/// function.
///
/// [`escape`]: crate::polyfills::globals::escape
pub fn unescape(bytes: &[u8], unescaped: &mut Vec<u8>) {
  let mut idx = 0;

  while idx < bytes.len() {
    // SAFETY: This would always be a valid index because of the condition.
    let current = unsafe { *bytes.get_unchecked(idx) };

    if current == b'%' {
      // SAFETY: We check beforehand that we have enough bytes.
      unsafe {
        if idx + 5 < bytes.len() && *bytes.get_unchecked(idx + 1) == b'u' {
          let [one, two, three, four] = match bytes.get_unchecked(idx + 2..idx + 6) {
            [one, two, three, four, ..] => [*one, *two, *three, *four],
            // SAFETY: The first arm would've been reached.
            _ => std::hint::unreachable_unchecked(),
          };

          let first_val = hex_value(one, two);
          let second_val = hex_value(three, four);

          // Only append valid hexadecimal bytes.
          if first_val < 0x100 && second_val < 0x100 {
            let combined = ((first_val << 8) | second_val) as u16;

            unescaped.extend_from_slice(&decode_two_octets(combined));
            idx += 6;
            continue;
          }
        } else if idx + 2 < bytes.len() {
          let one = *bytes.get_unchecked(idx + 1);
          let two = *bytes.get_unchecked(idx + 2);
          let total = hex_value(one, two) as u16;

          // Only append valid hexadecimal bytes.
          if total < 0x100 {
            if total < 0x80 {
              // ASCII
              unescaped.push(total as u8);
            } else {
              // Two octets.
              unescaped.extend_from_slice(&decode_two_octets(total));
            }

            idx += 3;
            continue;
          }
        }
      }
    }

    unescaped.push(current);
    idx += 1;
  }
}
