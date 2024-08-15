use std::{error::Error, fmt};

#[derive(Debug)]
#[non_exhaustive]
pub struct ParseError {
    pub data: Box<[u8]>,
    pub kind: ParseErrorKind,
}

impl ParseError {
    /// Create a new `ParseError` with the `UnknownHeader` kind.
    pub fn unknown_header(data: &[u8]) -> Self {
        Self {
            data: data.into(),
            kind: ParseErrorKind::UnknownHeader,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            ParseErrorKind::UnknownHeader => write!(f, "unknown header {:x?}", self.data),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            ParseErrorKind::UnknownHeader => None,
        }
    }
}

/// The kind of parse errors that can be generated
#[derive(Debug)]
#[non_exhaustive]
pub enum ParseErrorKind {
    #[non_exhaustive]
    UnknownHeader,
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
