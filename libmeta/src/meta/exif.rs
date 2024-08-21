use std::f32::consts::E;

use nom::{bytes::streaming as nom_bytes, error::Error as NomError, number::streaming as nom_nums};

use crate::errors::{ExifError, ExifErrorKind};

const EXIF_IDENTIFIER: [u8; 4] = [0x45, 0x78, 0x69, 0x66];
const BIG_ENDIAN: [u8; 2] = [0x4D, 0x4D];
const LITTLE_ENDIAN: [u8; 2] = [0x49, 0x49];

#[derive(Debug, Clone)]
pub struct Exif {
    align: Option<[u8; 2]>,
}

impl Default for Exif {
    fn default() -> Self {
        Self { align: None }
    }
}

impl Exif {
    /// Parse the given data into a Exif structure
    /// * **Field**        | **Bytes** | **Description**
    /// * *Identifier*     | 6     | `4578 6966 0000` = `Exif` and 2 bytes of padding 0000
    /// * *Tiff header*    | 8     | `4949 2A00 0800 0000`, 2 bytes align `0x4949` is Little-Endian, `0x4D4D` is Big-Endian
    pub fn parse(input: &[u8]) -> Result<Exif, ExifError> {
        let mut exif = Exif::default();

        // Parse the Exif identifier and drop the results
        let (remain, _) = nom::sequence::terminated(
            nom::bytes::streaming::tag::<[u8; 4], &[u8], nom::error::Error<&[u8]>>(EXIF_IDENTIFIER),
            nom::bytes::streaming::tag::<[u8; 2], &[u8], nom::error::Error<&[u8]>>([0x00, 0x00]),
        )(input)
        .map_err(|x| ExifError::identifier_invalid().with_nom_source(x))?;

        // Parse the TIFF header byte alignment
        let (remain, align) = nom::branch::alt((
            nom::bytes::streaming::tag::<[u8; 2], &[u8], nom::error::Error<&[u8]>>(BIG_ENDIAN),
            nom::bytes::streaming::tag::<[u8; 2], &[u8], nom::error::Error<&[u8]>>(LITTLE_ENDIAN),
        ))(remain)
        .map_err(|x| ExifError::alignment_invalid().with_nom_source(x))?;
        exif.align = Some([align[0], align[1]]);

        // Parse the TIFF header length
        // 2A00 = 42 in BE
        let (remain, length) = if exif.is_big_endian() {
            nom::number::streaming::be_u16(remain)
                .map_err(|x| ExifError::length_invalid().with_nom_source(x))?
        } else {
            nom::number::streaming::le_u16(remain)
                .map_err(|x| ExifError::length_invalid().with_nom_source(x))?
        };
        // println!("length: {}", length);
        // return Err(ExifError::offset_failed());

        // // Drop the TIFF offset to get to the first IFD
        // // 00000008 ?
        // let (remain, val) = nom::bytes::streaming::take(4usize)(remain)
        //     .map_err(|x| ExifError::offset_failed().with_nom_source(x))?;
        // //return Err(ExifError::offset_failed().with_data(val));

        // // Parse IFD 1's file number
        // let (remain, length) = if exif.is_big_endian() {
        //     nom::number::streaming::be_u16(remain)
        //         .map_err(|x| ExifError::length_invalid().with_nom_source(x))?
        // } else {
        //     nom::number::streaming::le_u16(remain)
        //         .map_err(|x| ExifError::length_invalid().with_nom_source(x))?
        // };

        Ok(exif)
    }

    /// Check if the TIFF byte alignment is Big-Endian
    pub fn is_big_endian(&self) -> bool {
        self.align == Some(BIG_ENDIAN)
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
