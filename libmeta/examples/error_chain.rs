// Experimenting with better error handling
// -------------------------------------------------------------------------------------------------
use std::{error::Error, fmt};

use libmeta::errors::JpegError;

// Tier 2 error type
// -------------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct JpegSegmentError {
    source: JpegError,
}
impl From<JpegError> for JpegSegmentError {
    fn from(source: JpegError) -> Self {
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
fn first_level_error() -> Result<(), JpegError> {
    let marker: [u8; 1] = [0xFF];

    match nom::sequence::preceded(
        nom::bytes::streaming::tag::<[u8; 1], &[u8], nom::error::Error<&[u8]>>(marker),
        nom::number::streaming::u8,
    )(&[0x00])
    {
        Ok((_, _)) => Ok(()),
        Err(e) => Err(JpegError::parse(": segment marker unknown").with_nom_source(e)),
    }
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
