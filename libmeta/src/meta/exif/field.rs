use super::{
    format,
    tag::{self, Tag},
    Endian, Orientation, ResolutionUnit, YCbCrPositioning,
};

/// Represents an IFD tag in cluding its identifier, format, number of components, and data.
#[derive(Debug, Clone)]
pub(crate) struct IfdField {
    pub(crate) endian: Endian,        // byte order
    pub(crate) tag: Tag,              // identifier
    pub(crate) format: u16,           // data format
    pub(crate) components: u32,       // number of components
    pub(crate) offset: Option<u32>,   // offset to data
    pub(crate) data: Option<Vec<u8>>, // actual data
}

impl IfdField {
    // Create a new IFD tag
    pub(crate) fn new<T: Into<Tag>>(endian: Endian, tag: T, format: u16, components: u32) -> Self {
        Self { endian, tag: tag.into(), format, components, offset: None, data: None }
    }

    /// Add additional error data for output with the error message
    pub(crate) fn with_data(mut self, data: &[u8]) -> Self {
        self.data = Some(data.into());
        self
    }

    // Calculate the length of the tag's data in number of bytes
    pub(crate) fn length(&self) -> u64 {
        match self.format {
            format::UNSIGNED_BYTE => self.components as u64,
            format::ASCII_STRING => self.components as u64,
            format::UNSIGNED_SHORT => self.components as u64 * 2,
            format::UNSIGNED_LONG => self.components as u64 * 4,
            format::UNSIGNED_RATIONAL => self.components as u64 * 8,
            format::SIGNED_BYTE => self.components as u64,
            format::UNDEFINED => self.components as u64,
            format::SIGNED_SHORT => self.components as u64 * 2,
            format::SIGNED_LONG => self.components as u64 * 4,
            format::SIGNED_RATIONAL => self.components as u64 * 8,
            format::SINGLE_FLOAT => self.components as u64 * 4,
            format::DOUBLE_FLOAT => self.components as u64 * 8,
            _ => 0,
        }
    }
    /// Convert the data to an ASCII string
    pub(crate) fn to_ascii(&self) -> Option<String> {
        match self.data {
            Some(ref data) => {
                let mut ascii = String::new();
                for &byte in data.iter() {
                    if byte == 0 {
                        break;
                    }
                    ascii.push(byte as char);
                }
                Some(ascii)
            }
            None => None,
        }
    }

    /// Convert the data to a rational number
    pub(crate) fn to_rational(&self) -> Option<(usize, usize)> {
        if self.length() != 8 {
            return None;
        }

        match self.data {
            Some(ref data) => data[0..4].try_into().ok().and_then(|num| {
                data[4..8].try_into().ok().map(|den| {
                    if self.endian == Endian::Little {
                        (u32::from_le_bytes(num) as usize, u32::from_le_bytes(den) as usize)
                    } else {
                        (u32::from_be_bytes(num) as usize, u32::from_be_bytes(den) as usize)
                    }
                })
            }),
            None => None,
        }
    }

    /// Convert the data to an unsigned integer
    pub(crate) fn to_unsigned(&self) -> Option<usize> {
        match self.data {
            Some(ref data) => match self.format {
                format::UNSIGNED_BYTE => match data.len() {
                    1.. => Some(data[0] as usize),
                    _ => None,
                },
                format::UNSIGNED_SHORT => match data.len() {
                    2.. => {
                        if self.endian == Endian::Little {
                            Some(u16::from_le_bytes(data[0..2].try_into().unwrap()) as usize)
                        } else {
                            Some(u16::from_be_bytes(data[0..2].try_into().unwrap()) as usize)
                        }
                    }
                    _ => None,
                },
                format::UNSIGNED_LONG => match data.len() {
                    4.. => {
                        if self.endian == Endian::Little {
                            Some(u32::from_le_bytes(data[0..4].try_into().unwrap()) as usize)
                        } else {
                            Some(u32::from_be_bytes(data[0..4].try_into().unwrap()) as usize)
                        }
                    }
                    _ => None,
                },
                _ => None,
            },
            None => None,
        }
    }

    /// Convert the data type into a human readable string
    pub(crate) fn data_to_string(&self) -> String {
        // Try by tag type
        match match self.tag {
            tag::RESOLUTION_UNIT => self
                .to_unsigned()
                .map(|x| ResolutionUnit::from(x).to_string()),

            // Try by format type
            _ => match self.format {
                // format::UNSIGNED_BYTE => self.to_unsigned().map(|v| v.to_string()),
                format::ASCII_STRING => self.to_ascii(),
                format::UNSIGNED_SHORT => match self.tag {
                    tag::ORIENTATION => {
                        self.to_unsigned().map(|x| Orientation::from(x).to_string())
                    }
                    tag::Y_CB_CR_POSITIONING => self
                        .to_unsigned()
                        .map(|x| YCbCrPositioning::from(x).to_string()),
                    _ => self.to_unsigned().map(|x| x.to_string()),
                },
                // format::UNSIGNED_LONG => self.to_unsigned().map(|v| v.to_string()),
                // format::UNSIGNED_RATIONAL => self.to_rational().map(|(n, d)| format!("{}/{}", n, d)),
                // format::SIGNED_BYTE => self.to_unsigned().map(|v| v.to_string()),
                // format::UNDEFINED => self.to_unsigned().map(|v| v.to_string()),
                // format::SIGNED_SHORT => self.to_unsigned().map(|v| v.to_string()),
                // format::SIGNED_LONG => self.to_unsigned().map(|v| v.to_string()),
                // format::SIGNED_RATIONAL => self.to_rational().map(|(n, d)| format!("{}/{}", n, d)),
                // format::SINGLE_FLOAT => self.to_unsigned().map(|v| v.to_string()),
                // format::DOUBLE_FLOAT => self.to_unsigned().map(|v| v.to_string()),
                _ => None,
            },
        } {
            Some(x) => x,
            None => "Unknown".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exif::tag;

    #[test]
    fn test_data_to_unsigned() {
        assert_eq!(
            IfdField::new(Endian::Big, tag::RESOLUTION_UNIT, format::UNSIGNED_SHORT, 1)
                .with_data(&[0x00, 0x02, 0x00, 0x00,])
                .to_unsigned(),
            Some(2)
        );
    }
    #[test]
    fn test_data_to_rational() {
        assert_eq!(
            IfdField::new(Endian::Big, tag::X_RESOLUTION, format::UNSIGNED_RATIONAL, 1)
                .with_data(&[0x00, 0x00, 0x00, 0x48, 0x00, 0x00, 0x00, 0x01,])
                .to_rational(),
            Some((72, 1))
        );
    }

    #[test]
    fn test_data_to_ascii() {
        assert_eq!(
            IfdField::new(Endian::Big, tag::IMAGE_DESCRIPTION, format::ASCII_STRING, 11)
                .with_data(&[
                    0x54, 0x65, 0x73, 0x74, 0x20, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x00, 0x46,
                ])
                .to_ascii(),
            Some("Test image".into())
        );
    }

    #[test]
    fn test_tag_data_length() {
        assert_eq!(IfdField::new(Endian::Big, 0, format::UNSIGNED_BYTE, 10).length(), 10);
        assert_eq!(IfdField::new(Endian::Big, 0, format::ASCII_STRING, 10).length(), 10);
        assert_eq!(IfdField::new(Endian::Big, 0, format::UNSIGNED_SHORT, 10).length(), 20);
        assert_eq!(IfdField::new(Endian::Big, 0, format::UNSIGNED_LONG, 10).length(), 40);
        assert_eq!(IfdField::new(Endian::Big, 0, format::UNSIGNED_RATIONAL, 10).length(), 80);
        assert_eq!(IfdField::new(Endian::Big, 0, format::SIGNED_BYTE, 10).length(), 10);
        assert_eq!(IfdField::new(Endian::Big, 0, format::UNDEFINED, 10).length(), 10);
        assert_eq!(IfdField::new(Endian::Big, 0, format::SIGNED_SHORT, 10).length(), 20);
        assert_eq!(IfdField::new(Endian::Big, 0, format::SIGNED_LONG, 10).length(), 40);
        assert_eq!(IfdField::new(Endian::Big, 0, format::SIGNED_RATIONAL, 10).length(), 80);
    }
}
