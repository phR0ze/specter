use libmeta::prelude::*;
use nom::{bytes::complete::tag, IResult};
use std::{fs::File, path::Path};

const Jpeg: u16 = 0xFFD8;

fn file_type(header: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("abc")(input)
}

#[test]
fn test_file_type() {
    let (remaining_input, output) = file_type(&[0xFFD8]).unwrap();
    assert_eq!(remaining_input, "Hello");
    assert_eq!(output, "abc");
}

#[test]
fn test_jpeg() {
    let f = File::open(Path::new("tests/images/nikon-e950.jpg")).unwrap();
    jpeg::exif(f);
    //assert_eq!(jpeg::exif(), 5);
}
