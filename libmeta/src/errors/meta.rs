use std::{error::Error, fmt, io};

use crate::Meta;

use super::{ContextError, JpegParseError};

#[derive(Debug)]
#[non_exhaustive]
pub struct MetaError {
    pub data: Box<[u8]>,
    pub kind: MetaErrorKind,
    pub source: Option<MetaErrorSource>,
}

impl MetaError {
    pub fn unknown_header(data: &[u8]) -> Self {
        Self {
            data: data.into(),
            kind: MetaErrorKind::UnknownHeader,
            source: None,
        }
    }
}

impl fmt::Display for MetaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            MetaErrorKind::Read => write!(f, "metadata file read failed")?,
            MetaErrorKind::Jpeg => write!(f, "metadata jpeg failed")?,
            MetaErrorKind::UnknownHeader => write!(f, "metadata unknown header")?,
        };

        // Display additional error data if available
        if self.data.len() > 0 {
            write!(f, " {:02x?}", self.data)?;
        };
        Ok(())
    }
}

impl Error for MetaError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.source {
            Some(MetaErrorSource::Io(source)) => Some(source),
            Some(MetaErrorSource::JpegParse(source)) => Some(source),
            None => None,
        }
    }
}

// Provides a way to get the generic Error type
impl AsRef<dyn Error> for MetaError {
    fn as_ref(&self) -> &(dyn Error + 'static) {
        self
    }
}

impl From<io::Error> for MetaError {
    fn from(e: io::Error) -> Self {
        Self {
            data: Box::new([]),
            kind: MetaErrorKind::Read,
            source: Some(MetaErrorSource::Io(ContextError::from("io::Error: ", e))),
        }
    }
}

impl From<JpegParseError> for MetaError {
    fn from(e: JpegParseError) -> Self {
        Self {
            data: Box::new([]),
            kind: MetaErrorKind::Jpeg,
            source: Some(MetaErrorSource::JpegParse(e)),
        }
    }
}

/// An extensible way to capture various error message types
#[derive(Debug)]
#[non_exhaustive]
pub enum MetaErrorKind {
    #[non_exhaustive]
    Read,

    #[non_exhaustive]
    Jpeg,

    #[non_exhaustive]
    UnknownHeader,
}

/// The kind of parse errors that can be generated
#[derive(Debug)]
#[non_exhaustive]
pub enum MetaErrorSource {
    Io(ContextError),
    JpegParse(JpegParseError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unknown_header() {
        assert_eq!(
            MetaError::unknown_header(&[0xFF, 0xD8]).to_string(),
            "metadata unknown header [ff, d8]"
        );
    }
}
