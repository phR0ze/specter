use std::io;

use crate::errors::ParseError;

// Custom nom error type
//type Res<T, U> = IResult<T, U, VerboseError<T>>;

// JPEG files can contain a variety of different metadata formats, including JFIF, Exif, IPTC, and XMP.
// pub fn exif<T: io::Read>(reader: T) {
//     // Check if the file is a JPEG
//     file_type("hellow").unwrap();

//     println!("Hello, world!");
// }
