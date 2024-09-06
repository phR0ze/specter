use std::{
    cell::{Ref, RefCell},
    fmt::Display,
    io::{self, Read},
    ops::Deref,
};

use crate::{
    container::{Container, Jpeg},
    errors::MetaError,
};

use super::{Exif, Jfif};

/// Simplify the Exif return type slightly
pub type MetaResult<T> = Result<T, MetaError>;

/// Meta provides encapsulation for the different metadata types that can be parsed
/// out of a variety of media types.
#[derive(Debug)]
#[non_exhaustive]
pub struct Meta {
    container: Option<Container>,
    jfif: RefCell<Option<Jfif>>,
    exif: RefCell<Option<Exif>>,
}

impl Meta {
    /// Private default constructor
    fn default() -> Self {
        Self { container: None, jfif: RefCell::new(None), exif: RefCell::new(None) }
    }

    /// Discover the media type and create a new instance based on that type
    pub(crate) fn parse<T: io::BufRead + io::Seek>(mut reader: T) -> MetaResult<Self> {
        // TODO:
        // * read some larger amount for magic file header testing
        // * try file extension if header is not recognized needed
        // * scan file for JPEG/TIFF markers?
        // * split out container types as separate features?
        let mut header = Vec::new();
        reader.by_ref().take(2).read_to_end(&mut header)?;

        // Create a new instance based on the media type
        let mut meta = Self::default();
        if Jpeg::is_jpeg(&header) {
            meta.container = Some(Container::Jpeg(Jpeg::parse(header.chain(reader))?));

            // TODO: run this only as needed
            meta.cache_jfif();
            meta.cache_exif();

            Ok(meta)
        } else {
            Err(MetaError::unknown_header(&header))
        }
    }

    /// Is the meta data type from a JPEG container
    pub(crate) fn is_jpeg(&self) -> bool {
        match self.container {
            Some(Container::Jpeg(_)) => true,
            _ => false,
        }
    }

    /// Get the JFIF meta data if it exists from the JPEG source and cache it
    fn cache_jfif(&self) -> Option<MetaResult<()>> {
        match &self.container {
            Some(Container::Jpeg(jpeg)) => match jpeg.jfif() {
                Some(jfif) => match jfif {
                    Ok(jfif) => {
                        self.jfif.borrow_mut().replace(jfif);
                        Some(Ok(()))
                    }
                    Err(e) => Some(Err(e.into())),
                },
                _ => None,
            },
            _ => None,
        }
    }

    /// Get the Exif meta data if it exists from the JPEG source and cache it
    fn cache_exif(&self) -> Option<MetaResult<()>> {
        match &self.container {
            Some(Container::Jpeg(jpeg)) => match jpeg.exif() {
                Some(exif) => match exif {
                    Ok(exif) => {
                        self.exif.borrow_mut().replace(exif);
                        Some(Ok(()))
                    }
                    Err(e) => Some(Err(e.into())),
                },
                _ => None,
            },
            _ => None,
        }
    }
}

impl Display for Meta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "  {: <32}: {}", "libmeta Version".to_string(), crate::VERSION)?;
        // if let Some(Container::Jpeg(_)) = &self.container {
        //     writeln!(f, "  {: <32}: {}", "File Type".to_string(), "JPEG".to_string())?;
        // } else {
        //     writeln!(f, "  {: <32}: {}", "File Type".to_string(), "None".to_string())?;
        // }

        // if let Some(ref jfif) = *self.jfif.borrow() {
        //     writeln!(f, "{}", jfif)?;
        // }
        if let Some(ref exif) = *self.exif.borrow() {
            writeln!(f, "{}", exif)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::container::JPEG_TEST_DATA;

    #[test]
    fn test_meta_parse_header_is_valid_jpeg() {
        let mut data = io::Cursor::new(&JPEG_TEST_DATA);
        let meta = Meta::parse(&mut data).unwrap();
        assert_eq!(meta.is_jpeg(), true);
    }

    #[test]
    fn test_meta_parse_not_enough_data() {
        let mut header = io::Cursor::new(&[0xFF]);
        let err = Meta::parse(&mut header).unwrap_err();
        assert_eq!(err.to_string(), "metadata unknown header [ff]");
    }

    #[test]
    fn test_meta_parse_header_is_invalid() {
        // unknown header type
        let mut header = io::Cursor::new(&[0xFF, 0x00]);
        assert_eq!(
            Meta::parse(&mut header).unwrap_err().to_string(),
            "metadata unknown header [ff, 00]"
        );
    }
}
