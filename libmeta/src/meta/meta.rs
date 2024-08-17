use std::{any::Any, fmt, io};

use super::Kind;
use crate::{
    errors::{CastError, MetaError},
    formats::{self, Jpeg},
};

#[derive(Debug)]
#[non_exhaustive]
pub enum Meta {
    Jpeg(Jpeg),
}

// Notes
// BufReader is used to read the file in chunks to reduce the number of system calls
// and improve performance. The default buffer size is 8KB. seeking with BufReader will
// discard the cache which is inefficient if your looking to reuse the data.

// Vec::with_capacity() will preallocate the memory for the vector to avoid reallocation
// when the vector grows. The vector is uninitialized and has length 0 but the memory is
// allocated for use. This is useful when the number of elements is known in advance.
// Vec will allocate on the heap while arrays are allocated on the stack.

// Another possiblity here is `memmap` which maps the file to memory and allows for the
// operating system to manage loading the contents into memory as needed transparently.
// This means though that the data isn't on the heap but rather in the OS buffer cache.
// https://github.com/getreu/stringsext

impl Meta {
    /// Discover the media type and create a new instance based on that type
    pub fn parse(mut reader: impl io::Read) -> Result<Self, MetaError> {
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;
        match buf {
            formats::JPEG_HEADER => Ok(Self::Jpeg(Jpeg::parse(reader)?)),
            _ => Err(MetaError::unknown_header(&buf)),
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
        let meta = Meta::parse(&mut header);
        assert!(meta.is_ok());
        assert!(meta.unwrap().kind() == Kind::Jpeg);
    }

    #[test]
    fn test_new_meta_is_not_valid() {
        // unknown header type
        let mut header = io::Cursor::new(&[0xFF, 0x00]);
        assert_eq!(
            Meta::parse(&mut header).unwrap_err().to_string(),
            "metadata unknown header [ff, 00]"
        );

        // bad header length
        let mut header = io::Cursor::new(&[0xFF]);
        let err = Meta::parse(&mut header).unwrap_err();

        assert_eq!(err.to_string(), "metadata file read failed");
        assert_eq!(
            err.as_ref().source().unwrap().to_string(),
            "io::Error: failed to fill whole buffer"
        );
    }
}
