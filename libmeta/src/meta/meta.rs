use std::{any::Any, fmt, io};

use super::Kind;
use crate::{
    errors::{CastError, ParseError},
    formats::{self, Jpeg},
};

#[derive(Debug)]
pub enum Meta {
    Jpeg(Jpeg),
}

impl Meta {
    /// Discover the media type and create a new instance based on that type
    pub fn new(mut reader: impl io::Read) -> Result<Self, ParseError> {
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;
        match buf {
            formats::JPEG_PREFIX => Ok(Self::Jpeg(Jpeg::new(reader)?)),
            _ => Err(ParseError::unknown_header(&buf)),
        }
    }

    /// Return the kind of media file were working with
    pub fn kind(&self) -> Kind {
        match self {
            Self::Jpeg(_) => Kind::Jpeg,
        }
    }

    /// Return the concrete jpeg type or an error if the cast fails
    pub fn as_jpeg(&self) -> Result<&Jpeg, CastError> {
        match self {
            Self::Jpeg(jpg) => Ok(jpg),
            _ => Err(CastError::new(format!(
                "Jpeg, real type is {}",
                self.kind()
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_meta_is_valid_jpeg() {
        let mut header = io::Cursor::new(&[0xFF, 0xD8]);
        let meta = Meta::new(&mut header);
        assert!(meta.is_ok());
        assert!(meta.unwrap().kind() == Kind::Jpeg);
    }

    #[test]
    fn test_new_meta_is_not_valid() {
        // unknown header type
        let mut header = io::Cursor::new(&[0xFF, 0x00]);
        assert_eq!(
            Meta::new(&mut header).unwrap_err().to_string(),
            "unknown header [ff, 0]"
        );

        // bad header length
        let mut header = io::Cursor::new(&[0xFF]);
        assert_eq!(
            Meta::new(&mut header).unwrap_err().to_string(),
            "read error: failed to fill whole buffer"
        );
    }
}
