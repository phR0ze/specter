pub mod jpeg;

// Well known media file hex signatures
pub(crate) const JPEG_PREFIX: [u8; 2] = [0xFF, 0xD8];
