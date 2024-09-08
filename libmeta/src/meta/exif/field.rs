use nom::bytes::streaming as nom_bytes;
use nom::number::streaming as nom_nums;

use crate::errors::ExifError;

use super::{
    format,
    tag::{self, *},
    Endian, ExifResult,
};

#[derive(Debug, PartialEq)]
pub enum Field {
    ImageWidth(u32),
    None,
}

/// Represents an IFD tag in cluding its identifier, format, number of components, and data.
#[derive(Debug, Clone)]
pub(crate) struct IfdField {
    // TODO: track display type?
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

    /// Parse IFD field which is 12 bytes of header an arbitrary data component
    /// e.g. TT TT ff ff NN NN NN NN DD DD DD DD
    /// * 2 byte Tag number
    /// * 2 byte Data format
    /// * 4 byte Number of components
    /// * 4 byte Offset to data value or data itself
    /// * **input** is the full data source from tiff header alignment
    /// * **remain** is where the header starts
    /// * Returns: (remaining bytes, IfdField)
    pub(crate) fn parse<'a>(
        input: &'a [u8],
        remain: &'a [u8],
        endian: Endian,
    ) -> ExifResult<(&'a [u8], IfdField)> {
        // Tag: 2 bytes
        let (remain, tag) = match endian {
            Endian::Big => nom_nums::be_u16(remain),
            Endian::Little => nom_nums::le_u16(remain),
        }
        .map_err(|x| ExifError::parse(": IFD field tag").with_nom_source(x))?;

        // Data format: 2 bytes
        let (remain, format) = match endian {
            Endian::Big => nom_nums::be_u16(remain),
            Endian::Little => nom_nums::le_u16(remain),
        }
        .map_err(|x| ExifError::parse(": IFD field data format").with_nom_source(x))?;

        // Number of components: 4 bytes
        let (remain, components) = match endian {
            Endian::Big => nom_nums::be_u32(remain),
            Endian::Little => nom_nums::le_u32(remain),
        }
        .map_err(|x| ExifError::parse(": IFD field components").with_nom_source(x))?;

        // Create the ifd field and calculate if there is an offset to extract data from
        let mut field = IfdField::new(endian, tag, format, components);
        let remain = if field.length() > 4 {
            let (remain, offset) = super::parse_ifd_offset(remain, endian)?;

            // Skip to the offset location
            let consumed = input.len() - remain.len();
            if consumed > offset as usize {
                return Err(ExifError::parse(": IFD field offset is negative"));
            }
            let inner = if offset as usize > consumed {
                let (inner, _) = nom_bytes::take(offset as usize - consumed)(remain)
                    .map_err(|x| ExifError::parse(": IFD field offset").with_nom_source(x))?;
                inner
            } else {
                remain
            };

            // Read the data from the offset location
            let (_, data) = nom_bytes::take(field.length())(inner)
                .map_err(|x| ExifError::parse(": IFD field data").with_nom_source(x))?;

            field.offset = Some(offset);
            field.data = Some(data.to_vec());
            remain

        // Raw data payload i.e. not an offset
        } else {
            let (remain, data) = super::parse_ifd_data(remain)?;
            field.data = Some(data.to_vec());
            remain
        };

        Ok((remain, field))
    }

    /// Add additional error data for output with the error message
    #[cfg(test)]
    fn with_data(mut self, data: &[u8]) -> Self {
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
    /// * Note: this only gets the first rational number
    pub(crate) fn to_rationals(&self) -> ExifResult<Vec<Rational>> {
        match self.data {
            Some(ref data) => {
                let mut rationals = Vec::new();
                for i in (0..data.len()).step_by(8) {
                    let rational = Rational::try_from(&data[i..i + 8], self.endian)?;
                    rationals.push(rational);
                }
                Ok(rationals)
            }
            None => Err(ExifError::parse(": no data to convert to rational")),
        }
    }

    /// Convert the data to an signed integer
    pub(crate) fn to_signed(&self) -> Option<isize> {
        match self.data {
            Some(ref data) => match self.format {
                format::SIGNED_BYTE => match data.len() {
                    1.. => Some(data[0] as isize),
                    _ => None,
                },
                format::SIGNED_SHORT => match data.len() {
                    2.. => {
                        if self.endian == Endian::Little {
                            Some(u16::from_le_bytes(data[0..2].try_into().unwrap()) as isize)
                        } else {
                            Some(u16::from_be_bytes(data[0..2].try_into().unwrap()) as isize)
                        }
                    }
                    _ => None,
                },
                format::SIGNED_LONG => match data.len() {
                    4.. => {
                        if self.endian == Endian::Little {
                            Some(u32::from_le_bytes(data[0..4].try_into().unwrap()) as isize)
                        } else {
                            Some(u32::from_be_bytes(data[0..4].try_into().unwrap()) as isize)
                        }
                    }
                    _ => None,
                },
                _ => None,
            },
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
    pub(crate) fn to_string(&self) -> String {
        // Try by tag type
        match match self.tag {
            Tag::Orientation=> self.to_unsigned().map(|x| Orientation::from(x).to_string()),
            Tag::Sharpness=> self.to_unsigned().map(|x| Sharpness::from(x).to_string()),
            Tag::Contrast=> self.to_unsigned().map(|x| Contrast::from(x).to_string()),
            Tag::Saturation=> self.to_unsigned().map(|x| Saturation::from(x).to_string()),
            Tag::SceneCaptureType=> self.to_unsigned().map(|x| Scene::from(x).to_string()),
            Tag::GainControl=> self.to_unsigned().map(|x| Gain::from(x).to_string()),

            // Lens specification consists of 4 rational numbers
            // tag::LENS_SPECIFICATION => self.to_rationals().ok().map(|x| {
            //     Gain::from(x).to_string()
            // }),
            Tag::ResolutionUnit=> self.to_unsigned()
                .map(|x| ResolutionUnit::from(x).to_string()),
            Tag::YCbCrPositioning=> self.to_unsigned()
                .map(|x| YCbCrPositioning::from(x).to_string()),

            // Try by format type
            _ => match self.format {
                format::ASCII_STRING => self.to_ascii(),
                format::UNSIGNED_BYTE => self.to_unsigned().map(|v| v.to_string()),
                format::UNSIGNED_SHORT => self.to_unsigned().map(|v| v.to_string()),
                format::UNSIGNED_LONG => self.to_unsigned().map(|v| v.to_string()),
                format::UNSIGNED_RATIONAL => self.to_rationals().ok().map(|v| {
                    v.iter().map(|r| r.to_string()).collect::<Vec<String>>().join(", ")
                }),
                format::SIGNED_BYTE => self.to_signed().map(|v| v.to_string()),
                format::SIGNED_SHORT => self.to_signed().map(|v| v.to_string()),
                format::SIGNED_LONG => self.to_signed().map(|v| v.to_string()),
                // format::SIGNED_RATIONAL => self.to_rational().map(|(n, d)| format!("{}/{}", n, d)),
                // format::SINGLE_FLOAT => self.to_unsigned().map(|v| v.to_string()),
                // format::DOUBLE_FLOAT => self.to_unsigned().map(|v| v.to_string()),
                format::UNDEFINED => self.to_ascii(),
                _ => None,
            },
        } {
            Some(x) => x,

            // Fallback to debug to be able to fix it easier
            None => format!(
                "Debug [format: {}, components: {}, length: {}]\n  {: <32}: Data: {:02x?}",
                self.format,
                self.components,
                self.length(),
                "".to_string(),
                self.data.as_ref().unwrap_or(&vec![0x00])
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exif::tag;

    const IFD_LE: [u8; 42] = [
        /* 00-01 */ 0x49, 0x49, // alignment, little endian
        /* 02-03 */ 0x2A, 0x00, // ifd marker
        /* 04-07 */ 0x08, 0x00, 0x00, 0x00, // ifd start
        /* 08-09 */ 0x02, 0x00, // field count
        /* 10-11 */ 0x1A, 0x01, // id: 0x011A, XResolution
        /* 12-13 */ 0x05, 0x00, // data format: 5, unsigned rational
        /* 14-17 */ 0x01, 0x00, 0x00, 0x00, // components: 1, so data 8 bytes
        /* 18-21 */ 0x22, 0x00, 0x00, 0x00, // offset of 34
        /* 22-23 */ 0x69, 0x87, // id:
        /* 24-25 */ 0x04, 0x00, // data format: 4, unsigned long
        /* 26-29 */ 0x01, 0x00, 0x00, 0x00, // components: 1
        /* 30-33 */ 0x2B, 0x00, 0x00, 0x00, // data for field 2
        /* 34-41 */ 0x48, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // data for field 1
    ];

    #[test]
    fn test_parse_ifd_field_header_big_endian() {
        let data = &[
            /* 00-01 */ 0x4D, 0x4D, // alignment, big endian
            /* 02-04 */ 0x00, 0x1A, // ifd marker
            /* 05-08 */ 0x00, 0x00, 0x00, 0x08, // ifd offset
            /* 09-10 */ 0x00, 0x01, // ifd field count
            0x01, 0x0e, // id
            0x00, 0x02, // data format
            0x00, 0x00, 0x00, 0x05, // number of components
            0x00, 0x00, 0x00, 0x16, // offset
            0x00, 0x00, 0x00, 0x00, 0x01, // data
        ];

        let (remain, ifd) = IfdField::parse(data, &data[10..], Endian::Big).unwrap();
        assert_eq!(remain, &data[22..]);
        assert_eq!(ifd.tag, Tag::from(270));
        assert_eq!(ifd.format, 2);
        assert_eq!(ifd.components, 5);
        assert_eq!(ifd.length(), 5);
        assert_eq!(ifd.offset, Some(22));
        assert_eq!(ifd.data, Some(data[22..].to_vec()));
    }

    #[test]
    fn test_parse_ifd_field_little_endian() {
        let (remain, ifd) = IfdField::parse(&IFD_LE, &IFD_LE[10..], Endian::Little).unwrap();
        assert_eq!(remain, &IFD_LE[22..]);
        assert_eq!(ifd.tag, Tag::from(282));
        assert_eq!(ifd.format, 5);
        assert_eq!(ifd.components, 1);
        assert_eq!(ifd.length(), 8);
        assert_eq!(ifd.offset, Some(34));
        assert_eq!(ifd.data, Some(IFD_LE[34..].to_vec()));
    }

    #[test]
    fn test_parse_ifd_offset_negative() {
        let data = &[
            0x01, 0x0e, // tag
            0x00, 0x02, // data format
            0x00, 0x00, 0x00, 0x05, // number of components
            0x00, 0x00, 0x00, 0x01, // invalid offset
            0x00, 0x00, 0x00, 0x00, 0x01, // data
        ];

        let err = IfdField::parse(data, data, Endian::Big).unwrap_err();
        assert_eq!(err.to_string(), "Exif parse failed: IFD field offset is negative");
    }

    #[test]
    fn test_data_to_unsigned() {
        assert_eq!(
            IfdField::new(Endian::Big, Tag::ResolutionUnit, format::UNSIGNED_SHORT, 1)
                .with_data(&[0x00, 0x02, 0x00, 0x00,])
                .to_unsigned(),
            Some(2)
        );
    }

    #[test]
    fn test_data_to_rational() {
        assert_eq!(
            IfdField::new(Endian::Big, Tag::XResolution, format::UNSIGNED_RATIONAL, 1)
                .with_data(&[0x00, 0x00, 0x00, 0x48, 0x00, 0x00, 0x00, 0x01,])
                .to_rationals()
                .unwrap(),
            vec![Rational::new(72, 1)]
        );
    }

    #[test]
    fn test_data_to_ascii() {
        assert_eq!(
            IfdField::new(Endian::Big, Tag::ImageDescription, format::ASCII_STRING, 11)
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
