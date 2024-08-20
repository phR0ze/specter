use std::io::{self, Read};

use super::{Exif, Jfif, Kind};
use crate::{
    errors::{CastError, MetaError},
    parsers::jpeg,
};

/// Meta provides encapsulation for the different metadata types that can be parsed
/// out of a variety of media types.
#[derive(Debug)]
#[non_exhaustive]
pub struct Meta {
    exif: Option<Exif>,
    // Itpc
    jfif: Option<Jfif>,
    // Xmp
    kind: Kind,
}

impl Meta {
    /// Discover the media type and create a new instance based on that type
    pub fn parse<T: io::BufRead + io::Seek>(reader: &mut T) -> Result<Self, MetaError> {
        let mut header = Vec::new();
        reader.by_ref().take(2).read_to_end(&mut header)?;

        // Check the header to determine the media type
        if jpeg::is_jpeg(&header) {
            let (jfif, exif) = jpeg::parse(header.chain(reader))?;
            Ok(Self {
                jfif,
                exif,
                kind: Kind::Jpeg,
            })
        } else {
            Err(MetaError::unknown_header(&header))
        }
    }

    /// Return the kind of media file were working with
    pub fn kind(&self) -> Kind {
        self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meta_parse_header_is_valid_jpeg() {
        let mut header = io::Cursor::new([jpeg::marker::HEADER, jpeg::marker::SOS].concat());
        let meta = Meta::parse(&mut header).unwrap();
        assert_eq!(meta.kind(), Kind::Jpeg);
    }

    #[test]
    fn test_meta_parse_not_enough_data() {
        let mut header = io::Cursor::new(&[0xFF]);
        let err = Meta::parse(&mut header).unwrap_err();
        assert_eq!(err.to_string(), "metadata unknown header [ff]");
    }

    #[test]
    fn test_meta_parse_header_is_invalid() {
        // unknown header type
        let mut header = io::Cursor::new(&[0xFF, 0x00]);
        assert_eq!(
            Meta::parse(&mut header).unwrap_err().to_string(),
            "metadata unknown header [ff, 00]"
        );
    }
}
