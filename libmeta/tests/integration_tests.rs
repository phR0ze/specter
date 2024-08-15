use libmeta::prelude::*;

use std::{fs::File, io, path::Path};

#[test]
fn test_jpeg() {
    let f = File::open(Path::new("tests/images/nikon-e950.jpg")).unwrap();
    let meta = libmeta::new(&mut io::BufReader::new(f));
    assert!(meta.is_ok());

    // Ensure the file was detected properly
    assert!(meta.unwrap().kind() == MetaKind::Jpeg);

    // Read the JFIF metadata
}

#[test]
fn test_new_meta_is_valid() {
    let mut header = io::Cursor::new(&[0xFF, 0xD8]);
    let meta = libmeta::new(&mut header);
    assert!(meta.is_ok());
    assert!(meta.unwrap().kind() == MetaKind::Jpeg);
}

#[test]
fn test_new_meta_is_not_valid() {
    // unknown header type
    let mut header = io::Cursor::new(&[0xFF, 0x00]);
    assert_eq!(
        libmeta::new(&mut header).unwrap_err().to_string(),
        "unknown header [ff, 0]"
    );

    // bad header length
    let mut header = io::Cursor::new(&[0xFF]);
    assert_eq!(
        libmeta::new(&mut header).unwrap_err().to_string(),
        "read error: failed to fill whole buffer"
    );
}