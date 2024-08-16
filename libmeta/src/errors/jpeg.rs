use std::{error::Error, fmt, io};

#[derive(Debug)]
#[non_exhaustive] // allow for future error fields
pub struct JpegParseError {
    data: Box<[u8]>,                      // additional error data
    kind: JpegParseErrorKind,             // extensible kind messaging
    source: Option<JpegParseErrorSource>, // optional extensible source error
}

impl JpegParseError {
    // Helper to construct the error
    pub fn segment_marker_invalid() -> Self {
        Self {
            data: Box::new([]),
            kind: JpegParseErrorKind::JpegSegmentMarkerInvalid,
            source: None,
        }
    }

    // Add additional error data for output with the error message
    pub fn with_data(mut self, data: &[u8]) -> Self {
        self.data = data.into();
        self
    }

    // Add an optional source error
    pub fn with_source(mut self, source: io::Error) -> Self {
        self.source = Some(JpegParseErrorSource::Read(source));
        self
    }
}

impl fmt::Display for JpegParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            JpegParseErrorKind::JpegSegmentMarkerInvalid => {
                write!(f, "JPEG segment marker invalid")?
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
            Some(JpegParseErrorSource::Read(e)) => Some(e),
            _ => None,
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum JpegParseErrorKind {
    JpegSegmentMarkerInvalid,
    JpegSegmentLengthInvalid,
    JpegSegmentDataInvalid,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum JpegParseErrorSource {
    Read(std::io::Error),
}

#[cfg(test)]
mod tests {
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
            .with_source(io::Error::from(io::ErrorKind::NotFound));
        assert_eq!(err.to_string(), "JPEG segment marker invalid [00, 01]");
        if let Some(JpegParseErrorSource::Read(err)) = err.source {
            assert_eq!(err.to_string(), "entity not found");
        }
    }
}
