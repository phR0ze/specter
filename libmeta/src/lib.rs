mod meta;

pub mod errors;
pub mod formats;

use std::{fs::File, io, path::Path};

use errors::ParseError;
use meta::*;

/// All essential symbols in a simple consumable form
///
/// ### Examples
/// ```
/// use libmeta::prelude::*;
/// ```
pub mod prelude {
    pub use crate::errors;
    pub use crate::formats;
    pub use crate::meta::*;
}

/// Create a new meta data instance for the given media stream
pub fn new(mut reader: impl io::Read) -> Result<Meta, ParseError> {
    Meta::new(reader)
}
