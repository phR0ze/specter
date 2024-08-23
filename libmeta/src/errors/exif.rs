use std::{error::Error, fmt};

use super::{BaseError, ContextError};

#[derive(Debug)]
#[non_exhaustive] // allow for future error fields
pub struct ExifError {
    pub kind: ExifErrorKind,         // extensible kind messaging
    data: Option<ExifErrorDataKind>, // additional error data
    source: Option<ContextError>,    // optional extensible source error
}

impl ExifError {
    pub fn new(kind: ExifErrorKind) -> Self {
        Self {
            data: None,
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

    pub fn marker_invalid() -> Self {
        ExifError::new(ExifErrorKind::MarkerInvalid)
    }

    pub fn count_invalid() -> Self {
        ExifError::new(ExifErrorKind::CountInvalid)
    }

    pub fn offset_failed() -> Self {
        ExifError::new(ExifErrorKind::OffsetFailed)
    }

    pub fn file_failed() -> Self {
        ExifError::new(ExifErrorKind::FileFailed)
    }

    pub fn ifd_file_tag_failed() -> Self {
        ExifError::new(ExifErrorKind::IfdFileTagFailed)
    }

    pub fn ifd_file_data_format_failed() -> Self {
        ExifError::new(ExifErrorKind::IfdFileDataFormatFailed)
    }

    pub fn ifd_file_component_count_failed() -> Self {
        ExifError::new(ExifErrorKind::IfdFileComponentCountFailed)
    }

    // Add additional error data for output with the error message
    pub fn with_str<T: fmt::Display>(mut self, str: T) -> Self {
        self.data = Some(ExifErrorDataKind::String(str.to_string()));
        self
    }

    // Add additional error data for output with the error message
    pub fn with_data(mut self, data: &[u8]) -> Self {
        self.data = Some(ExifErrorDataKind::Bytes(data.into()));
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
            ExifErrorKind::IdentifierInvalid => write!(f, "Exif identifier invalid")?,
            ExifErrorKind::AlignmentInvalid => write!(f, "Exif TIFF alignment invalid")?,
            ExifErrorKind::MarkerInvalid => write!(f, "Exif IFD marker invalid")?,
            ExifErrorKind::CountInvalid => write!(f, "Exif IFD entries count invalid")?,
            ExifErrorKind::OffsetFailed => write!(f, "Exif IFD offset failed")?,
            ExifErrorKind::FileFailed => write!(f, "Exif IFD file failed")?,
            ExifErrorKind::IfdFileTagFailed => write!(f, "Exif IFD file tag failed")?,
            ExifErrorKind::IfdFileDataFormatFailed => {
                write!(f, "Exif IFD file data format failed")?
            }
            ExifErrorKind::IfdFileComponentCountFailed => {
                write!(f, "Exif IFD file component count failed")?
            }
        };

        // Display additional error data if available
        if let Some(ExifErrorDataKind::String(str)) = &self.data {
            write!(f, ": {}", str)?;
        } else if let Some(ExifErrorDataKind::Bytes(data)) = &self.data {
            write!(f, ": {:02x?}", data)?;
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
pub enum ExifErrorDataKind {
    String(String),
    Bytes(Box<[u8]>),
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ExifErrorKind {
    IdentifierInvalid,
    AlignmentInvalid,
    MarkerInvalid,
    CountInvalid,
    OffsetFailed,
    FileFailed,
    IfdFileTagFailed,
    IfdFileDataFormatFailed,
    IfdFileComponentCountFailed,
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
