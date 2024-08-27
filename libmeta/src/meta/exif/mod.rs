mod entry;
mod exif;
mod ifd;
mod tags;

// Surface module directly

// Surface types from modules directly in the meta module
pub(crate) use entry::*;
pub use exif::*;
pub(crate) use ifd::*;

pub(crate) const EXIF_IDENTIFIER: [u8; 4] = [0x45, 0x78, 0x69, 0x66];
pub(crate) const BIG_ENDIAN: [u8; 2] = [0x4D, 0x4D];
pub(crate) const LITTLE_ENDIAN: [u8; 2] = [0x49, 0x49];
