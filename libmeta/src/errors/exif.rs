use std::{error::Error, fmt};

use super::ContextError;

#[derive(Debug)]
#[non_exhaustive] // allow for future error fields
pub struct ExifError {
    pub kind: ExifErrorKind,      // extensible kind messaging
    data: Box<[u8]>,              // additional error data
    source: Option<ContextError>, // optional extensible source error
}

impl ExifError {
    pub fn new(kind: ExifErrorKind) -> Self {
        Self {
            data: Box::new([]),
            kind,
            source: None,
        }
    }

    pub fn identifier_invalid() -> Self {
        ExifError::new(ExifErrorKind::IdentifierInvalid)
    }

    pub fn alignment_invalid() -> Self {
        ExifError::new(ExifErrorKind::AlignmentInvalid)
    }

    pub fn length_invalid() -> Self {
        ExifError::new(ExifErrorKind::LengthInvalid)
    }

    pub fn offset_failed() -> Self {
        ExifError::new(ExifErrorKind::OffsetFailed)
    }

    // Add additional error data for output with the error message
    pub fn with_data(mut self, data: &[u8]) -> Self {
        self.data = data.into();
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

impl fmt::Display for ExifError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            ExifErrorKind::IdentifierInvalid => write!(f, "Exif identifier invalid")?,
            ExifErrorKind::AlignmentInvalid => write!(f, "Exif alignment invalid")?,
            ExifErrorKind::LengthInvalid => write!(f, "Exif length invalid")?,
            ExifErrorKind::OffsetFailed => write!(f, "Exif ifd offset failed")?,
        };

        // Display additional error data if available
        if self.data.len() > 0 {
            write!(f, " {:02x?}", self.data)?;
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
    IdentifierInvalid,
    AlignmentInvalid,
    LengthInvalid,
    OffsetFailed,
}

#[cfg(test)]
mod tests {
    use nom::error::{ErrorKind, ParseError};

    use super::*;

    // #[test]
    // fn test_jpeg_use_as_result() {
    //     assert_eq!(
    //         jpeg_error_as_result().unwrap_err().to_string(),
    //         "JPEG segment marker invalid [00, 01]"
    //     );
    // }
}
