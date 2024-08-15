use libmeta::errors::ParseError;

use std::{fs::File, io, path::Path};

#[test]
fn test_jpeg() {
    let f = File::open(Path::new("tests/images/nikon-e950.jpg")).unwrap();
    //jpeg::exif(f);
    //assert_eq!(jpeg::exif(), 5);
}
