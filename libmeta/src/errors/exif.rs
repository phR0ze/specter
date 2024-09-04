use std::{error::Error, fmt};

use super::{BaseError, ContextError};

#[derive(Debug)]
#[non_exhaustive] // allow for future error fields
pub struct ExifError {
    kind: ExifErrorKind,          // extensible kind
    data: Option<Box<[u8]>>,      // additional error data
    msg: Option<String>,          // optional error message to include
    source: Option<ContextError>, // optional extensible source error
}

impl ExifError {
    pub fn new() -> Self {
        Self { kind: ExifErrorKind::Parse, data: None, msg: None, source: None }
    }

    fn with_kind(kind: ExifErrorKind) -> Self {
        Self { kind, data: None, msg: None, source: None }
    }

    /// Get the error data
    pub fn data(&self) -> Option<&[u8]> {
        match &self.data {
            Some(data) => Some(data.as_ref()),
            None => None,
        }
    }

    /// Get the error kind
    pub fn kind(&self) -> &ExifErrorKind {
        &self.kind
    }

    /// Create a new error for a failed operation
    pub fn parse<T: AsRef<str>>(msg: T) -> Self {
        ExifError::with_kind(ExifErrorKind::Parse).with_msg(msg)
    }

    /// Create a new error for a failed operation
    pub fn offset_zero() -> Self {
        ExifError::with_kind(ExifErrorKind::OffsetIsZero)
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

    // Add a nom source error and override the kind in particular cases
    pub fn with_nom_source(self, source: nom::Err<nom::error::Error<&[u8]>>) -> Self {
        self.with_source("nom::", source)
    }

    // Add an optional source error
    pub fn with_source<T: Error>(mut self, kind: &str, source: T) -> Self {
        self.source = Some(ContextError::from(kind, source));
        self
    }

    // Add an optional source error
    pub fn wrap<T: Error>(mut self, source: T) -> Self {
        self.source = Some(ContextError::from("", source));
        self
    }
}

impl BaseError for ExifError {}

impl fmt::Display for ExifError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            ExifErrorKind::Parse => write!(f, "Exif parse failed")?,
            ExifErrorKind::OffsetIsZero => write!(f, "Exif parse failed: Offset is zero")?,
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
impl Error for ExifError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.source {
            Some(source) => Some(source),
            None => None,
        }
    }
}

// Provides a way to get the generic Error type
impl AsRef<dyn Error> for ExifError {
    fn as_ref(&self) -> &(dyn Error + 'static) {
        self
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ExifErrorKind {
    Parse,
    OffsetIsZero,
}

#[cfg(test)]
mod tests {

    // #[test]
    // fn test_jpeg_use_as_result() {
    //     assert_eq!(
    //         jpeg_error_as_result().unwrap_err().to_string(),
    //         "JPEG segment marker invalid [00, 01]"
    //     );
    // }
}
