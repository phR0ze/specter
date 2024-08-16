// JPEG's are constructed using `Markers`. Markers are a binary formatted value used to mark a segment
// of the file for a specific purpose e.g. start of the image data, end of the image data, app specific
// segments etc...

use nom::{bytes::streaming as nom_bytes, error::context, number::streaming as nom_nums};
use std::{any::Any, io};

use super::*;
use crate::{
    errors::{CastError, ParseError},
    Kind, Meta,
};

/// Nom result type for surfacing custom errors
//pub type NomResult<T, U> = nom::IResult<T, U, nom::error::ParseError<T>>;

// JPEG segments are defined by an identifier, their length and the data they contain
#[derive(Debug, PartialEq)]
struct Segment {
    id: [u8; 2],   // JPEG segment identifier
    data: Vec<u8>, // JPEG segment data
}

#[derive(Debug)]
pub struct Jpeg {
    pub jfif: Option<Jfif>,
}

impl Jpeg {
    pub fn new(mut reader: impl io::Read) -> Result<Self, ParseError> {
        Ok(Self { jfif: None })
    }
}

/// Parse out a segment from the file. A segment consists of the following left to right:
/// * (1 byte)  Marker prefix e.g `0xFF`
/// * (1 byte)  Marker Number e.g. `0xE0`
/// * (2 bytes) Data size, including 2 size bytes, in Big Endian e.g. e.g 0x00 0x10 = 14 bytes
fn segment(input: &[u8]) -> nom::IResult<&[u8], Segment> {
    // Parse out the segment marker
    let (remain, (marker, number)) =
        nom::sequence::tuple((nom_bytes::tag(JPEG_MARKER_PREFIX), nom_nums::u8))(input)?;
    let id = [marker[0], number];

    // Match marker and parse the corresponding segment type
    match id {
        APP0_MARKER | APP1_MARKER => {
            let (remain, length) = nom_nums::be_u16(remain)?;
            let (remain, data) = nom::multi::count(nom_nums::u8, length as usize)(remain)?;
            Ok((remain, Segment { id, data }))
        }
        _ => Ok((remain, Segment { id, data: vec![] })),
        //Err(JpegError::unknown_segment_identifier),
    }
}

/// Parse out a JPEG marker which is a 2 byte value consisting of:
/// * (1 byte) magic hex value `0xFF`
/// * (1 byte) number e.g. `0xE0`
fn marker(input: &[u8]) -> nom::IResult<&[u8], [u8; 2]> {
    nom::sequence::preceded(nom_bytes::tag(JPEG_MARKER_PREFIX), nom_nums::u8)(input)
        .map(|(remain, num)| (remain, [JPEG_MARKER_PREFIX[0], num]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formats::*;

    const jfif_data_1: [u8; 18] = [
        0xff, 0xe0, 0x00, 0x10, 0x4a, 0x46, 0x49, 0x46, 0x00, 0x01, 0x02, 0x01, 0x00, 0x48, 0x00,
        0x48, 0x00, 0x00,
    ];

    #[test]
    fn test_marker_parser_success() {
        let (remain, marker) = marker(&jfif_data_1).unwrap();
        assert_eq!(remain, &jfif_data_1[2..]);
        assert_eq!(marker, [0xFF, 0xE0]);
    }

    #[test]
    fn test_marker_parser_fail() {
        let result = marker(&jfif_data_1[2..]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "JPEG Marker");
    }

    #[test]
    fn test_jpeg_get_segment_marker() {
        let (remains, segment) = segment(&[0xFF, 0xE0, 0x00, 0x03, 0x01]).unwrap();
        // assert_eq!(remains, &[]);
        // assert_eq!(
        //     segment,
        //     Segment {
        //         id: APP0_MARKER,
        //         data: vec![0x01]
        //     }
        // );
    }

    #[test]
    fn test_jpeg_valid() {
        let mut header = io::Cursor::new(APP0_MARKER);
        let jpeg = Jpeg::new(&mut header);
        assert!(jpeg.is_ok());
    }
}
