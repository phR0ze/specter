// JPEG's are constructed using `Markers`. Markers are a binary formatted value used to mark a segment
// of the file for a specific purpose e.g. start of the image data, end of the image data, app specific
// segments etc...
pub(crate) mod marker;
mod parser;
mod segment;

pub use parser::*;
