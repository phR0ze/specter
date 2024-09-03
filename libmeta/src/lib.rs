pub mod errors;
mod formats;
mod meta;

use std::io;

pub use errors::MetaError;
use meta::*;

/// All essential symbols in a simple consumable form
///
/// ### Examples
/// ```
/// use libmeta::prelude::*;
/// ```
pub mod prelude {
    pub use crate::errors::*;
    pub use crate::formats::*;
    pub use crate::meta::*;
}

/// Create a new meta data instance for the given media stream
pub fn parse<T: io::BufRead + io::Seek>(reader: &mut T) -> Result<Meta, MetaError> {
    Meta::parse(reader)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        //
    }
}
