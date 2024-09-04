use libmeta::prelude::*;

use std::{fs::File, io, path::Path};

#[test]
fn test_jpeg() {
    // let f = File::open(Path::new("tests/images/nikon-e950.jpg")).unwrap();
    // let mut buf = io::BufReader::new(f);
    // let meta = libmeta::parse(&mut buf);
    // assert!(meta.is_ok());
    // let meta = meta.unwrap();

    // // Ensure the file was detected properly
    // assert!(meta.kind() == Kind::Jpeg);
}

// #[test]
// fn test_meta_parse_header_is_valid() {
//     let mut header = io::Cursor::new(&[0xFF, 0xD8, 0xFF, 0xDA]);
//     let meta = libmeta::parse(&mut header);
//     assert!(meta.is_ok());
//     assert_eq!(meta.unwrap().is_jpeg(), true);
// }

#[test]
fn test_meta_parse_header_is_not_valid() {
    let mut header = io::Cursor::new(&[0xFF, 0x00]);
    let err = libmeta::parse(&mut header).unwrap_err();
    assert_eq!(err.to_string(), "metadata unknown header [ff, 00]");
    assert_eq!(err.as_ref().source().is_none(), true);
}

#[test]
fn test_meta_parse_header_is_not_enough_data() {
    let mut header = io::Cursor::new(&[0xFF]);
    let err = libmeta::parse(&mut header).unwrap_err();
    assert_eq!(err.to_string(), "metadata unknown header [ff]");
}
