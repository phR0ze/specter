mod endian;
mod exif;
mod field;
mod format;
mod ifd;
mod tag;

// Surface module directly

// Surface types from modules directly in the meta module
pub(crate) use endian::*;
pub use exif::*;
pub(crate) use field::*;
pub(crate) use ifd::*;

pub(crate) const EXIF_IDENTIFIER: [u8; 4] = [0x45, 0x78, 0x69, 0x66];
