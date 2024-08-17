use std::{error::Error, fmt, io};

use super::ContextError;

#[derive(Debug)]
#[non_exhaustive] // allow for future error fields
pub struct JpegParseError {
    data: Box<[u8]>,              // additional error data
    kind: JpegParseErrorKind,     // extensible kind messaging
    source: Option<ContextError>, // optional extensible source error
}

impl JpegParseError {
    pub fn header_invalid() -> Self {
        Self {
            data: Box::new([]),
            kind: JpegParseErrorKind::HeaderInvalid,
            source: None,
        }
    }

    pub fn jfif_identifier_invalid() -> Self {
        Self {
            data: Box::new([]),
            kind: JpegParseErrorKind::JfifIdentifierInvalid,
            source: None,
        }
    }

    pub fn jfif_version_invalid() -> Self {
        Self {
            data: Box::new([]),
            kind: JpegParseErrorKind::JfifVersionInvalid,
            source: None,
        }
    }

    pub fn jfif_density_units_invalid() -> Self {
        Self {
            data: Box::new([]),
            kind: JpegParseErrorKind::JfifDensityUnitsInvalid,
            source: None,
        }
    }

    pub fn jfif_density_units_unknown() -> Self {
        Self {
            data: Box::new([]),
            kind: JpegParseErrorKind::JfifDensityUnitsUnknown,
            source: None,
        }
    }

    pub fn jfif_thumbnail_invalid() -> Self {
        Self {
            data: Box::new([]),
            kind: JpegParseErrorKind::JfifThumbnailInvalid,
            source: None,
        }
    }

    pub fn jfif_thumbnail_dimensions_invalid() -> Self {
        Self {
            data: Box::new([]),
            kind: JpegParseErrorKind::JfifThumbnailDimensionsInvalid,
            source: None,
        }
    }

    pub fn segment_invalid() -> Self {
        Self {
            data: Box::new([]),
            kind: JpegParseErrorKind::JpegSegmentInvalid,
            source: None,
        }
    }

    pub fn segment_marker_invalid() -> Self {
        Self {
            data: Box::new([]),
            kind: JpegParseErrorKind::JpegSegmentMarkerInvalid,
            source: None,
        }
    }

    pub fn segment_length_invalid() -> Self {
        Self {
            data: Box::new([]),
            kind: JpegParseErrorKind::JpegSegmentLengthInvalid,
            source: None,
        }
    }

    pub fn segment_data_invalid() -> Self {
        Self {
            data: Box::new([]),
            kind: JpegParseErrorKind::JpegSegmentDataInvalid,
            source: None,
        }
    }

    pub fn segment_marker_unknown(marker: &[u8]) -> Self {
        Self {
            data: marker.into(),
            kind: JpegParseErrorKind::JpegSegmentMarkerUnknown,
            source: None,
        }
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

    // Add an optional source error
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

impl fmt::Display for JpegParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            JpegParseErrorKind::HeaderInvalid => write!(f, "JPEG header is invalid")?,
            JpegParseErrorKind::JfifIdentifierInvalid => write!(f, "JFIF identifier invalid")?,
            JpegParseErrorKind::JfifVersionInvalid => write!(f, "JFIF version invalid")?,
            JpegParseErrorKind::JfifDensityUnitsInvalid => write!(f, "JFIF density units invalid")?,
            JpegParseErrorKind::JfifDensityUnitsUnknown => write!(f, "JFIF density units unknown")?,
            JpegParseErrorKind::JfifThumbnailInvalid => write!(f, "JFIF thumbnail invalid")?,
            JpegParseErrorKind::JfifThumbnailDimensionsInvalid => {
                write!(f, "JFIF thumbnail dimensions invalid")?
            }
            JpegParseErrorKind::JpegSegmentInvalid => write!(f, "JPEG segment invalid")?,
            JpegParseErrorKind::JpegSegmentMarkerInvalid => {
                write!(f, "JPEG segment marker invalid")?
            }
            JpegParseErrorKind::JpegSegmentMarkerUnknown => {
                write!(f, "JPEG segment marker unknown")?
            }
            JpegParseErrorKind::JpegSegmentLengthInvalid => {
                write!(f, "JPEG segment length invalid")?
            }
            JpegParseErrorKind::JpegSegmentDataInvalid => write!(f, "JPEG segment data invalid")?,
        };

        // Display additional error data if available
        if self.data.len() > 0 {
            write!(f, " {:02x?}", self.data)?;
        };
        Ok(())
    }
}
impl Error for JpegParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.source {
            Some(source) => Some(source),
            None => None,
        }
    }
}

// Provides a way to get the generic Error type
impl AsRef<dyn Error> for JpegParseError {
    fn as_ref(&self) -> &(dyn Error + 'static) {
        self
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum JpegParseErrorKind {
    HeaderInvalid,
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

    fn jpeg_error_as_result() -> Result<(), JpegParseError> {
        Err(JpegParseError::segment_marker_invalid().with_data(&[0x00, 0x01]))
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
            JpegParseError::segment_marker_invalid().to_string(),
            "JPEG segment marker invalid"
        );
    }

    #[test]
    fn test_segment_marker_invalid_with_data() {
        assert_eq!(
            JpegParseError::segment_marker_invalid()
                .with_data(&[0x00, 0x01])
                .to_string(),
            "JPEG segment marker invalid [00, 01]"
        );
    }

    #[test]
    fn test_segment_marker_invalid_with_data_and_source() {
        let err = JpegParseError::segment_marker_invalid()
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
        let err = JpegParseError::segment_marker_invalid()
            .with_data(&[0x00, 0x01])
            .with_io_source(io::Error::from(io::ErrorKind::NotFound));
        assert_eq!(err.to_string(), "JPEG segment marker invalid [00, 01]");
        if let Some(err) = err.source {
            assert_eq!(err.to_string(), "io::Error: entity not found");
        }
    }
}
