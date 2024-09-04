// JPEG's are constructed using `Markers`. Markers are a binary formatted value used to mark a segment
// of the file for a specific purpose e.g. start of the image data, end of the image data, app specific
// segments etc...
mod jpeg;
mod marker;
mod segment;
mod test_data;

pub(crate) use jpeg::Jpeg;

// Expose testing data to other modules
#[cfg(test)]
pub(crate) use test_data::JPEG_TEST_DATA;
