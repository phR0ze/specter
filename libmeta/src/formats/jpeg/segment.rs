use nom::{bytes::streaming as nom_bytes, error::Error as NomError, number::streaming as nom_nums};
use std::fmt::Display;

use super::marker;
use crate::errors::JpegError;

// JPEG segments are defined by an identifier, their length and the data they contain
#[derive(Debug, PartialEq)]
pub(crate) struct Segment {
    pub(crate) marker: [u8; 2],       // JPEG segment identifier
    pub(crate) length: u16,           // JPEG segment length
    pub(crate) data: Option<Vec<u8>>, // JPEG segment data
}
impl Segment {
    pub(crate) fn new(marker: [u8; 2], length: u16, data: Option<Vec<u8>>) -> Self {
        Self {
            marker,
            length,
            data,
        }
    }

    pub(crate) fn data_to_ascii(&self) -> Result<String, JpegError> {
        match self.data {
            Some(ref data) => {
                let mut ascii = String::new();
                for byte in data {
                    if *byte >= 32 && *byte <= 126 {
                        ascii.push(*byte as char);
                    } else {
                        ascii.push('.');
                    }
                }
                Ok(ascii)
            }
            None => Ok(String::new()),
        }
    }
}

impl Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Print out a Rust array of the data
        writeln!(
            f,
            "const {}: [u8; {}] = [",
            marker::to_string(&self.marker)
                .split_whitespace() // Split the marker into words
                .next() // Get the first word
                .unwrap()
                .to_lowercase(), // Convert to lowercase
            self.length
        )?;
        for line in self.data.as_ref().unwrap().chunks(10) {
            write!(f, "    ")?;
            for byte in line {
                write!(f, "{:#04x},", byte)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "];")
    }
}

/// Parse out a segment. A segment has the following structure left to right:
/// * (1 byte)  Marker prefix e.g `0xFF`
/// * (1 byte)  Marker Number e.g. `0xE0`
/// * (2 bytes) Data size, including 2 size bytes, in Big Endian e.g. e.g 0x00 0x10 = 14 bytes
pub(crate) fn parse(input: &[u8]) -> Result<(&[u8], Segment), JpegError> {
    let (remain, marker) = parse_marker(input)?;

    // Match marker and parse the corresponding segment type
    match marker {
        // Parse segments with data
        marker::APP0 | marker::APP1 | marker::DQT | marker::SOF | marker::DHT | marker::DRI => {
            let (remain, length) = parse_length(remain)?;
            let (remain, data) = parse_data(remain, length)?;
            Ok((remain, Segment::new(marker, length, Some(data))))
        }

        // Parse segments with no data.
        // SOS actually has data but we don't care about the image data for metadata parsing
        marker::SOS => Ok((remain, Segment::new(marker, 0, None))),

        // Unknown segment
        _ => Err(JpegError::parse(": segment marker unknown").with_data(&marker)),
    }
}

// Parse out a JPEG segment marker which is a 2 byte value consisting of:
// * (1 byte) magic hex value `0xFF`
// * (1 byte) number e.g. `0xE0`
pub(crate) fn parse_marker(input: &[u8]) -> Result<(&[u8], [u8; 2]), JpegError> {
    nom::sequence::preceded(
        nom_bytes::tag::<[u8; 1], &[u8], NomError<&[u8]>>([marker::PREFIX]),
        nom_nums::u8,
    )(input)
    .map(|(remain, num)| (remain, [marker::PREFIX, num]))
    .map_err(|e| JpegError::parse(": segment marker").with_nom_source(e))
}

// Parse out a JPEG segment length 2 byte in Big Endian format that includes the 2 size bytes.
// Thus a length of `0x00 0x10` would be length 14 not 16.
pub(crate) fn parse_length(input: &[u8]) -> Result<(&[u8], u16), JpegError> {
    nom_nums::be_u16(input)
        .map(|(remain, val)| (remain, val - 2))
        .map_err(|x| JpegError::parse(": segment length").with_nom_source(x))
}

// Parse out the segment data
pub(crate) fn parse_data(input: &[u8], length: u16) -> Result<(&[u8], Vec<u8>), JpegError> {
    nom::multi::count(nom_nums::u8, length as usize)(input)
        .map_err(|x| JpegError::parse(": segment data").with_nom_source(x))
}

#[cfg(test)]
mod tests {
    use super::{super::JPEG_TEST_DATA, *};
    use crate::errors::BaseError;

    #[test]
    fn test_parse_marker_unknown() {
        let err = parse(&[0xff, 0xe9]).unwrap_err();
        assert_eq!(
            err.to_string(),
            JpegError::parse(": segment marker unknown")
                .with_data(&[0xff, 0xe9])
                .to_string()
        );
    }

    #[test]
    fn test_parse_ask_for_more_data() {
        let err = parse(&[]).unwrap_err();
        assert_eq!(
            err.to_string(),
            JpegError::truncated()
                .with_msg(": segment marker")
                .to_string()
        );
        assert_eq!(
            err.source_to_string(),
            "nom::Parsing requires 1 bytes/chars"
        );
    }

    #[test]
    fn test_parse_exif_success() {
        let (_, segment) = parse(&JPEG_TEST_DATA[20..]).unwrap();
        assert_eq!(segment.marker, marker::APP1);
        assert_eq!(segment.length, 860);
        assert_eq!(segment.data.unwrap().len(), 860);
    }

    #[test]
    fn test_parse_segment_jfif_success() {
        let (remain, segment) = parse(&JPEG_TEST_DATA[2..20]).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(segment.marker, marker::APP0);
        assert_eq!(segment.data.unwrap(), &JPEG_TEST_DATA[6..20]);
    }

    #[test]
    fn test_parse_data_not_enough_data() {
        let err = parse_data(&JPEG_TEST_DATA[6..19], 14).unwrap_err();
        assert_eq!(
            err.to_string(),
            JpegError::truncated()
                .with_msg(": segment data")
                .to_string()
        );
        assert_eq!(
            err.source_to_string(),
            "nom::Parsing requires 1 bytes/chars"
        );
    }

    #[test]
    fn test_parse_data_valid() {
        let (remain, data) = parse_data(&JPEG_TEST_DATA[6..20], 14).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(data, &JPEG_TEST_DATA[6..20]);
    }

    #[test]
    fn test_parse_length_ask_for_more_data() {
        let err = parse_length(&[]).unwrap_err();
        assert_eq!(
            err.to_string(),
            JpegError::truncated()
                .with_msg(": segment length")
                .to_string()
        );
        assert_eq!(
            err.source_to_string(),
            "nom::Parsing requires 2 bytes/chars"
        );
    }

    #[test]
    fn test_parse_length_success() {
        let (remain, length) = parse_length(&JPEG_TEST_DATA[4..6]).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(length, 14);
    }

    #[test]
    fn test_parse_marker_success() {
        let (remain, marker) = parse_marker(&JPEG_TEST_DATA[2..4]).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(marker, marker::APP0);
    }

    #[test]
    fn test_parse_marker_not_enough_data() {
        let err = parse_marker(&[0xFF]).unwrap_err();
        assert_eq!(
            err.to_string(),
            JpegError::truncated()
                .with_msg(": segment marker")
                .to_string()
        );
        assert_eq!(
            err.source_to_string(),
            "nom::Parsing requires 1 bytes/chars"
        );
    }
}
