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
    pub fn new() -> Self {
        Self { jfif: None }
    }
}

impl Meta for Jpeg {
    fn new() -> Self {
        Self::new()
    }

    fn discover(&self, reader: &mut impl io::Read) -> Result<(), ParseError> {
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn kind(&self) -> Kind {
        Kind::Jpeg
    }

    fn as_jpeg(&self) -> Result<&Jpeg, CastError> {
        match self.as_any().downcast_ref::<Jpeg>() {
            Some(jpg) => Ok(jpg),
            None => Err(CastError::new(format!("Jpeg real type {}", self.kind()))),
        }
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
