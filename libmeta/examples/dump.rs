use libmeta::errors::MetaError;
use std::{fs::File, path::Path};

fn main() -> anyhow::Result<()> {
    //let f = File::open(Path::new("tests/images/nikon-e950.jpg"));
    //jpeg::exif(f);
    return Err(MetaError::unknown_header(&[0xFF, 0xD8]).into());
}
