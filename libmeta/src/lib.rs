pub mod errors;
pub mod formats;
mod utils;

/// All essential symbols in a simple consumable form
///
/// ### Examples
/// ```
/// use libmeta::prelude::*;
/// ```
pub mod prelude {
    pub use crate::errors;
    pub use crate::formats;
}
