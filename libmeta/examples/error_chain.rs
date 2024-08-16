// Experimenting with better error handling
// -------------------------------------------------------------------------------------------------
use std::{error::Error, fmt, io};

use libmeta::errors::JpegParseError;

// Tier 2 error type
// -------------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct JpegSegmentError {
    source: JpegParseError,
}
impl From<JpegParseError> for JpegSegmentError {
    fn from(source: JpegParseError) -> Self {
        Self { source }
    }
}
impl fmt::Display for JpegSegmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "JPEG segment parsing failed")
    }
}
impl Error for JpegSegmentError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

// Tier 3 error type
// -------------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct JpegMetaError {
    source: JpegSegmentError,
}
impl From<JpegSegmentError> for JpegMetaError {
    fn from(source: JpegSegmentError) -> Self {
        Self { source }
    }
}
impl fmt::Display for JpegMetaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "JPEG metadata parsing failed")
    }
}
impl Error for JpegMetaError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

// Tier 1 error handling
// -------------------------------------------------------------------------------------------------
fn first_level_error() -> Result<(), JpegParseError> {
    Err(JpegParseError::segment_marker_invalid()
        .with_data(&[0xFF, 0xD8])
        .with_source(io::Error::from(io::ErrorKind::NotFound)))
}

// Tier 2 error handling
// -------------------------------------------------------------------------------------------------
fn second_level_error() -> Result<(), JpegSegmentError> {
    first_level_error()?;
    Ok(())
}

// Tier 3 error handling
// -------------------------------------------------------------------------------------------------
fn third_level_error() -> Result<(), JpegMetaError> {
    second_level_error()?;
    Ok(())
}

// Tier 3 error handling
// -------------------------------------------------------------------------------------------------
fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LIB_BACKTRACE", "0");
    third_level_error().map_err(|x| anyhow::Error::new(x))?;
    Ok(())
}
