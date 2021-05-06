use draco_utilities::polyfills::globals::encode_uri;

#[test]
fn encode_uri_string() {
  let expected = "https://developer.mozilla.org/ru/docs/JavaScript_%D1%88%D0%B5%D0%BB%D0%BB%D1%8B";
  let mut encoded = Vec::new();

  encode_uri(
    "https://developer.mozilla.org/ru/docs/JavaScript_шеллы".as_bytes(),
    &mut encoded,
  )
  .unwrap();

  let result = &String::from_utf8(encoded).unwrap();

  assert_eq!(result, expected);
}
