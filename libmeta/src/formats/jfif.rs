use nom::{bytes::streaming as nom_bytes, error::Error as NomError, number::streaming as nom_nums};

use crate::errors::JpegError;

const JFIF_IDENTIFIER: [u8; 4] = [0x4A, 0x46, 0x49, 0x46];

#[derive(Debug, Clone, PartialEq)]
pub enum DensityUnit {
    PixelsPerInch,
    PixelsPerCm,
    None,
    Unknown,
}

impl From<u8> for DensityUnit {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::None,
            0x01 => Self::PixelsPerInch,
            0x02 => Self::PixelsPerCm,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Jfif {
    pub major: u8,                  // major version
    pub minor: u8,                  // minor version
    pub density: DensityUnit,       // density unit
    pub x_density: u16,             // horizontal pixel density
    pub y_density: u16,             // vertical pixel density
    pub x_thumbnail: u8,            // horizontal pixels of the embedded RGB thumbnail
    pub y_thumbnail: u8,            // vertical pixels of the embedded RGB thumbnail
    pub thumbnail: Option<Vec<u8>>, // uncompressed 24 bit RGB raster thumbnail
}

impl Default for Jfif {
    fn default() -> Self {
        Self {
            major: 0,
            minor: 0,
            density: DensityUnit::None,
            x_density: 0,
            y_density: 0,
            x_thumbnail: 0,
            y_thumbnail: 0,
            thumbnail: None,
        }
    }
}

impl Jfif {
    /// Parse the given data into a JFIF structure
    /// * **Field**        | **Bytes** | **Description**
    /// * *Identifier*     | 5     | `0x4a 0x46 0x49 0x46 0x00` = `JFIF` in ASCII terminated by a null byte
    /// * *JFIF version*   | 2     | `0x01 0x02` is the major and minor JFIF version i.e. `1.02`
    /// * *Density Units*  | 1     | `0x00` = None, `0x01` = pixels per inch, `0x02` = pixels per centimeter
    /// * *Xdensity*       | 2     | `0x00 0x48` = `72` Horizontal pixel density, Must not be zero
    /// * *Ydensity*       | 2     | `0x00 0x48` = `72` Vertical pixel density, Must not be zero
    /// * *Xthumbnail*     | 1     | `0x00` Horizontal pixels of the embedded RGB thumbnail, May be zero
    /// * *Ythumbnail*     | 1     | `0x00` Vertical pixels of the embedded RGB thumbnail, May be zero
    /// * *Thumbnail data* | 3 x n | Uncompressed 24 bit RGB (8 bits per color channel) raster thumbnail
    pub fn parse(input: &[u8]) -> Result<Jfif, JpegError> {
        let mut jfif = Jfif::default();

        // Parse the JFIF identifier and drop the results
        let (remain, _) = nom::sequence::terminated(
            nom_bytes::tag::<[u8; 4], &[u8], NomError<&[u8]>>(JFIF_IDENTIFIER),
            nom_bytes::tag::<[u8; 1], &[u8], NomError<&[u8]>>([0x00]),
        )(input)
        .map_err(|x| JpegError::jfif_identifier_invalid().with_nom_source(x))?;

        // Parse the JFIF version
        let (remain, (major, minor)) =
            nom::sequence::tuple((nom_nums::u8, nom_nums::u8))(remain)
                .map_err(|x| JpegError::jfif_version_invalid().with_nom_source(x))?;
        jfif.major = major;
        jfif.minor = minor;

        // Parse the JFIF density units
        let (remain, (density, xdensity, ydensity)) =
            nom::sequence::tuple((nom_nums::u8, nom_nums::be_u16, nom_nums::be_u16))(remain)
                .map_err(|x| JpegError::jfif_density_units_invalid().with_nom_source(x))?;
        jfif.density = density.into();
        if jfif.density == DensityUnit::Unknown {
            return Err(JpegError::jfif_density_units_unknown().with_data(&[density]));
        }
        jfif.x_density = xdensity;
        jfif.y_density = ydensity;

        // Parse the JFIF thumbnail dimensions
        let (remain, (x_thumbnail, y_thumbnail)) =
            nom::sequence::tuple((nom_nums::u8, nom_nums::u8))(remain).map_err(|x| {
                JpegError::jfif_thumbnail_dimensions_invalid().with_nom_source(x)
            })?;
        jfif.x_thumbnail = x_thumbnail;
        jfif.y_thumbnail = y_thumbnail;

        // Check if a thumbnail was included
        if x_thumbnail != 0 && y_thumbnail != 0 {
            Err(JpegError::jfif_thumbnail_invalid())?;
        }

        Ok(jfif)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const JFIF_DATA_1: [u8; 18] = [
        0xff, 0xe0, 0x00, 0x10, 0x4a, 0x46, 0x49, 0x46, 0x00, 0x01, 0x02, 0x01, 0x00, 0x48, 0x00,
        0x48, 0x00, 0x00,
    ];

    #[test]
    fn test_parse_jfif_success() {
        let jfif = Jfif::parse(&JFIF_DATA_1[4..]).unwrap();
        assert_eq!(jfif.major, 1);
        assert_eq!(jfif.minor, 2);
        assert_eq!(jfif.density, DensityUnit::PixelsPerInch);
        assert_eq!(jfif.x_density, 72);
        assert_eq!(jfif.y_density, 72);
        assert_eq!(jfif.x_thumbnail, 0);
        assert_eq!(jfif.y_thumbnail, 0);
    }
}
