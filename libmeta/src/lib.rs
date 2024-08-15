mod meta;

pub mod errors;
pub mod formats;

use std::io;

use errors::ParseError;
use meta::*;

/// All essential symbols in a simple consumable form
///
/// ### Examples
/// ```
/// use libmeta::prelude::*;
/// ```
pub mod prelude {
    pub use crate::errors;
    pub use crate::formats;
    pub use crate::meta::*;
}

/// Create a new meta data instance for the given media stream
pub fn new(reader: &mut impl io::Read) -> Result<impl Meta, ParseError> {
    let mut buf = [0u8; 2];
    reader.read_exact(&mut buf)?;
    match buf {
        formats::JPEG_PREFIX => Ok(formats::Jpeg::new()),
        _ => Err(ParseError::unknown_header(&buf)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_meta_is_valid_jpeg() {
        let mut header = io::Cursor::new(&[0xFF, 0xD8]);
        let meta = new(&mut header);
        assert!(meta.is_ok());
        assert!(meta.unwrap().kind() == MetaKind::Jpeg);
    }

    #[test]
    fn test_new_meta_is_not_valid() {
        // unknown header type
        let mut header = io::Cursor::new(&[0xFF, 0x00]);
        assert_eq!(
            new(&mut header).unwrap_err().to_string(),
            "unknown header [ff, 0]"
        );

        // bad header length
        let mut header = io::Cursor::new(&[0xFF]);
        assert_eq!(
            new(&mut header).unwrap_err().to_string(),
            "read error: failed to fill whole buffer"
        );
    }
}
