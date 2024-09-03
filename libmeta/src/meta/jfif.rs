use nom::number::streaming as nom_nums;

use crate::errors::JfifError;

const JFIF_IDENTIFIER: [u8; 4] = [0x4A, 0x46, 0x49, 0x46];

/// Jfif Density Units
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
    pub(crate) major: u8,                  // major version
    pub(crate) minor: u8,                  // minor version
    pub(crate) density: DensityUnit,       // density unit
    pub(crate) x_density: u16,             // horizontal pixel density
    pub(crate) y_density: u16,             // vertical pixel density
    pub(crate) x_dimension: u8,            // horizontal pixels of the embedded RGB thumbnail
    pub(crate) y_dimension: u8,            // vertical pixels of the embedded RGB thumbnail
    pub(crate) thumbnail: Option<Vec<u8>>, // uncompressed 24 bit RGB raster thumbnail
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
    pub(crate) fn parse(input: &[u8]) -> Result<Jfif, JfifError> {
        let (remain, _) = parse_header(input)?;
        let (remain, major, minor) = parse_version(remain)?;

        let (remain, density, x_density, y_density) = parse_density(remain)?;

        let (remain, x_dimension, y_dimension) = parse_thumbnail_dimensions(remain)?;

        // Check if a thumbnail was included
        if x_dimension != 0 && y_dimension != 0 {
            // TODO: Parse the thumbnail data
            Err(JfifError::parse(": thumbnail invalid").with_data(remain))?;
        }

        Ok(Self {
            major: major,
            minor: minor,
            density: density,
            x_density: x_density,
            y_density: y_density,
            x_dimension: x_dimension,
            y_dimension: y_dimension,
            thumbnail: None,
        })
    }
}

// Parse the JFIF identifier
fn parse_header(input: &[u8]) -> Result<(&[u8], [u8; 4]), JfifError> {
    let (remain, id) = nom::sequence::terminated(
        nom::bytes::streaming::tag::<[u8; 4], &[u8], nom::error::Error<&[u8]>>(JFIF_IDENTIFIER),
        nom::bytes::streaming::tag::<[u8; 1], &[u8], nom::error::Error<&[u8]>>([0x00]),
    )(input)
    .map_err(|x| JfifError::parse(": identifier invalid").with_nom_source(x))?;

    Ok((remain, id.try_into().unwrap()))
}

// Parse the JFIF version
fn parse_version(input: &[u8]) -> Result<(&[u8], u8, u8), JfifError> {
    let (remain, (major, minor)) = nom::sequence::tuple((nom_nums::u8, nom_nums::u8))(input)
        .map_err(|x| JfifError::parse(": version invalid").with_nom_source(x))?;
    Ok((remain, major, minor))
}

// Parse the JFIF density units
fn parse_density(input: &[u8]) -> Result<(&[u8], DensityUnit, u16, u16), JfifError> {
    let (remain, (density_data, x_density, y_density)) =
        nom::sequence::tuple((nom_nums::u8, nom_nums::be_u16, nom_nums::be_u16))(input)
            .map_err(|x| JfifError::parse(": density units invalid").with_nom_source(x))?;
    let density: DensityUnit = density_data.into();
    if density == DensityUnit::Unknown {
        return Err(JfifError::parse(": density units unknown").with_data(&[density_data]));
    };
    Ok((remain, density, x_density, y_density))
}

// Parse the JFIF thumbnail dimensions
fn parse_thumbnail_dimensions(input: &[u8]) -> Result<(&[u8], u8, u8), JfifError> {
    let (remain, (x_thumbnail, y_thumbnail)) =
        nom::sequence::tuple((nom_nums::u8, nom_nums::u8))(input)
            .map_err(|x| JfifError::parse(": thumbnail dimensions invalid").with_nom_source(x))?;
    Ok((remain, x_thumbnail, y_thumbnail))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::BaseError;

    const JFIF_DATA_1: [u8; 18] = [
        0xff, 0xe0, // marker
        0x00, 0x10, // size
        0x4a, 0x46, 0x49, 0x46, 0x00, // header
        0x01, // major version
        0x02, // minor version
        0x01, // density units
        0x00, 0x48, // x density
        0x00, 0x48, // y density
        0x00, 0x00,
    ];

    #[test]
    fn test_parse_jfif_success() {
        let jfif = Jfif::parse(&JFIF_DATA_1[4..]).unwrap();
        assert_eq!(jfif.major, 1);
        assert_eq!(jfif.minor, 2);
        assert_eq!(jfif.density, DensityUnit::PixelsPerInch);
        assert_eq!(jfif.x_density, 72);
        assert_eq!(jfif.y_density, 72);
        assert_eq!(jfif.x_dimension, 0);
        assert_eq!(jfif.y_dimension, 0);
    }

    #[test]
    fn test_parse_jfif_thumbnail_dimensions_not_enough_data() {
        let err = parse_thumbnail_dimensions(&[]).unwrap_err();
        assert_eq!(
            err.all_to_string(),
            "JFIF parse failed: thumbnail dimensions invalid ==> nom::Parsing requires 1 bytes/chars"
        );
    }

    #[test]
    fn test_parse_jfif_thumbnail_dimensions() {
        let (remain, xdimension, ydimension) =
            parse_thumbnail_dimensions(&JFIF_DATA_1[16..]).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(xdimension, 0);
        assert_eq!(ydimension, 0);
    }

    #[test]
    fn test_parse_jfif_density_not_enough_data() {
        let err = parse_density(&[]).unwrap_err();
        assert_eq!(
            err.all_to_string(),
            "JFIF parse failed: density units invalid ==> nom::Parsing requires 1 bytes/chars"
        );
    }

    #[test]
    fn test_parse_jfif_density() {
        let (remain, density, xdensity, ydensity) = parse_density(&JFIF_DATA_1[11..]).unwrap();
        assert_eq!(remain, &JFIF_DATA_1[16..]);
        assert_eq!(density, DensityUnit::PixelsPerInch);
        assert_eq!(xdensity, 72);
        assert_eq!(ydensity, 72);
    }

    #[test]
    fn test_parse_jfif_version_not_enough_data() {
        let err = parse_version(&[]).unwrap_err();
        assert_eq!(
            err.all_to_string(),
            "JFIF parse failed: version invalid ==> nom::Parsing requires 1 bytes/chars"
        );
    }

    #[test]
    fn test_parse_jfif_version() {
        let (remain, major, minor) = parse_version(&JFIF_DATA_1[9..]).unwrap();
        assert_eq!(remain, &JFIF_DATA_1[11..]);
        assert_eq!(major, 1);
        assert_eq!(minor, 2);
    }

    #[test]
    fn test_parse_jfif_header_not_enough_data() {
        let err = parse_header(&[]).unwrap_err();
        assert_eq!(
            err.all_to_string(),
            "JFIF parse failed: identifier invalid ==> nom::Parsing requires 4 bytes/chars"
        );
    }

    #[test]
    fn test_parse_jfif_header() {
        let (remain, id) = parse_header(&JFIF_DATA_1[4..]).unwrap();
        assert_eq!(remain, &JFIF_DATA_1[9..]);
        assert_eq!(id, JFIF_DATA_1[4..8]);
    }
}
