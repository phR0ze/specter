use std::io;

// JPEG File Interchange Format (JFIF) is supplementary image file format to JPEG.
//
// https://en.wikipedia.org/wiki/JPEG_File_Interchange_Format
//

pub fn exif<T: io::Read>(reader: T) {
    println!("Hello, world!");
}
