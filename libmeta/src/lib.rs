mod meta;

pub mod errors;
pub mod formats;

use std::io;

use errors::MetaError;
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
pub fn new<T: io::Read + io::Seek>(reader: T) -> Result<Meta, MetaError> {
    Meta::parse(reader)
}
