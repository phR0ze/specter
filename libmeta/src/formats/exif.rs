use nom::{bytes::streaming as nom_bytes, error::Error as NomError, number::streaming as nom_nums};

use crate::errors::JpegError;

const JFIF_IDENTIFIER: [u8; 4] = [0x4A, 0x46, 0x49, 0x46];

#[derive(Debug, Clone)]
pub struct Exif {}

impl Default for Exif {
    fn default() -> Self {
        Self {}
    }
}

impl Exif {
    /// Parse the given data into a Exif structure
    /// * **Field**        | **Bytes** | **Description**
    /// * *Identifier*     | 5     | `0x4a 0x46 0x49 0x46 0x00` = `JFIF` in ASCII terminated by a null byte
    /// * *JFIF version*   | 2     | `0x01 0x02` is the major and minor JFIF version i.e. `1.02`
    /// * *Density Units*  | 1     | `0x00` = None, `0x01` = pixels per inch, `0x02` = pixels per centimeter
    /// * *Xdensity*       | 2     | `0x00 0x48` = `72` Horizontal pixel density, Must not be zero
    /// * *Ydensity*       | 2     | `0x00 0x48` = `72` Vertical pixel density, Must not be zero
    /// * *Xthumbnail*     | 1     | `0x00` Horizontal pixels of the embedded RGB thumbnail, May be zero
    /// * *Ythumbnail*     | 1     | `0x00` Vertical pixels of the embedded RGB thumbnail, May be zero
    /// * *Thumbnail data* | 3 x n | Uncompressed 24 bit RGB (8 bits per color channel) raster thumbnail
    pub fn parse(input: &[u8]) -> Result<Exif, JpegError> {
        let mut exif = Exif::default();

        Ok(exif)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXIF_DATA_1: [u8; 20] = [
        0xff, 0xe1, 0x1c, 0x45, 0x45, 0x78, 0x69, 0x66, 0x00, 0x00, 0x49, 0x49, 0x2a, 0x00, 0x08,
        0x00, 0x00, 0x00, 0x0b, 0x00,
    ];

    #[test]
    fn test_parse_exif_success() {
        let exif = Exif::parse(&EXIF_DATA_1[4..]).unwrap();
        //assert_eq!(exif.y_thumbnail, 0);
    }
}
