// JPEG's are constructed using `Markers`. Markers are a binary formatted value used to mark a segment
// of the file for a specific purpose e.g. start of the image data, end of the image data, app specific
// segments etc...

use std::{any::Any, io};

use super::*;
use crate::{
    errors::{CastError, ParseError},
    Kind, Meta,
};

#[derive(Debug)]
pub struct Jpeg {
    pub jfif: Option<Jfif>,
}

impl Jpeg {
    pub fn new(mut reader: impl io::Read) -> Result<Self, ParseError> {
        Ok(Self { jfif: None })
    }
}

#[derive(Debug, PartialEq)]
pub struct Segment {
    marker: [u8; 2],
    length: u16,
    data: Vec<u8>,
}

/// Parse out a segment from the file. A segment consists of the following left to right:
/// * Marker prefix `0xFF` (1 byte)
/// * Marker Number e.g. `0xE0` (1 byte)
/// * Data size e.g 0x00 0x10 (2 bytes) in Big Endian 16 bit including the size bytes e.g. 14 bytes
fn get_segment(input: &[u8]) -> nom::IResult<&[u8], Segment> {
    // Parse out the segment marker
    let (remain, (marker, number)) = nom::sequence::tuple((
        nom::bytes::streaming::tag(JPEG_APP_MARKER),
        nom::number::streaming::u8,
    ))(input)?;
    let marker = [marker[0], number];
    match marker {
        APP0_MARKER | APP1_MARKER => {
            let (remain, length) = nom::number::streaming::be_u16(remain)?;
            let (remain, data) =
                nom::multi::count(nom::number::streaming::u8, length as usize)(remain)?;
            Ok((
                remain,
                Segment {
                    marker,
                    length,
                    data,
                },
            ))
        }
        _ => Ok((
            remain,
            Segment {
                marker,
                length: 0,
                data: vec![],
            },
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formats::*;

    #[test]
    fn test_jpeg_get_segment_marker() {
        let (remains, segment) = get_segment(&[0xFF, 0xE0, 0x00, 0x03, 0x01]).unwrap();
        assert_eq!(remains, &[]);
        assert_eq!(
            segment,
            Segment {
                marker: APP0_MARKER,
                length: 1,
                data: vec![0x01]
            }
        );
    }

    #[test]
    fn test_jpeg_valid() {
        let mut header = io::Cursor::new(APP0_MARKER);
        let jpeg = Jpeg::new(&mut header);
        assert!(jpeg.is_ok());
    }
}
