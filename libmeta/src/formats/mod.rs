// Surface types directly to avoid ugly studdering namespacing
mod jpeg;
pub use jpeg::*;
mod jfif;
pub use jfif::*;

// Well known media file hex signatures
pub(crate) const JPEG_APP_MARKER: [u8; 1] = [0xFF];
pub(crate) const JPEG_PREFIX: [u8; 2] = [0xFF, 0xD8];
pub(crate) const APP0_MARKER: [u8; 2] = [0xFF, 0xE0];
pub(crate) const APP1_MARKER: [u8; 2] = [0xFF, 0xE1];
