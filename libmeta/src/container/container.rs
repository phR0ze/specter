use std::fmt;

use super::Jpeg;
use crate::{Exif, MetaResult};

#[derive(Debug)]
pub enum Container {
    Jpeg(Jpeg),
    None,
}

impl Container {
    /// Get the Exif meta data if it exists from the JPEG source and cache it
    pub(crate) fn parse_exif(&self) -> Option<MetaResult<Exif>> {
        match self {
            Container::Jpeg(jpeg) => match jpeg.exif() {
                Some(exif) => match exif {
                    Ok(exif) => Some(Ok(exif)),
                    Err(e) => Some(Err(e.into())),
                },
                _ => None,
            },
            _ => None,
        }
    }
}

impl Default for Container {
    fn default() -> Self {
        Container::None
    }
}

impl fmt::Display for Container {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Container::Jpeg(_) => write!(f, "Jpeg"),
            Container::None => write!(f, "None"),
        }
    }
}
