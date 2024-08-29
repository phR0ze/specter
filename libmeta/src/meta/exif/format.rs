// IFD field data format
pub(crate) const UNSIGNED_BYTE: u16 = 0x01; // 1 byte per component
pub(crate) const ASCII_STRING: u16 = 0x02; // 1 byte per component
pub(crate) const UNSIGNED_SHORT: u16 = 0x03; // 2 bytes per component
pub(crate) const UNSIGNED_LONG: u16 = 0x04; // 4 bytes per component
pub(crate) const UNSIGNED_RATIONAL: u16 = 0x05; // 8 bytes per component
pub(crate) const SIGNED_BYTE: u16 = 0x06; // 1 byte per component
pub(crate) const UNDEFINED: u16 = 0x07; // 1 byte per component
pub(crate) const SIGNED_SHORT: u16 = 0x08; // 2 bytes per component
pub(crate) const SIGNED_LONG: u16 = 0x09; // 4 bytes per component
pub(crate) const SIGNED_RATIONAL: u16 = 0x0A; // 8 bytes per component
pub(crate) const SINGLE_FLOAT: u16 = 0x0B; // 4 bytes per component
pub(crate) const DOUBLE_FLOAT: u16 = 0x0C; // 8 bytes per component
