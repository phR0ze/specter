use std::{any::Any, io};

use super::Jfif;
use crate::{
    errors::{CastError, ParseError},
    Kind, Meta,
};

#[derive(Debug)]
pub struct Jpeg {
    pub jfif: Option<Jfif>,
}

impl Jpeg {
    pub fn new(mut reader: impl io::Read) -> Result<Self, ParseError> {
        Ok(Self { jfif: None })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_meta_is_valid_jpeg() {
        // let mut header = io::Cursor::new(&[0xFF, 0xD8]);
        // let meta = new(&mut header);
        // assert!(meta.is_ok());
        // assert!(meta.unwrap().kind() == Kind::Jpeg);
    }

    #[test]
    fn test_new_meta_is_not_valid() {
        // // unknown header type
        // let mut header = io::Cursor::new(&[0xFF, 0x00]);
        // assert_eq!(
        //     new(&mut header).unwrap_err().to_string(),
        //     "unknown header [ff, 0]"
        // );

        // // bad header length
        // let mut header = io::Cursor::new(&[0xFF]);
        // assert_eq!(
        //     new(&mut header).unwrap_err().to_string(),
        //     "read error: failed to fill whole buffer"
        // );
    }
}
