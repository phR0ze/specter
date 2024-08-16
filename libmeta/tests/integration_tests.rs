use libmeta::prelude::*;

use std::{fs::File, io, path::Path};

#[test]
fn test_jpeg() {
    let f = File::open(Path::new("tests/images/nikon-e950.jpg")).unwrap();
    let buf = io::BufReader::new(f);
    let meta = libmeta::new(buf);
    assert!(meta.is_ok());
    let meta = meta.unwrap();

    // Ensure the file was detected properly
    assert!(meta.kind() == Kind::Jpeg);

    // Downcast to a jpeg
    let jpeg = meta.as_jpeg();
    assert!(jpeg.is_ok());
    let jpeg = jpeg.unwrap();

    // Read the JFIF metadata
    let jfif = jpeg.jfif.as_ref();
}

#[test]
fn test_new_meta_is_valid() {
    let mut header = io::Cursor::new(&[0xFF, 0xD8]);
    let meta = libmeta::new(&mut header);
    assert!(meta.is_ok());
    assert!(meta.unwrap().kind() == Kind::Jpeg);
}

#[test]
fn test_new_meta_is_not_valid() {
    // unknown header type
    let mut header = io::Cursor::new(&[0xFF, 0x00]);
    let err = libmeta::new(&mut header).unwrap_err();
    assert_eq!(err.to_string(), "metadata unknown header [ff, 00]");
    assert_eq!(err.as_ref().source().is_none(), true);

    // bad header length
    let mut header = io::Cursor::new(&[0xFF]);
    let err = libmeta::new(&mut header).unwrap_err();
    assert_eq!(err.to_string(), "metadata file read failed");
    assert_eq!(
        err.as_ref().source().unwrap().to_string(),
        "io::Error: failed to fill whole buffer"
    );
}
