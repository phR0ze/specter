use std::fmt::Display;

use super::marker;
use crate::errors::JpegError;

// JPEG segments are defined by an identifier, their length and the data they contain
#[derive(Debug, PartialEq)]
pub(crate) struct Segment {
    pub(crate) marker: [u8; 2],       // JPEG segment identifier
    pub(crate) length: u16,           // JPEG segment length
    pub(crate) data: Option<Vec<u8>>, // JPEG segment data
}
impl Segment {
    pub(crate) fn new(marker: [u8; 2], length: u16, data: Option<Vec<u8>>) -> Self {
        Self { marker, length, data }
    }

    pub(crate) fn data_to_ascii(&self) -> Result<String, JpegError> {
        match self.data {
            Some(ref data) => {
                let mut ascii = String::new();
                for byte in data {
                    if *byte >= 32 && *byte <= 126 {
                        ascii.push(*byte as char);
                    } else {
                        ascii.push('.');
                    }
                }
                Ok(ascii)
            }
            None => Ok(String::new()),
        }
    }
}

impl Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Print out a Rust array of the data
        writeln!(
            f,
            "const {}: [u8; {}] = [",
            marker::to_string(&self.marker)
                .split_whitespace() // Split the marker into words
                .next() // Get the first word
                .unwrap()
                .to_lowercase(), // Convert to lowercase
            self.length
        )?;
        for line in self.data.as_ref().unwrap().chunks(10) {
            write!(f, "    ")?;
            for byte in line {
                write!(f, "{:#04x}, ", byte)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "];")
    }
}

#[cfg(test)]
mod tests {
    //use super::{super::JPEG_TEST_DATA, *};
}
