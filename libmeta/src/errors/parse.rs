use std::{error::Error, fmt, io};

#[derive(Debug)]
#[non_exhaustive]
pub struct ParseError {
    pub data: Box<[u8]>,
    pub kind: ParseErrorKind,
}

impl ParseError {
    /// Create a new `ParseError` with the `UnknownHeader` kind.
    pub fn invalid_jpeg_marker(data: &[u8]) -> Self {
        Self {
            data: data.into(),
            kind: ParseErrorKind::UnknownHeader,
        }
    }

    /// Create a new `ParseError` with the `UnknownHeader` kind.
    pub fn unknown_header(data: &[u8]) -> Self {
        Self {
            data: data.into(),
            kind: ParseErrorKind::UnknownHeader,
        }
    }

    pub fn read(e: io::Error) -> Self {
        Self {
            data: Box::new([]),
            kind: ParseErrorKind::Read(e),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            ParseErrorKind::InvalidJpegMarker => write!(f, "invalid jpeg marker {:x?}", self.data),
            ParseErrorKind::Read(e) => write!(f, "read error: {}", e),
            ParseErrorKind::UnknownHeader => write!(f, "unknown header {:x?}", self.data),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            ParseErrorKind::InvalidJpegMarker => None,
            ParseErrorKind::Read(e) => Some(e),
            ParseErrorKind::UnknownHeader => None,
        }
    }
}

impl From<io::Error> for ParseError {
    fn from(e: io::Error) -> Self {
        Self::read(e)
    }
}

/// The kind of parse errors that can be generated
#[derive(Debug)]
#[non_exhaustive]
pub enum ParseErrorKind {
    Read(io::Error),

    #[non_exhaustive]
    UnknownHeader,

    #[non_exhaustive]
    InvalidJpegMarker,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unknown_header() {
        assert_eq!(
            ParseError::unknown_header(&[0xFF, 0xD8]).to_string(),
            "unknown header [ff, d8]"
        );
    }
}
