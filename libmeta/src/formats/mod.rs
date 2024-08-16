// Surface types directly to avoid ugly studdering namespacing
mod jpeg;
pub(crate) use jpeg::Jpeg;
mod jfif;
pub(crate) use jfif::Jfif;

// Well known media file hex signatures
pub(crate) const JPEG_PREFIX: [u8; 2] = [0xFF, 0xD8];
