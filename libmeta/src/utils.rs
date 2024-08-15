use std::io;

use crate::{
    errors::ParseError,
    formats::{self, Meta},
};

/// Determine the file type
pub(crate) fn file_type(reader: &mut impl io::Read) -> Result<impl Meta, ParseError> {
    let mut buf = [0u8; 2];
    reader.read_exact(&mut buf)?;
    match buf {
        formats::JPEG_PREFIX => Ok(formats::jpeg::Jpeg::new()),
        _ => Err(ParseError::unknown_header(&buf)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_type_is_valid_jpeg() {
        let mut header = io::Cursor::new(&[0xFF, 0xD8]);
        assert!(file_type(&mut header).is_ok());
    }

    #[test]
    fn test_file_type_is_not_valid() {
        // unknown header type
        let mut header = io::Cursor::new(&[0xFF, 0x00]);
        assert_eq!(
            file_type(&mut header).unwrap_err().to_string(),
            "unknown header [ff, 0]"
        );

        // bad header length
        let mut header = io::Cursor::new(&[0xFF]);
        assert_eq!(
            file_type(&mut header).unwrap_err().to_string(),
            "read error: failed to fill whole buffer"
        );
    }
}
