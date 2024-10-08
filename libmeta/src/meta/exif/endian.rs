use std::fmt::Display;

pub(crate) const BIG_ENDIAN: [u8; 2] = [0x4D, 0x4D];
pub(crate) const LITTLE_ENDIAN: [u8; 2] = [0x49, 0x49];

/// Track the endianness of the TIFF data
#[derive(Debug, Clone, PartialEq, Copy)]
pub(crate) enum Endian {
    Big,
    Little,
}

impl From<[u8; 2]> for Endian {
    fn from(data: [u8; 2]) -> Self {
        match data {
            BIG_ENDIAN => Endian::Big,
            LITTLE_ENDIAN => Endian::Little,
            _ => panic!("Invalid TIFF alignment"),
        }
    }
}

impl Display for Endian {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Endian::Big => write!(f, "Big Endian (Motorola, MM)"),
            Endian::Little => write!(f, "Little Endian (Intel, II)"),
        }
    }
}
