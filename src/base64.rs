// Port of Daniel Lemire's base64 encoding and decoding - https://arxiv.org/abs/1704.00605
//
// Copyright (c) 2015-2016, Wojciech Muła, Alfred Klomp,  Daniel Lemire
// (Unless otherwise stated in the source code)
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
// 1. Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in the
//    documentation and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
// IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
// TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
// PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED
// TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

/// An error when decoding a base64 encoded string.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum DecodeError {
  /// Not enough bytes were provided or it wasn't a multiple of 4.
  InvalidLength,
  /// An invalid content was provided.
  InvalidContent,
}

const PADDING_CHAR: u8 = b'=';
const INVALID_CHAR: u32 = 0x01FFFFFF;

/// Encodes a base64 string.
pub fn encode_base64(bytes: &[u8], encoded: &mut Vec<u8>) {
  let length = bytes.len();
  let mut idx = 0;

  unsafe {
    if length > 2 {
      while idx < length - 2 {
        let [one, two, three] = match bytes.get_unchecked(idx..idx + 3) {
          [one, two, three, ..] => [*one, *two, *three],
          // SAFETY: We have enough bytes.
          _ => std::hint::unreachable_unchecked(),
        };

        encoded.extend_from_slice(&[
          E0[one as usize],
          E1[(((one & 0x03) << 4) | ((two >> 4) & 0x0F)) as usize],
          E1[(((two & 0x0F) << 2) | ((three >> 6) & 0x03)) as usize],
          E2[three as usize],
        ]);

        idx += 3;
      }
    }

    match length - idx {
      0 => {}
      1 => {
        let one = *bytes.get_unchecked(idx);

        encoded.extend_from_slice(&[
          E0[one as usize],
          E1[((one & 0x03) << 4) as usize],
          PADDING_CHAR,
          PADDING_CHAR,
        ]);
      }
      2 => {
        let one = *bytes.get_unchecked(idx);
        let two = *bytes.get_unchecked(idx + 1);

        encoded.extend_from_slice(&[
          E0[one as usize],
          E1[(((one & 0x03) << 4) | ((two >> 4) & 0x0F)) as usize],
          E2[((two & 0x0F) << 2) as usize],
          PADDING_CHAR,
        ]);
      }
      // SAFETY: Other arms would've been reached by now.
      _ => std::hint::unreachable_unchecked(),
    }
  }
}

/// Decodes a base64 encoded string.
pub fn decode_base64(bytes: &[u8], decoded: &mut Vec<u8>) -> Result<(), DecodeError> {
  let mut length = bytes.len();

  if length < 4 || length % 4 != 0 {
    return Err(DecodeError::InvalidLength);
  }

  if bytes[length - 1] == PADDING_CHAR {
    length -= 1;

    if bytes[length - 1] == PADDING_CHAR {
      length -= 1;
    }
  }

  let rem = length % 4;
  let mut idx = 0;

  unsafe {
    while idx < length & !3 {
      // SAFETY: We would have 4 bytes due to the condition
      let (total, pair) = decode_pair(bytes, idx);

      if total >= INVALID_CHAR {
        return Err(DecodeError::InvalidContent);
      }

      decoded.extend_from_slice(&pair);
      idx += 4
    }

    // SAFETY: There's enough remainder bytes to get without branching.
    let total = match rem {
      0 => return Ok(()),
      1 => {
        let total = D0[*bytes.get_unchecked(idx) as usize];
        decoded.push(total as u8);
        total
      }
      2 => {
        let total =
          D0[*bytes.get_unchecked(idx) as usize] | D1[*bytes.get_unchecked(idx + 1) as usize];
        decoded.push(total as u8);
        total
      }
      3 => {
        let total = D0[*bytes.get_unchecked(idx) as usize]
          | D1[*bytes.get_unchecked(idx + 1) as usize]
          | D2[*bytes.get_unchecked(idx + 2) as usize];

        decoded.extend_from_slice(&[
          (total & 0x000000FF) as u8,
          ((total & 0x0000FF00) >> 8) as u8,
        ]);
        total
      }
      // SAFETY: `rem % 4` would've reached the other arms.
      _ => std::hint::unreachable_unchecked(),
    };

    if total >= INVALID_CHAR {
      return Err(DecodeError::InvalidContent);
    }
  }

  Ok(())
}

unsafe fn decode_pair(bytes: &[u8], index: usize) -> (u32, [u8; 3]) {
  // SAFETY: The caller must uphold the condition of having enough bytes
  let [one, two, three, four] = match bytes.get_unchecked(index..index + 4) {
    [one, two, three, four, ..] => [*one, *two, *three, *four],
    _ => std::hint::unreachable_unchecked(),
  };

  let total = D0[one as usize] | D1[two as usize] | D2[three as usize] | D3[four as usize];

  (
    total,
    [
      (total & 0xFF) as u8,
      ((total & 0xFF00) >> 8) as u8,
      ((total & 0xFF0000) >> 16) as u8,
    ],
  )
}

const D0: [u32; 256] = [
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x000000f8, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x000000fc,
  0x000000d0, 0x000000d4, 0x000000d8, 0x000000dc, 0x000000e0, 0x000000e4, 0x000000e8, 0x000000ec,
  0x000000f0, 0x000000f4, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x00000000, 0x00000004, 0x00000008, 0x0000000c, 0x00000010, 0x00000014, 0x00000018,
  0x0000001c, 0x00000020, 0x00000024, 0x00000028, 0x0000002c, 0x00000030, 0x00000034, 0x00000038,
  0x0000003c, 0x00000040, 0x00000044, 0x00000048, 0x0000004c, 0x00000050, 0x00000054, 0x00000058,
  0x0000005c, 0x00000060, 0x00000064, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x00000068, 0x0000006c, 0x00000070, 0x00000074, 0x00000078, 0x0000007c, 0x00000080,
  0x00000084, 0x00000088, 0x0000008c, 0x00000090, 0x00000094, 0x00000098, 0x0000009c, 0x000000a0,
  0x000000a4, 0x000000a8, 0x000000ac, 0x000000b0, 0x000000b4, 0x000000b8, 0x000000bc, 0x000000c0,
  0x000000c4, 0x000000c8, 0x000000cc, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
];

const D1: [u32; 256] = [
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x0000e003, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x0000f003,
  0x00004003, 0x00005003, 0x00006003, 0x00007003, 0x00008003, 0x00009003, 0x0000a003, 0x0000b003,
  0x0000c003, 0x0000d003, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x00000000, 0x00001000, 0x00002000, 0x00003000, 0x00004000, 0x00005000, 0x00006000,
  0x00007000, 0x00008000, 0x00009000, 0x0000a000, 0x0000b000, 0x0000c000, 0x0000d000, 0x0000e000,
  0x0000f000, 0x00000001, 0x00001001, 0x00002001, 0x00003001, 0x00004001, 0x00005001, 0x00006001,
  0x00007001, 0x00008001, 0x00009001, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x0000a001, 0x0000b001, 0x0000c001, 0x0000d001, 0x0000e001, 0x0000f001, 0x00000002,
  0x00001002, 0x00002002, 0x00003002, 0x00004002, 0x00005002, 0x00006002, 0x00007002, 0x00008002,
  0x00009002, 0x0000a002, 0x0000b002, 0x0000c002, 0x0000d002, 0x0000e002, 0x0000f002, 0x00000003,
  0x00001003, 0x00002003, 0x00003003, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
];

const D2: [u32; 256] = [
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x00800f00, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x00c00f00,
  0x00000d00, 0x00400d00, 0x00800d00, 0x00c00d00, 0x00000e00, 0x00400e00, 0x00800e00, 0x00c00e00,
  0x00000f00, 0x00400f00, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x00000000, 0x00400000, 0x00800000, 0x00c00000, 0x00000100, 0x00400100, 0x00800100,
  0x00c00100, 0x00000200, 0x00400200, 0x00800200, 0x00c00200, 0x00000300, 0x00400300, 0x00800300,
  0x00c00300, 0x00000400, 0x00400400, 0x00800400, 0x00c00400, 0x00000500, 0x00400500, 0x00800500,
  0x00c00500, 0x00000600, 0x00400600, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x00800600, 0x00c00600, 0x00000700, 0x00400700, 0x00800700, 0x00c00700, 0x00000800,
  0x00400800, 0x00800800, 0x00c00800, 0x00000900, 0x00400900, 0x00800900, 0x00c00900, 0x00000a00,
  0x00400a00, 0x00800a00, 0x00c00a00, 0x00000b00, 0x00400b00, 0x00800b00, 0x00c00b00, 0x00000c00,
  0x00400c00, 0x00800c00, 0x00c00c00, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
];

const D3: [u32; 256] = [
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x003e0000, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x003f0000,
  0x00340000, 0x00350000, 0x00360000, 0x00370000, 0x00380000, 0x00390000, 0x003a0000, 0x003b0000,
  0x003c0000, 0x003d0000, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x00000000, 0x00010000, 0x00020000, 0x00030000, 0x00040000, 0x00050000, 0x00060000,
  0x00070000, 0x00080000, 0x00090000, 0x000a0000, 0x000b0000, 0x000c0000, 0x000d0000, 0x000e0000,
  0x000f0000, 0x00100000, 0x00110000, 0x00120000, 0x00130000, 0x00140000, 0x00150000, 0x00160000,
  0x00170000, 0x00180000, 0x00190000, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x001a0000, 0x001b0000, 0x001c0000, 0x001d0000, 0x001e0000, 0x001f0000, 0x00200000,
  0x00210000, 0x00220000, 0x00230000, 0x00240000, 0x00250000, 0x00260000, 0x00270000, 0x00280000,
  0x00290000, 0x002a0000, 0x002b0000, 0x002c0000, 0x002d0000, 0x002e0000, 0x002f0000, 0x00300000,
  0x00310000, 0x00320000, 0x00330000, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
  0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff, 0x01ffffff,
];

const E0: [u8; 256] = [
  b'A', b'A', b'A', b'A', b'B', b'B', b'B', b'B', b'C', b'C', b'C', b'C', b'D', b'D', b'D', b'D',
  b'E', b'E', b'E', b'E', b'F', b'F', b'F', b'F', b'G', b'G', b'G', b'G', b'H', b'H', b'H', b'H',
  b'I', b'I', b'I', b'I', b'J', b'J', b'J', b'J', b'K', b'K', b'K', b'K', b'L', b'L', b'L', b'L',
  b'M', b'M', b'M', b'M', b'N', b'N', b'N', b'N', b'O', b'O', b'O', b'O', b'P', b'P', b'P', b'P',
  b'Q', b'Q', b'Q', b'Q', b'R', b'R', b'R', b'R', b'S', b'S', b'S', b'S', b'T', b'T', b'T', b'T',
  b'U', b'U', b'U', b'U', b'V', b'V', b'V', b'V', b'W', b'W', b'W', b'W', b'X', b'X', b'X', b'X',
  b'Y', b'Y', b'Y', b'Y', b'Z', b'Z', b'Z', b'Z', b'a', b'a', b'a', b'a', b'b', b'b', b'b', b'b',
  b'c', b'c', b'c', b'c', b'd', b'd', b'd', b'd', b'e', b'e', b'e', b'e', b'f', b'f', b'f', b'f',
  b'g', b'g', b'g', b'g', b'h', b'h', b'h', b'h', b'i', b'i', b'i', b'i', b'j', b'j', b'j', b'j',
  b'k', b'k', b'k', b'k', b'l', b'l', b'l', b'l', b'm', b'm', b'm', b'm', b'n', b'n', b'n', b'n',
  b'o', b'o', b'o', b'o', b'p', b'p', b'p', b'p', b'q', b'q', b'q', b'q', b'r', b'r', b'r', b'r',
  b's', b's', b's', b's', b't', b't', b't', b't', b'u', b'u', b'u', b'u', b'v', b'v', b'v', b'v',
  b'w', b'w', b'w', b'w', b'x', b'x', b'x', b'x', b'y', b'y', b'y', b'y', b'z', b'z', b'z', b'z',
  b'0', b'0', b'0', b'0', b'1', b'1', b'1', b'1', b'2', b'2', b'2', b'2', b'3', b'3', b'3', b'3',
  b'4', b'4', b'4', b'4', b'5', b'5', b'5', b'5', b'6', b'6', b'6', b'6', b'7', b'7', b'7', b'7',
  b'8', b'8', b'8', b'8', b'9', b'9', b'9', b'9', b'+', b'+', b'+', b'+', b'/', b'/', b'/', b'/',
];

const E1: [u8; 256] = [
  b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
  b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
  b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
  b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
  b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
  b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
  b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
  b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
  b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
  b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
  b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
  b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
  b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
  b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
  b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
  b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
];

const E2: [u8; 256] = [
  b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
  b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
  b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
  b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
  b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
  b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
  b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
  b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
  b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
  b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
  b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
  b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
  b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
  b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
  b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
  b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
];
