use libexif::prelude::*;
use nom::{bytes::complete::tag, IResult};
use std::{fs::File, path::Path};

fn parser(input: &str) -> IResult<&str, &str> {
    tag("abc")(input)
}

#[test]
fn test_file_type() {
    let (remaining_input, output) = parser("abcHello").unwrap();
    assert_eq!(remaining_input, "Hello");
    assert_eq!(output, "abc");
}

#[test]
fn test_jpeg() {
    let f = File::open(Path::new("tests/images/nikon-e950.jpg")).unwrap();
    jpeg::exif(f);
    //assert_eq!(jpeg::exif(), 5);
}
