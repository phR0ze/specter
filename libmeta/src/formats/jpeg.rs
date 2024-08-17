// JPEG's are constructed using `Markers`. Markers are a binary formatted value used to mark a segment
// of the file for a specific purpose e.g. start of the image data, end of the image data, app specific
// segments etc...

use nom::{bytes::streaming as nom_bytes, error::Error as NomError, number::streaming as nom_nums};
use std::io::{self, prelude::*};

use super::*;
use crate::{
    errors::{CastError, JpegParseError, MetaError},
    Kind, Meta,
};

// JPEG Markers
const JPEG_MARKER_PREFIX: [u8; 1] = [0xFF];
const JFIF_IDENTIFIER: [u8; 4] = [0x4A, 0x46, 0x49, 0x46];
const SOI: [u8; 2] = [0xFF, 0xD8];
const SOS: [u8; 2] = [0xFF, 0xDA];
const EOI: [u8; 2] = [0xFF, 0xD9];
const APP0: [u8; 2] = [0xFF, 0xE0];
const APP1: [u8; 2] = [0xFF, 0xE1];
const APP2: [u8; 2] = [0xFF, 0xE2];
const APP8: [u8; 2] = [0xFF, 0xE8];

enum Marker {
    Soi,  // 0xFFD8 - Start of any JPEG file
    Sos,  // 0xFFDA - Start of scan i.e. start of image data
    Eoi,  // 0xFFD9 - End of image data
    App0, // 0xFFE0 - JFIF marker segment
    App1, // 0xFFE1 - Exif marker segment
    App2, // 0xFFE2 - CIFF Canon Camera Image File Format
    App8, // 0xFFE8 - SPIFF Still Picture Interchange File Format
}

// JPEG segments are defined by an identifier, their length and the data they contain
#[derive(Debug, PartialEq)]
struct Segment {
    data: Vec<u8>,   // JPEG segment data
    marker: [u8; 2], // JPEG segment identifier
}

#[derive(Debug)]
pub struct Jpeg {
    pub jfif: Option<Jfif>,
    //pub reader: Box<dyn io::Read>,
}

impl Jpeg {
    pub fn new<T: io::Read + io::Seek>(mut reader: T) -> Result<Self, JpegParseError> {
        Ok(Self { jfif: None })
    }

    // Internal path used to bypass header validation of JPEG as it was already done to determine
    // the media file type used in creating this instance.
    pub(crate) fn factory<T: io::Read + io::Seek>(mut reader: T) -> Result<Self, JpegParseError> {
        Ok(Self { jfif: None })
    }
}

/// Parse out a segment. A segment has the following structure left to right:
/// * (1 byte)  Marker prefix e.g `0xFF`
/// * (1 byte)  Marker Number e.g. `0xE0`
/// * (2 bytes) Data size, including 2 size bytes, in Big Endian e.g. e.g 0x00 0x10 = 14 bytes
fn segment(input: &[u8]) -> Result<(&[u8], Segment), JpegParseError> {
    let (remain, marker) = marker(input).map_err(|x| JpegParseError::segment_invalid().wrap(x))?;

    // Match marker and parse the corresponding segment type
    match marker {
        //APP0 | APP1 => {
        APP0 => {
            let (remain, length) = length(remain)?;
            let (remain, data) = data(remain, length)?;
            Ok((remain, Segment { marker, data }))
        }
        _ => Err(JpegParseError::segment_marker_unknown(&marker)),
    }
}

/// Parse out a JPEG marker which is a 2 byte value consisting of:
/// * (1 byte) magic hex value `0xFF`
/// * (1 byte) number e.g. `0xE0`
fn marker(input: &[u8]) -> Result<(&[u8], [u8; 2]), JpegParseError> {
    match nom::sequence::preceded(
        nom_bytes::tag::<[u8; 1], &[u8], NomError<&[u8]>>(JPEG_MARKER_PREFIX),
        nom_nums::u8,
    )(input)
    {
        Ok((remain, num)) => Ok((remain, [JPEG_MARKER_PREFIX[0], num])),
        Err(e) => Err(JpegParseError::segment_marker_invalid().with_nom_source(e)),
    }
}

/// Parse out a JPEG segment length 2 byte in Big Endian format that includes the 2 size bytes.
/// Thus a length of `0x00 0x10` would be length 14 not 16.
fn length(input: &[u8]) -> Result<(&[u8], u16), JpegParseError> {
    let (remain, length) = nom_nums::be_u16(input)
        .map_err(|x| JpegParseError::segment_length_invalid().with_nom_source(x))?;
    Ok((remain, length - 2))
}

/// Parse out a JPEG segment data.
fn data(input: &[u8], length: u16) -> Result<(&[u8], Vec<u8>), JpegParseError> {
    let (remain, data) = nom::multi::count(nom_nums::u8, length as usize)(input)
        .map_err(|x| JpegParseError::segment_data_invalid().with_nom_source(x))?;

    // Convert the data to a vector
    let mut vec = Vec::with_capacity(length as usize);
    vec.extend_from_slice(&data);

    Ok((remain, vec))
}

/// Parse out all JPEG segments
fn parse(mut reader: impl io::Read) -> Result<Vec<Marker>, JpegParseError> {
    let results = vec![];

    // Potential constants
    let input_len: usize = 4096; // 4KB buffer, use something smaller for tests like 32 bytes

    // Loop over the file reading a chunk at a time and parsing the results.
    // * Break out into a multi-threaded approach later for performance?
    loop {
        let mut chunk: Vec<u8> = Vec::with_capacity(input_len);
        reader
            .by_ref() // Create a new reader that will read from the current position.
            .take(input_len as u64) // Create a new reader that only allows reading up to the input length.
            .read_to_end(&mut chunk) // Read until the new reader EOFs which is when the buffer is full.
            .map_err(|x| JpegParseError::segment_invalid().with_io_source(x))?;

        // Parse the chunk

        // Read another chunk
        chunk.clear();
    }

    Ok(results)
}

// Parse the given data into a JFIF structure
//| Field             | Byte# | Description
//| Identifier        | 5     | `0x4a 0x46 0x49 0x46 0x00` = `JFIF` in ASCII terminated by a null byte
//| JFIF version      | 2     | `0x01 0x02` is the major and minor JFIF version i.e. `1.02`
//| Density Units     | 1     | `0x00` = None, `0x01` = pixels per inch, `0x02` = pixels per centimeter
//| Xdensity          | 2     | `0x00 0x48` = `72` Horizontal pixel density, Must not be zero
//| Ydensity          | 2     | `0x00 0x48` = `72` Vertical pixel density, Must not be zero
//| Xthumbnail        | 1     | `0x00` Horizontal pixels of the embedded RGB thumbnail, May be zero
//| Ythumbnail        | 1     | `0x00` Vertical pixels of the embedded RGB thumbnail, May be zero
//| Thumbnail data    | 3 x n | Uncompressed 24 bit RGB (8 bits per color channel) raster thumbnail
fn jfif(input: &[u8]) -> Result<Jfif, JpegParseError> {
    let mut jfif = Jfif::new();

    // Parse the JFIF identifier and drop the results
    let (remain, _) = nom::sequence::terminated(
        nom_bytes::tag::<[u8; 4], &[u8], NomError<&[u8]>>(JFIF_IDENTIFIER),
        nom_bytes::tag::<[u8; 1], &[u8], NomError<&[u8]>>([0x00]),
    )(input)
    .map_err(|x| JpegParseError::jfif_identifier_invalid().with_nom_source(x))?;

    // Parse the JFIF version
    let (remain, (major, minor)) = nom::sequence::tuple((nom_nums::u8, nom_nums::u8))(remain)
        .map_err(|x| JpegParseError::jfif_version_invalid().with_nom_source(x))?;
    jfif.major = major;
    jfif.minor = minor;

    Ok(jfif)
}

#[cfg(test)]
mod tests {
    use super::*;

    const JFIF_DATA_1: [u8; 18] = [
        0xff, 0xe0, 0x00, 0x10, 0x4a, 0x46, 0x49, 0x46, 0x00, 0x01, 0x02, 0x01, 0x00, 0x48, 0x00,
        0x48, 0x00, 0x00,
    ];

    #[test]
    fn test_jfif_success() {
        let jfif = jfif(&JFIF_DATA_1[4..]).unwrap();
        assert_eq!(jfif.major, 1);
        assert_eq!(jfif.minor, 2);
    }

    #[test]
    fn test_segment_parser_success() {
        let (remain, segment) = segment(&JFIF_DATA_1).unwrap();
        assert_eq!(segment.marker, APP0);
        assert_eq!(segment.data, &JFIF_DATA_1[4..]);
        assert_eq!(remain, &[]);
        assert_eq!(
            std::str::from_utf8(&segment.data).unwrap(),
            "JFIF\0\u{1}\u{2}\u{1}\0H\0H\0\0"
        );
    }

    #[test]
    fn test_segment_marker_unknown() {
        let err = segment(&[0xff, 0xe9]).unwrap_err();
        assert_eq!(err.to_string(), "JPEG segment marker unknown [ff, e9]");
    }

    #[test]
    fn test_data_parser() {
        let (remain, data) = data(&JFIF_DATA_1[4..], 14).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(data, &JFIF_DATA_1[4..]);
    }

    #[test]
    fn test_length_success() {
        {
            let (remain, length) = length(&[0x00, 0x03, 0x02]).unwrap();
            assert_eq!(remain, &[0x02]);
            assert_eq!(length, 1);
        }
        {
            let (remain, length) = length(&JFIF_DATA_1[2..]).unwrap();
            assert_eq!(remain, &JFIF_DATA_1[4..]);
            assert_eq!(length, 14);
        }
    }

    #[test]
    fn test_marker_parser_success() {
        let (remain, marker) = marker(&JFIF_DATA_1).unwrap();
        assert_eq!(remain, &JFIF_DATA_1[2..]);
        assert_eq!(marker, [0xFF, 0xE0]);
    }

    #[test]
    fn test_marker_parser_fail() {
        let result = marker(&JFIF_DATA_1[2..]);
        assert!(result.is_err());
        let err = result.unwrap_err();

        assert_eq!(err.to_string(), "JPEG segment marker invalid");
        assert_eq!(
            err.as_ref().source().unwrap().to_string(),
            "nom::Parsing Error: Error { input: [0, 16, 74, 70, 73, 70, 0, 1, 2, 1, 0, 72, 0, 72, 0, 0], code: Tag }"
        );
    }

    #[test]
    fn test_jpeg_valid() {
        let mut header = io::Cursor::new(APP0);
        let jpeg = Jpeg::new(&mut header);
        assert!(jpeg.is_ok());
    }
}
