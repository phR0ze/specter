use nom::{bytes::streaming as nom_bytes, error::Error as NomError, number::streaming as nom_nums};

use crate::errors::ExifError;

const EXIF_IDENTIFIER: [u8; 4] = [0x45, 0x78, 0x69, 0x66];

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
    /// * *Identifier*     | 6     | `4578 6966 0000` = `Exif` and 2 bytes of padding 0000
    pub fn parse(input: &[u8]) -> Result<Exif, ExifError> {
        let mut exif = Exif::default();

        // Parse the Exif identifier and drop the results
        let (remain, _) = nom::sequence::terminated(
            nom::bytes::streaming::tag::<[u8; 4], &[u8], nom::error::Error<&[u8]>>(EXIF_IDENTIFIER),
            nom::bytes::streaming::tag::<[u8; 2], &[u8], nom::error::Error<&[u8]>>([0x00, 0x00]),
        )(input)
        .map_err(|x| ExifError::identifier_invalid().with_nom_source(x))?;

        // // Parse the TIFF byte alignment
        // let (remain, (major, minor)) =
        //     nom::sequence::tuple((nom_nums::u8, nom_nums::u8))(remain)
        //         .map_err(|x| JfifError::version_invalid().with_nom_source(x))?;
        // jfif.major = major;
        // jfif.minor = minor;

        Ok(exif)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_parse_exif_success() {
    //     let exif = Exif::parse(&EXIF_DATA_1[4..]).unwrap();
    //     //assert_eq!(exif.y_thumbnail, 0);
    // }
}
