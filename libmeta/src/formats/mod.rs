// Surface types directly to avoid ugly studdering namespacing
mod jpeg;
pub(crate) use jpeg::Jpeg;

// Well known media file hex signatures
pub(crate) const JPEG_PREFIX: [u8; 2] = [0xFF, 0xD8];
