/// Decoding URIs.
pub mod decode_uri;
/// Encoding URIs.
pub mod encode_uri;
/// Global utilities.
pub mod utils;

pub use decode_uri::*;
pub use encode_uri::*;

/// An error when encoding or decoding a URI.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum UriError {
  /// The provided URI was malformed.
  InvalidUri,
  /// An unexpected character was encountered.
  InvalidUtf8Character,
}
