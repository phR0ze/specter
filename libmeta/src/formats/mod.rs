use std::fmt;

pub mod jpeg;

pub trait Meta: fmt::Debug {
    fn exif(&self);
}

// Well known media file hex signatures
pub(crate) const JPEG_PREFIX: [u8; 2] = [0xFF, 0xD8];
