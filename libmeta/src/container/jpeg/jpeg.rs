use std::io::{self, prelude::*};

use super::{marker, segment::Segment};
use crate::{
    errors::{JpegError, JpegErrorKind},
    meta::{Exif, Jfif},
};

/// Simplify the Exif return type slightly
pub type JpegResult<T> = Result<T, JpegError>;

#[derive(Debug)]
pub struct Jpeg {
    pub(crate) segments: Vec<Segment>,
}

impl Jpeg {
    /// Parse all meta data from the given JPEG source.
    pub fn parse(mut reader: impl io::Read) -> JpegResult<Self> {
        // Check the header to determine the media type
        let mut header = [0u8; 2];
        reader
            .by_ref()
            .read_exact(&mut header)
            .map_err(|x| JpegError::read_failed().with_io_source(x))?;
        if !Self::is_jpeg(&header) {
            return Err(JpegError::parse(": invalid header"));
        }

        // Parse out the segments
        let segments = parse_segments(&mut reader)?;

        Ok(Jpeg { segments })
    }

    /// Dump meta data segments from the given JPEG source for debugging purposes.
    pub fn dump_segments(&self) -> Result<(), JpegError> {
        for segment in self.segments.iter() {
            println!("{}", segment);
        }
        Ok(())
    }

    // Determine if the given header is from a jpeg source
    pub fn is_jpeg(header: &[u8]) -> bool {
        header.starts_with(&marker::HEADER)
    }

    /// Get the JFIF meta data from the parsed JPEG.
    pub(crate) fn jfif(&self) -> Option<JpegResult<Jfif>> {
        match self.segments.iter().find(|x| x.marker == marker::APP0) {
            Some(segment) => match segment.data.as_ref() {
                Some(data) => Some(match Jfif::parse(data) {
                    Ok(jfif) => Ok(jfif),
                    Err(e) => Err(e.into()),
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
                    Err(e) => Err(e.into()),
                }),
                None => None,
            },
            None => None,
        }
    }
}

/// Parse out all the segments for the given JPEG source.
fn parse_segments(mut reader: impl io::Read) -> Result<Vec<Segment>, JpegError> {
    let mut segments = Vec::new();

    let chunk_len = 10;

    // Loop over the source reading chunks of data and parse it into segments.
    // * Progressively load more data until all segments are parsed, but bail before
    //   reading the actual image data to avoid the unnecessary overhead.
    // * Break out into a multi-threaded approach later for performance, maybe?
    //   Highest performance option is to use a single thread to read data in chunks in
    //   a loop until all segments are parsed and image data is detected then abort.
    //   Meanwhile a worker thread is spawned to parse the segmments in parallel.
    let mut end_of_meta_data = false;
    let mut get_more_data = false;
    let mut buffer: Vec<u8> = Vec::with_capacity(4096); // rust std use 8k for most things
    let mut i = 0; // unacked start
    let mut j = 0; // unacked length
    loop {
        // Defensively discard unrecognized bytes up to next marker in an attempt to recover
        // from a corrupt JPEG source. Note: read_until will also discard the target value.
        // reader.read_until(JPEG_MARKER_PREFIX, &mut Vec::new());

        // Read the next chunk of data and store it in the buffer
        let mut buf: Vec<u8> = Vec::with_capacity(chunk_len);
        match reader.by_ref().take(chunk_len as u64).read_to_end(&mut buf) {
            Ok(0) => end_of_meta_data = true,
            Err(e) => return Err(JpegError::read_failed().with_io_source(e)),
            _ => (),
        }
        j += buf.len();
        buffer.extend_from_slice(&buf);
        get_more_data = false;

        // Loop parsing all segements switching on APP segment header
        loop {
            match Segment::parse(&buffer[i..i + j]) {
                Ok((remain, segment)) => match segment.marker {
                    marker::APP0 | marker::APP1 => {
                        segments.push(segment);
                        i += j - remain.len();
                        j = remain.len();
                    }
                    marker::DQT | marker::SOF | marker::DHT | marker::DRI => {
                        i += j - remain.len();
                        j = remain.len();
                    }
                    marker::SOS => {
                        // last segment before image data so break out
                        end_of_meta_data = true;
                        break;
                    }
                    _ => {
                        return Err(JpegError::parse(": segment marker unknown").with_data(remain));
                    }
                },
                Err(JpegError { kind: JpegErrorKind::Truncated, .. }) => {
                    get_more_data = true;
                    break;
                }
                Err(e) => {
                    return Err(JpegError::new().wrap(e));
                }
            }
        }

        // End of metadata i.e. no more data in file or hit SOS
        if end_of_meta_data {
            if get_more_data {
                // If more data is still needed it must be truncated JPEG source
                return Err(JpegError::truncated());
            }
            break;
        }
    }

    Ok(segments)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::container::JPEG_TEST_DATA;
    use crate::meta::jfif;

    #[test]
    fn test_parse() {
        let mut data = io::Cursor::new(JPEG_TEST_DATA);
        let jpeg = Jpeg::parse(&mut data).unwrap();

        // Validate JFIF
        let jfif = jpeg.jfif().unwrap().unwrap();
        assert_eq!(jfif.major, 1);
        assert_eq!(jfif.minor, 1);
        assert_eq!(jfif.density, jfif::DensityUnit::PixelsPerInch);
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
    fn test_parse_segments() {
        let mut data = io::Cursor::new(&JPEG_TEST_DATA[2..]);
        let segments = parse_segments(&mut data).unwrap();
        assert_eq!(segments.len(), 2);
    }

    #[test]
    fn test_parse_not_enough_data() {
        let mut data = io::Cursor::new(marker::HEADER);
        let err = Jpeg::parse(&mut data).unwrap_err();
        assert_eq!(err.to_string(), JpegError::truncated().to_string());
    }

    #[test]
    fn test_parse_header_invalid() {
        let mut header = io::Cursor::new([0xFF, 0x00]);
        assert_eq!(
            Jpeg::parse(&mut header).unwrap_err().to_string(),
            JpegError::parse(": invalid header").to_string()
        );
    }

    #[test]
    fn test_is_jpeg() {
        assert_eq!(Jpeg::is_jpeg(&marker::HEADER), true);
        assert_eq!(Jpeg::is_jpeg(&[0xFF, 0xF0]), false);
    }
}
