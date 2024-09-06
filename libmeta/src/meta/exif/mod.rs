mod endian;
mod field;
mod format;
mod ifd;

// Surface module directly
pub(crate) mod exif;
pub(crate) mod tag;
pub(crate) mod test_data;

// Surface types from modules directly in the meta module
pub(crate) use endian::*;
pub(crate) use exif::*;
pub(crate) use field::*;
pub(crate) use ifd::*;

const EXIF_IDENTIFIER: [u8; 4] = [0x45, 0x78, 0x69, 0x66];
const TIFF_VERSION: [u8; 2] = [0x00, 0x2A];
