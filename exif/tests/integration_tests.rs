use libexif::prelude::*;
use std::{fs::File, path::Path};

#[test]
fn test_jpeg() {
    let f = File::open(Path::new("tests/images/nikon-e950.jpg")).unwrap();
    jpeg::exif(f);
    //assert_eq!(jpeg::exif(), 5);
}
