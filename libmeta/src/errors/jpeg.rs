use std::{error::Error, fmt, io};

use super::ContextError;

#[derive(Debug)]
#[non_exhaustive] // allow for future error fields
pub struct JpegError {
    pub kind: JpegErrorKind,      // extensible kind messaging
    data: Box<[u8]>,              // additional error data
    source: Option<ContextError>, // optional extensible source error
}

impl JpegError {
    pub fn new(kind: JpegErrorKind) -> Self {
        Self {
            data: Box::new([]),
            kind,
            source: None,
        }
    }

    pub fn failed() -> Self {
        JpegError::new(JpegErrorKind::Failed)
    }

    pub fn header_invalid() -> Self {
        JpegError::new(JpegErrorKind::HeaderInvalid)
    }

    pub fn not_enough_data() -> Self {
        JpegError::new(JpegErrorKind::NotEnoughData)
    }

    pub fn read_failed() -> Self {
        JpegError::new(JpegErrorKind::ReadFailed)
    }

    pub fn jfif_identifier_invalid() -> Self {
        JpegError::new(JpegErrorKind::JfifIdentifierInvalid)
    }

    pub fn jfif_version_invalid() -> Self {
        JpegError::new(JpegErrorKind::JfifVersionInvalid)
    }

    pub fn jfif_density_units_invalid() -> Self {
        JpegError::new(JpegErrorKind::JfifDensityUnitsInvalid)
    }

    pub fn jfif_density_units_unknown() -> Self {
        JpegError::new(JpegErrorKind::JfifDensityUnitsUnknown)
    }

    pub fn jfif_thumbnail_invalid() -> Self {
        JpegError::new(JpegErrorKind::JfifThumbnailInvalid)
    }

    pub fn jfif_thumbnail_dimensions_invalid() -> Self {
        JpegError::new(JpegErrorKind::JfifThumbnailDimensionsInvalid)
    }

    pub fn segment_invalid() -> Self {
        JpegError::new(JpegErrorKind::JpegSegmentInvalid)
    }

    pub fn segment_marker_invalid() -> Self {
        JpegError::new(JpegErrorKind::JpegSegmentMarkerInvalid)
    }

    pub fn segment_length_invalid() -> Self {
        JpegError::new(JpegErrorKind::JpegSegmentLengthInvalid)
    }

    pub fn segment_data_invalid() -> Self {
        JpegError::new(JpegErrorKind::JpegSegmentDataInvalid)
    }

    pub fn segment_marker_unknown(marker: &[u8]) -> Self {
        JpegError::new(JpegErrorKind::JpegSegmentMarkerUnknown).with_data(marker)
    }

    // Add additional error data for output with the error message
    pub fn with_data(mut self, data: &[u8]) -> Self {
        self.data = data.into();
        self
    }

    // Add an optional source error
    pub fn with_io_source(self, source: io::Error) -> Self {
        self.with_source("io::Error: ", source)
    }

    // Add a nom source error and override the kind in particular cases
    pub fn with_nom_source(mut self, source: nom::Err<nom::error::Error<&[u8]>>) -> Self {
        if let nom::Err::Incomplete(_) = source {
            self.kind = JpegErrorKind::NotEnoughData;
        } else {
            if source.to_string().contains("requires") {
                self.kind = JpegErrorKind::NotEnoughData;
            }
        }
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

impl fmt::Display for JpegError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            JpegErrorKind::Failed => write!(f, "JPEG parse failed")?,
            JpegErrorKind::HeaderInvalid => write!(f, "JPEG header is invalid")?,
            JpegErrorKind::NotEnoughData => write!(f, "JPEG not enough data")?,
            JpegErrorKind::ReadFailed => write!(f, "JPEG read failed")?,
            JpegErrorKind::JfifIdentifierInvalid => write!(f, "JFIF identifier invalid")?,
            JpegErrorKind::JfifVersionInvalid => write!(f, "JFIF version invalid")?,
            JpegErrorKind::JfifDensityUnitsInvalid => write!(f, "JFIF density units invalid")?,
            JpegErrorKind::JfifDensityUnitsUnknown => write!(f, "JFIF density units unknown")?,
            JpegErrorKind::JfifThumbnailInvalid => write!(f, "JFIF thumbnail invalid")?,
            JpegErrorKind::JfifThumbnailDimensionsInvalid => {
                write!(f, "JFIF thumbnail dimensions invalid")?
            }
            JpegErrorKind::JpegSegmentInvalid => write!(f, "JPEG segment invalid")?,
            JpegErrorKind::JpegSegmentMarkerInvalid => write!(f, "JPEG segment marker invalid")?,
            JpegErrorKind::JpegSegmentMarkerUnknown => write!(f, "JPEG segment marker unknown")?,
            JpegErrorKind::JpegSegmentLengthInvalid => write!(f, "JPEG segment length invalid")?,
            JpegErrorKind::JpegSegmentDataInvalid => write!(f, "JPEG segment data invalid")?,
        };

        // Display additional error data if available
        if self.data.len() > 0 {
            write!(f, " {:02x?}", self.data)?;
        };
        Ok(())
    }
}
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

#[derive(Debug)]
#[non_exhaustive]
pub enum JpegErrorKind {
    Failed,
    HeaderInvalid,
    NotEnoughData,
    ReadFailed,
    JfifIdentifierInvalid,
    JfifVersionInvalid,
    JfifDensityUnitsInvalid,
    JfifDensityUnitsUnknown,
    JfifThumbnailInvalid,
    JfifThumbnailDimensionsInvalid,
    JpegSegmentInvalid,
    JpegSegmentMarkerInvalid,
    JpegSegmentMarkerUnknown,
    JpegSegmentLengthInvalid,
    JpegSegmentDataInvalid,
}

#[cfg(test)]
mod tests {
    use nom::error::{ErrorKind, ParseError};

    use super::*;

    fn jpeg_error_as_result() -> Result<(), JpegError> {
        Err(JpegError::segment_marker_invalid().with_data(&[0x00, 0x01]))
    }

    #[test]
    fn test_jpeg_use_as_result() {
        assert_eq!(
            jpeg_error_as_result().unwrap_err().to_string(),
            "JPEG segment marker invalid [00, 01]"
        );
    }

    #[test]
    fn test_segment_marker_invalid_without_data() {
        assert_eq!(
            JpegError::segment_marker_invalid().to_string(),
            "JPEG segment marker invalid"
        );
    }

    #[test]
    fn test_segment_marker_invalid_with_data() {
        assert_eq!(
            JpegError::segment_marker_invalid()
                .with_data(&[0x00, 0x01])
                .to_string(),
            "JPEG segment marker invalid [00, 01]"
        );
    }

    #[test]
    fn test_segment_marker_invalid_with_data_and_source() {
        let err = JpegError::segment_marker_invalid()
            .with_data(&[0x00, 0x01])
            .with_source(
                "nom::",
                nom::error::Error::from_error_kind(1, ErrorKind::Tag),
            );
        assert_eq!(err.to_string(), "JPEG segment marker invalid [00, 01]");
        assert_eq!(
            err.as_ref().source().unwrap().to_string(),
            "nom::error Tag at: 1"
        );
    }

    #[test]
    fn test_segment_marker_invalid_with_data_and_io_source() {
        let err = JpegError::segment_marker_invalid()
            .with_data(&[0x00, 0x01])
            .with_io_source(io::Error::from(io::ErrorKind::NotFound));
        assert_eq!(err.to_string(), "JPEG segment marker invalid [00, 01]");
        if let Some(err) = err.source {
            assert_eq!(err.to_string(), "io::Error: entity not found");
        }
    }
}
