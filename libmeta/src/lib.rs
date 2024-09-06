mod container;
pub mod errors;
mod meta;

use std::io;

use meta::*;

pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");

/// All essential symbols in a simple consumable form
///
/// ### Examples
/// ```
/// use libmeta::prelude::*;
/// ```
pub mod prelude {
    pub use crate::container::*;
    pub use crate::errors::*;
    //pub use crate::meta::*;
}

/// Create a new meta data instance for the given media stream
pub fn parse<T: io::BufRead + io::Seek>(reader: T) -> MetaResult<Meta> {
    Meta::parse(reader)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_parse() {
        //
    }
}
