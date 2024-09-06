use std::{error::Error, fmt, io};

use super::{BaseError, ContextError, ExifError, JfifError};

#[derive(Debug)]
#[non_exhaustive] // allow for future error fields
pub struct JpegError {
    pub kind: JpegErrorKind,      // extensible kind
    pub data: Option<Box<[u8]>>,  // additional error data
    pub msg: Option<String>,      // optional error message to include
    source: Option<ContextError>, // optional extensible source error
}

impl JpegError {
    pub(crate) fn new(kind: JpegErrorKind) -> Self {
        Self { kind, data: None, msg: None, source: None }
    }

    /// Create a new error for a failed operation
    pub fn operation<T: AsRef<str>>(msg: T) -> Self {
        JpegError::new(JpegErrorKind::Operation).with_msg(msg)
    }

    /// Create a new error for a failed operation
    pub fn parse<T: AsRef<str>>(msg: T) -> Self {
        JpegError::new(JpegErrorKind::Parse).with_msg(msg)
    }

    /// Create a new error for not enough data
    pub(crate) fn truncated() -> Self {
        JpegError::new(JpegErrorKind::Truncated)
    }

    /// Create a new error for a read failure
    pub fn read_failed<T: AsRef<str>>(msg: T) -> Self {
        JpegError::new(JpegErrorKind::ReadFailed).with_msg(msg)
    }

    /// Add additional error data for output with the error message
    pub(crate) fn with_data(mut self, data: &[u8]) -> Self {
        self.data = Some(data.into());
        self
    }

    /// Add optional error message detail for output with the standard error messsage for this kind
    pub(crate) fn with_msg<T: AsRef<str>>(mut self, msg: T) -> Self {
        self.msg = Some(msg.as_ref().into());
        self
    }

    /// Add an optional source error
    pub(crate) fn with_io_source(self, source: io::Error) -> Self {
        self.with_source("io::Error: ", source)
    }

    /// Add a nom source error and override the kind in particular cases
    pub fn with_nom_source(mut self, source: nom::Err<nom::error::Error<&[u8]>>) -> Self {
        if let nom::Err::Incomplete(_) = source {
            self.kind = JpegErrorKind::Truncated;
        } else {
            if source.to_string().contains("requires") {
                self.kind = JpegErrorKind::Truncated;
            }
        }
        self.with_source("nom::", source)
    }

    /// Add an optional source error
    pub(crate) fn with_source<T: Error>(mut self, kind: &str, source: T) -> Self {
        self.source = Some(ContextError::from(kind, source));
        self
    }

    /// Add an optional source error
    pub(crate) fn wrap<T: Error>(mut self, source: T) -> Self {
        self.source = Some(ContextError::from("", source));
        self
    }
}

impl fmt::Display for JpegError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            JpegErrorKind::Operation => write!(f, "JPEG operation failed")?,
            JpegErrorKind::Parse => write!(f, "JPEG parse failed")?,
            JpegErrorKind::Truncated => write!(f, "JPEG truncated")?,
            JpegErrorKind::ReadFailed => write!(f, "JPEG read failed")?,
        };

        // Display additional messaging if available
        if let Some(msg) = self.msg.as_ref() {
            if !msg.is_empty() {
                write!(f, "{}", msg)?;
            };
        };
        if let Some(data) = self.data.as_ref() {
            if data.len() > 0 {
                write!(f, " {:02x?}", data)?;
            };
        };
        Ok(())
    }
}

impl BaseError for JpegError {}

impl Error for JpegError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.source {
            Some(source) => Some(source),
            None => None,
        }
    }
}

// Provides a way to get the generic Error type
impl AsRef<dyn Error> for JpegError {
    fn as_ref(&self) -> &(dyn Error + 'static) {
        self
    }
}

impl From<io::Error> for JpegError {
    fn from(e: io::Error) -> Self {
        JpegError::new(JpegErrorKind::ReadFailed).wrap(e)
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum JpegErrorKind {
    Operation,  // Parsed component was not found
    Parse,      // any parsing related erorrs including nom errors
    Truncated,  // used to signal to read more data in some cases
    ReadFailed, // low level io errors
}

#[cfg(test)]
mod tests {
    use nom::error::{ErrorKind, ParseError};

    use super::*;

    fn jpeg_error_as_result() -> Result<(), JpegError> {
        Err(JpegError::parse(": invalid").with_data(&[0x00, 0x01]))
    }

    #[test]
    fn test_jpeg_use_as_result() {
        assert_eq!(
            jpeg_error_as_result().unwrap_err().to_string(),
            "JPEG parse failed: invalid [00, 01]"
        );
    }

    #[test]
    fn test_segment_marker_invalid_with_data_and_source() {
        let err = JpegError::parse(": segment marker invalid")
            .with_data(&[0x00, 0x01])
            .with_source("nom::", nom::error::Error::from_error_kind(1, ErrorKind::Tag));
        assert_eq!(err.to_string(), "JPEG parse failed: segment marker invalid [00, 01]");
        assert_eq!(err.as_ref().source().unwrap().to_string(), "nom::error Tag at: 1");
    }

    #[test]
    fn test_segment_marker_invalid_with_data_and_io_source() {
        let err = JpegError::parse(": segment marker invalid")
            .with_data(&[0x00, 0x01])
            .with_io_source(io::Error::from(io::ErrorKind::NotFound));
        assert_eq!(err.to_string(), "JPEG parse failed: segment marker invalid [00, 01]");
        if let Some(err) = err.source {
            assert_eq!(err.to_string(), "io::Error: entity not found");
        }
    }
}
