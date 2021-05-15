mod decode_uri;
mod encode_uri;
mod escape;

/// Global utilities.
pub mod utils;

pub use escape::escape;
pub use uri::*;

/// URI related functions
pub mod uri {
  pub use super::decode_uri::*;
  pub use super::encode_uri::*;

  /// An error when encoding or decoding a URI.
  #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
  pub enum UriError {
    /// The provided URI was malformed.
    InvalidUri,
    /// An unexpected character was encountered.
    InvalidUtf8Character,
  }
}
