use draco_utilities::polyfills::number::utils;

mod byte_conversion {
  use super::*;

  #[test]
  fn alphanumeric_byte() {
    let result = utils::to_character_byte(0);
    assert_eq!(result, b'0');
  }

  #[test]
  fn alphanumeric_byte_non_continuous() {
    let result = (utils::to_character_byte(10), utils::to_character_byte(35));
    assert_eq!(result, (b'a', b'z'));
  }

  #[test]
  fn from_char_byte() {
    let result = utils::from_character_byte(b'0');
    assert_eq!(result, 0);
  }

  #[test]
  fn from_char_byte_non_continuous() {
    let result = (
      utils::from_character_byte(b'a'),
      utils::from_character_byte(b'Z'),
    );

    assert_eq!(result, (10, 35));
  }
}

mod float_utils {
  use super::*;

  #[test]
  fn float_to_bits() {
    let float = 3.14;
    let parts = utils::floating_point_parts(float);

    let expected_sign = 0b0000000000000000000000000000000000000000000000000000000000000000;
    let expected_exponent = 0b0000000000000000000000000000000000000000000000000000000000000001;
    let expected_mantissa = 0b0000000000011001000111101011100001010001111010111000010100011111;

    assert_eq!(parts, (expected_sign, expected_exponent, expected_mantissa));
  }

  #[test]
  fn bits_to_float() {
    let expected = 3.14;

    let sign = 0b0000000000000000000000000000000000000000000000000000000000000000;
    let exponent = 0b0000000000000000000000000000000000000000000000000000000000000001;
    let mantissa = 0b0000000000011001000111101011100001010001111010111000010100011111;

    let float = utils::from_floating_point_parts(sign, exponent, mantissa);

    assert_eq!(float, expected);
  }

  #[test]
  fn next_fp_inf() {
    let result = utils::next_floating_point(f64::INFINITY);
    assert!(result.is_nan());
  }

  #[test]
  fn next_fp_neg_and_subnormal() {
    let result = utils::next_floating_point(-0.0);
    assert_eq!(result, 0.0);
  }

  #[test]
  fn next_fp_pos() {
    let result = utils::next_floating_point(0.0);
    assert_eq!(result, 5e-324);
  }

  #[test]
  fn next_fp_neg() {
    let result = utils::next_floating_point(-0.01);
    assert_eq!(result, -0.009999999999999998);
  }
}
