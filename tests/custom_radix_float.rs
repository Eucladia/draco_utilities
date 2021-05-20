mod float_radix {
  use draco_utilities::number::float_to_custom_radix;

  macro_rules! test {
    ($name:ident, $float:expr, $radix:literal, $expected:literal) => {
      #[test]
      fn $name() {
        let mut bytes = Vec::new();

        float_to_custom_radix::<$radix>($float, &mut bytes);

        let decoded = std::str::from_utf8(&bytes).unwrap();

        assert_eq!(decoded, $expected);
      }
    };
  }

  test!(whole_number_hex, 2021.0, 16, "7e5");
  test!(fractional_hex, 0.141592653589793, 16, "0.243f6a8885a2f8");
  test!(all_hex, 2102.3230, 16, "836.52b020c49b8");
  test!(integer_carry_over, 1.9999999999999999, 16, "2");

  // V8's tests
  //
  // https://github.com/v8/v8/blob/348cc6f152dbcb2d49a45aa3de32a87defcb127c/test/mjsunit/number-tostring.js#L71
  //
  // Copyright 2008 the V8 project authors. All rights reserved.
  // Redistribution and use in source and binary forms, with or without
  // modification, are permitted provided that the following conditions are
  // met:
  //
  //     * Redistributions of source code must retain the above copyright
  //       notice, this list of conditions and the following disclaimer.
  //     * Redistributions in binary form must reproduce the above
  //       copyright notice, this list of conditions and the following
  //       disclaimer in the documentation and/or other materials provided
  //       with the distribution.
  //     * Neither the name of Google Inc. nor the names of its
  //       contributors may be used to endorse or promote products derived
  //       from this software without specific prior written permission.
  //
  // THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
  // "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
  // LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
  // A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
  // OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
  // SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
  // LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
  // DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
  // THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
  // (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
  // OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

  test!(nan, f64::NAN, 16, "NaN");
  test!(infinity, f64::INFINITY, 16, "Infinity");
  test!(negative_infinity, -f64::INFINITY, 16, "-Infinity");
  test!(zero, 0.0, 16, "0");
  test!(nine, 9.0, 16, "9");
  test!(ninety, 90.0, 16, "5a");
  test!(ninety_decimal, 90.12, 16, "5a.1eb851eb852");
  test!(repeating, 0.1, 16, "0.1999999999999a");
  test!(point_zero_one, 0.01, 16, "0.028f5c28f5c28f6");
  test!(point_zero_one_two_three, 0.0123, 16, "0.032617c1bda511a");
  test!(ones, 111111111111111111111.0, 16, "605f9f6dd18bc8000");
  test!(
    even_more_ones,
    1111111111111111111111.0,
    16,
    "3c3bc3a4a2f75c0000"
  );
  test!(
    v8_devs_sure_love_ones,
    11111111111111111111111.0,
    16,
    "25a55a46e5da9a00000"
  );
  test!(smaller_point_one, 0.00001, 16, "0.0000a7c5ac471b4788");
  test!(even_smaller_point_one, 0.000001, 16, "0.000010c6f7a0b5ed8d");
  test!(
    really_small_point_one,
    0.0000001,
    16,
    "0.000001ad7f29abcaf48"
  );
  test!(
    the_smallest_zero_point_one,
    0.00000001,
    16,
    "0.0000002af31dc4611874"
  );
  test!(small_point_twelve, 0.00000012, 16, "0.000002036565348d256");
  test!(
    small_point_one_two_three,
    0.000000123,
    16,
    "0.0000021047ee22aa466"
  );
  test!(
    smaller_point_twelve,
    0.000000012,
    16,
    "0.000000338a23b87483be"
  );
  test!(
    smaller_point_zero_one_two_three,
    0.0000000123,
    16,
    "0.00000034d3fe36aaa0a2"
  );

  test!(negative_zero, -0.0, 16, "0");
  test!(negative_nine, -9.0, 16, "-9");
  test!(negative_ninety, -90.0, 16, "-5a");
  test!(negative_ninety_point_twelve, -90.12, 16, "-5a.1eb851eb852");
  test!(negative_point_one, -0.1, 16, "-0.1999999999999a");
  test!(negative_point_zero_one, -0.01, 16, "-0.028f5c28f5c28f6");
  test!(
    negative_point_zero_one_two_three,
    -0.0123,
    16,
    "-0.032617c1bda511a"
  );
  test!(
    negative_ones,
    -111111111111111111111.0,
    16,
    "-605f9f6dd18bc8000"
  );
  test!(
    more_negative_ones,
    -1111111111111111111111.0,
    16,
    "-3c3bc3a4a2f75c0000"
  );
  test!(
    even_more_negative_ones,
    -11111111111111111111111.0,
    16,
    "-25a55a46e5da9a00000"
  );
  test!(
    negative_small_point_one,
    -0.00001,
    16,
    "-0.0000a7c5ac471b4788"
  );
  test!(
    negative_smaller_point_one,
    -0.000001,
    16,
    "-0.000010c6f7a0b5ed8d"
  );
  test!(
    negative_even_smaller_point_one,
    -0.0000001,
    16,
    "-0.000001ad7f29abcaf48"
  );
  test!(
    negative_smallest_point_one,
    -0.00000001,
    16,
    "-0.0000002af31dc4611874"
  );
  test!(
    negative_small_point_twelve,
    -0.00000012,
    16,
    "-0.000002036565348d256"
  );
  test!(
    negative_small_point_one_two_three,
    -0.000000123,
    16,
    "-0.0000021047ee22aa466"
  );
  test!(
    negative_smallest_point_twelve,
    -0.000000012,
    16,
    "-0.000000338a23b87483be"
  );
  test!(
    negative_smallest_point_one_two_three,
    -0.0000000123,
    16,
    "-0.00000034d3fe36aaa0a2"
  );

  test!(
    one_div_four_repeating,
    1.0 / 4.0,
    3,
    "0.0202020202020202020202020202020202"
  );
  test!(
    two_div_seven_repeating,
    2.0 / 7.0,
    3,
    "0.0212010212010212010212010212010212"
  );
  test!(four_div_three, 4.0 / 3.0, 3, "1.1");
  test!(thirteen_div_three, 13.0 / 3.0, 3, "11.1");
  test!(one_div_nine, 1.0 / 9.0, 3, "0.01");
  test!(eighty_one, 81.0, 3, "10000");
  test!(
    eighty_one_plus_one_div_nine,
    81.0 + 1.0 / 9.0,
    3,
    "10000.01"
  );
}
