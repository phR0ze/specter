// Surface types directly to avoid ugly studdering namespacing
mod jpeg;
pub use jpeg::*;
mod jfif;
pub use jfif::*;

/// Well known JPEG header prefix
pub(crate) const JPEG_HEADER: [u8; 2] = [0xFF, 0xD8];
