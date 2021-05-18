mod decode_uri;
mod encode_uri;
mod escape;
mod unescape;

/// Global utilities.
pub mod utils;

pub use decode_uri::*;
pub use encode_uri::*;
pub use escape::escape;
pub use unescape::unescape;

/// An error when encoding or decoding a URI.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum UriError {
  /// The provided URI was malformed.
  InvalidUri,
  /// An unexpected character was encountered.
  InvalidUtf8Character,
}
