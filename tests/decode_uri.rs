use draco_utilities::polyfills::globals::decode_uri;

#[test]
fn decode_uri_string() {
  let expected = "https://developer.mozilla.org/ru/docs/JavaScript_шеллы";
  let mut decoded = Vec::new();

  decode_uri(
    "https://developer.mozilla.org/ru/docs/JavaScript_%D1%88%D0%B5%D0%BB%D0%BB%D1%8B".as_bytes(),
    &mut decoded,
  )
  .unwrap();
  let result = std::str::from_utf8(&decoded).unwrap();

  assert_eq!(result, expected);
}

#[test]
fn decode_invalid_uri_string() {
  let mut decoded = Vec::new();

  assert!(decode_uri("%E0%A4%A".as_bytes(), &mut decoded).is_err());
}
