use std::{
    fmt::Display,
    io::{self, prelude::*},
};

use super::{marker, segment::Segment};
use crate::{
    errors::JpegError,
    meta::{Exif, Jfif},
    slice,
};

/// Simplify the Exif return type slightly
pub type JpegResult<T> = Result<T, JpegError>;

#[derive(Debug)]
pub struct Jpeg {
    pub(crate) segments: Vec<Segment>,
}

impl Jpeg {
    /// Parse all meta data from the given JPEG source.
    pub fn parse<T: io::BufRead>(mut reader: T) -> JpegResult<Self> {
        // Check the header to determine the media type
        let mut header = Vec::new();
        reader
            .by_ref()
            .take(2)
            .read_to_end(&mut header)
            .map_err(|x| JpegError::read_failed(": invalid header").with_io_source(x))?;
        if !Self::is_jpeg(&header) {
            return Err(JpegError::parse(": invalid header"));
        }

        // Parse out the segments
        let segments = parse_segments(&mut reader)?;

        Ok(Jpeg { segments })
    }

    // /// Dump meta data segments from the given JPEG source for debugging purposes.
    // pub fn dump_segments(&self, no_data: bool) -> JpegResult<()> {
    //     for segment in self.segments.iter() {
    //         println!("{}", segment);
    //     }
    //     Ok(())
    // }

    // Determine if the given header is from a jpeg source
    pub(crate) fn is_jpeg(header: &[u8]) -> bool {
        header.starts_with(&marker::HEADER)
    }

    /// Get the JFIF meta data from the parsed JPEG.
    pub(crate) fn jfif(&self) -> Option<JpegResult<Jfif>> {
        match self.segments.iter().find(|x| x.marker == marker::APP0) {
            Some(segment) => match segment.data.as_ref() {
                Some(data) => Some(match Jfif::parse(data) {
                    Ok(jfif) => Ok(jfif),
                    Err(e) => Err(JpegError::parse(": jfif parsing").wrap(e)),
                }),
                _ => None,
            },
            _ => None,
        }
    }

    /// Get the Exif meta data from the parsed JPEG.
    pub(crate) fn exif(&self) -> Option<JpegResult<Exif>> {
        match self.segments.iter().find(|x| x.marker == marker::APP1) {
            Some(segment) => match segment.data.as_ref() {
                Some(data) => Some(match Exif::parse(data) {
                    Ok(exif) => Ok(exif),
                    Err(e) => Err(JpegError::parse(": exif parsing").wrap(e)),
                }),
                None => None,
            },
            None => None,
        }
    }
}

impl Display for Jpeg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for segment in self.segments.iter() {
            writeln!(f, "{}", segment)?;
        }
        Ok(())
    }
}

/// Parse out all the meta data related segments for the given JPEG source.
/// A segment has the following structure left to right:
/// * (1 byte)  Marker prefix e.g `0xFF`
/// * (1 byte)  Marker Number e.g. `0xE0`
/// * (2 bytes) Data size, including 2 size bytes, in Big Endian e.g. e.g 0x00 0x10 = 14 bytes
fn parse_segments(mut reader: impl io::BufRead) -> JpegResult<Vec<Segment>> {
    let mut segments = Vec::new();

    loop {
        // Defensively consume up to the marker incase the JPEG source is corrupted
        if !slice::skip_until(&mut reader, marker::PREFIX)
            .map_err(|e| JpegError::read_failed(": segment marker search").with_io_source(e))?
        {
            break;
        }

        // Read out the segment marker
        let marker = slice::read_u8(&mut reader)
            .map(|v| [0xFF, v])
            .map_err(|e| JpegError::read_failed(": segment marker").with_io_source(e))?;

        match marker {
            // Parse meta data related segments
            marker::APP0 | marker::APP1 => {
                // Parse out a JPEG segment length, 2 bytes in Big Endian format including
                // 2 size bytes. Thus a length of `0x00 0x10` would be length 14 not 16.
                let len = slice::read_be_u16(&mut reader)
                    .map_err(|e| JpegError::read_failed(": segment length").with_io_source(e))?;
                if len < 2 {
                    return Err(JpegError::parse(": segment length too short"));
                }
                let len = len - 2;

                // Parse out the segment data
                let data = slice::read_bytes(&mut reader, len as usize)
                    .map_err(|e| JpegError::read_failed(": segment data").with_io_source(e))?;

                segments.push(Segment::new(marker, len, Some(data)));
            }

            // Stop when we hit a non meta data marker
            _ => break,
        }
    }

    // Nothing was found
    if segments.is_empty() {
        return Err(JpegError::parse(": no segments found"));
    }

    Ok(segments)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::container::JPEG_TEST_DATA;
    use crate::errors::BaseError;
    use crate::meta::jfif::DensityUnit;

    #[test]
    fn test_parse() {
        let mut data = io::Cursor::new(JPEG_TEST_DATA);
        let jpeg = Jpeg::parse(&mut data).unwrap();

        // Validate JFIF
        let jfif = jpeg.jfif().unwrap().unwrap();
        assert_eq!(jfif.major, 1);
        assert_eq!(jfif.minor, 1);
        assert_eq!(jfif.density, DensityUnit::PixelsPerInch);
        assert_eq!(jfif.x_density, 72);
        assert_eq!(jfif.y_density, 72);
        assert_eq!(jfif.x_dimension, 0);
        assert_eq!(jfif.y_dimension, 0);

        // Exif
        // let exif = exif.unwrap();
        // assert_eq!(exif.is_big_endian(), true);
        // //let exif = exif.unwrap();

        //assert_eq!(err_to_string(&err), "");
    }

    #[test]
    fn test_parse_exif_success() {
        let segments = parse_segments(&JPEG_TEST_DATA[20..]).unwrap();
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].marker, marker::APP1);
        assert_eq!(segments[0].length, 860);
        assert_eq!(segments[0].data.as_ref().unwrap().len(), 860);
    }

    #[test]
    fn test_parse_data_not_enough_data() {
        let err = parse_segments(&mut &JPEG_TEST_DATA[2..19]).unwrap_err();
        assert_eq!(err.to_string(), JpegError::read_failed(": segment data").to_string());
        assert_eq!(err.source_to_string(), "io::Error: failed to fill whole buffer");
    }

    #[test]
    fn test_parse_length_ask_for_more_data() {
        let err = parse_segments(&mut &JPEG_TEST_DATA[2..4]).unwrap_err();
        assert_eq!(err.to_string(), JpegError::read_failed(": segment length").to_string());
        assert_eq!(err.source_to_string(), "io::Error: failed to fill whole buffer");
    }

    #[test]
    fn test_parse_jfif_segment_success() {
        let segments = parse_segments(&mut &JPEG_TEST_DATA[2..20]).unwrap();
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].marker, marker::APP0);
        assert_eq!(segments[0].length, 14);
        assert_eq!(segments[0].data, Some(JPEG_TEST_DATA[6..20].to_vec()));
    }

    #[test]
    fn test_parse_marker_not_enough_data() {
        let data = [];
        let err = parse_segments(&mut &data[..]).unwrap_err();
        assert_eq!(err.to_string(), JpegError::parse(": no segments found").to_string());

        let err = parse_segments(&mut &JPEG_TEST_DATA[2..3]).unwrap_err();
        assert_eq!(err.to_string(), JpegError::read_failed(": segment marker").to_string());
        assert_eq!(err.source_to_string(), "io::Error: failed to fill whole buffer");
    }

    #[test]
    fn test_parse_segments() {
        let segments = parse_segments(&mut &JPEG_TEST_DATA[2..]).unwrap();
        assert_eq!(segments.len(), 2);
    }

    #[test]
    fn test_parse_not_enough_data() {
        let data = marker::HEADER;
        let err = Jpeg::parse(&mut &data[..]).unwrap_err();
        assert_eq!(err.to_string(), JpegError::parse(": no segments found").to_string());
    }

    #[test]
    fn test_parse_header_invalid() {
        let data = [0xFF, 0x00];
        assert_eq!(
            Jpeg::parse(&mut &data[..]).unwrap_err().to_string(),
            JpegError::parse(": invalid header").to_string()
        );
    }

    #[test]
    fn test_is_jpeg() {
        assert_eq!(Jpeg::is_jpeg(&marker::HEADER), true);
        assert_eq!(Jpeg::is_jpeg(&[0xFF, 0xF0]), false);
    }
}
