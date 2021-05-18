use draco_utilities::globals::escape;

#[test]
fn escape_reserved() {
  let expected = "abc123@*_+-./";
  let mut bytes = Vec::new();

  escape(expected.as_bytes(), &mut bytes);
  let result = std::str::from_utf8(&bytes).unwrap();
  assert_eq!(result, expected);
}

#[test]
fn escape_combined() {
  let expected = "abc123%u0107./-2-_2-32%25348%E4%F6%FC";
  let mut bytes = Vec::new();

  escape("abc123\u{0107}./-2-_2-32%348äöü".as_bytes(), &mut bytes);
  let result = std::str::from_utf8(&bytes).unwrap();
  assert_eq!(result, expected);
}

#[test]
fn escape_bytes() {
  let expected = "%E4%F6%FC";
  let mut bytes = Vec::new();

  escape(b"\xE4\xF6\xFC", &mut bytes);

  let result = std::str::from_utf8(&bytes).unwrap();
  assert_eq!(result, expected);
}

#[test]
fn escape_utf16() {
  let expected = "%u0107";
  let mut bytes = Vec::new();

  escape("\u{0107}".as_bytes(), &mut bytes);
  let result = std::str::from_utf8(&bytes).unwrap();
  assert_eq!(result, expected);
}
