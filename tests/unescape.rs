use draco_utilities::polyfills::globals::unescape;

#[test]
fn unescape_combined() {
  let expected = "abc123\u{0107}./-2-_2-32%348äöü";
  let mut bytes = Vec::new();

  unescape(
    "abc123%u0107./-2-_2-32%25348%E4%F6%FC".as_bytes(),
    &mut bytes,
  );
  let result = std::str::from_utf8(&bytes).unwrap();
  assert_eq!(result, expected);
}

#[test]
fn unescape_reserved() {
  let expected = "abc123@*_+-./";
  let mut bytes = Vec::new();

  unescape(expected.as_bytes(), &mut bytes);
  let result = std::str::from_utf8(&bytes).unwrap();
  assert_eq!(result, expected);
}

#[test]
fn unescape_bytes() {
  let expected = "äöü";
  let mut bytes = Vec::new();

  unescape(b"%E4%F6%FC", &mut bytes);
  let result = std::str::from_utf8(&bytes).unwrap();
  assert_eq!(result, expected);
}

#[test]
fn unescape_utf16() {
  let expected = "\u{0107}";
  let mut bytes = Vec::new();

  unescape("%u0107".as_bytes(), &mut bytes);
  let result = std::str::from_utf8(&bytes).unwrap();
  assert_eq!(result, expected);
}
