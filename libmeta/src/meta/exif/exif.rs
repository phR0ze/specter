use nom::bytes::streaming as nom_bytes;
use nom::number::streaming as nom_nums;
use std::fmt::Display;

use super::{Tag, Endian, Ifd, BIG_ENDIAN, EXIF_IDENTIFIER, LITTLE_ENDIAN, TIFF_VERSION};
use crate::errors::{ExifError, ExifErrorKind};

/// Simplify the Exif return type slightly
pub type ExifResult<T> = Result<T, ExifError>;

#[derive(Debug)]
pub struct Exif {
    pub(crate) ifds: Vec<Ifd>,
}

impl Exif {
    /// Parse the given data into a Exif structure
    /// * **Field**        | **Bytes** | **Description**
    /// * *Identifier*     | 6     | `4578 6966 0000` = `Exif` and 2 bytes of padding 0000
    /// * *Tiff header*    | 8     | `4949 2A00 0800 0000`, 2 bytes align `0x4949` is Little-Endian, `0x4D4D` is Big-Endian
    pub(crate) fn parse(input: &[u8]) -> ExifResult<Exif> {
        let (exif_data, _) = parse_exif_header(input)?;

        // Parse TIFF alignment
        let (remain, endian) = parse_tiff_endian(exif_data)?;

        // Parse TIFF version
        let (remain, marker) = parse_tiff_version(remain, endian)?;
        if marker != TIFF_VERSION {
            return Err(ExifError::parse(": TIFF version invalid").with_data(&marker));
        }

        // Parse the IFDs
        let (_, ifds) = parse_ifds(exif_data, remain, endian)?;

        Ok(Self { ifds })
    }
}

impl Display for Exif {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let endian = match self.ifds.first() {
        //     Some(ifd) => ifd.endian,
        //     None => return Ok(()),
        // };
        // writeln!(f, "  {: <32}: {}", "Exif Byte Order".to_string(), endian)?;

        for ifd in &self.ifds {
            for field in &ifd.fields {
                // writeln!(f, "\n  {:?}", field)?;
                writeln!(f, "  {: <32}: {}", field.tag.to_string(), field.to_string())?;
            }
        }
        Ok(())
    }
}

/// Parse IFDs
/// * **input** is the full data source from tiff header alignment
/// * **remain** starts with the ifd offset
fn parse_ifds<'a>(
    input: &'a [u8],
    remain: &'a [u8],
    endian: Endian,
) -> ExifResult<(&'a [u8], Vec<Ifd>)> {
    let mut ifds: Vec<Ifd> = Vec::new();

    let mut outer = remain;
    loop {
        // Parse the IFD offset or end of IFDs
        let (inner, offset) = match parse_ifd_offset(outer, endian) {
            Ok((inner, offset)) => (inner, offset as usize),
            Err(e) => match e.kind() {
                ExifErrorKind::OffsetIsZero => break,
                _ => return Err(e),
            },
        };

        // Parse the IFD passing in the offset
        let (inner, ifd) = Ifd::parse(input, inner, endian, offset)?;
        ifds.push(ifd);

        // Parse Sub IFDs
        let ifd = ifds.last().unwrap();
        if let Some(field) = ifd.field_by_tag(Tag::ExifSubIfdOffset) {
            if let Some(offset) = field.to_unsigned() {
                // Don't need to track location as it is in an arbitrary location
                let (_, ifd) = Ifd::parse(input, inner, endian, offset as usize)?;
                ifds.push(ifd);
            }
        }

        // Track location
        outer = inner;
    }

    Ok((outer, ifds))
}

/// Parse out a 4 byte value as raw data
/// Returns: (remaining bytes, data bytes)
pub(crate) fn parse_ifd_data(input: &[u8]) -> ExifResult<(&[u8], &[u8])> {
    nom_bytes::take(4 as usize)(input)
        .map_err(|x| ExifError::parse(": IFD data").with_nom_source(x))
}

/// Parse out a 4 byte values as an offset
/// Returns: (remaining bytes, offset)
pub(crate) fn parse_ifd_offset(input: &[u8], endian: Endian) -> ExifResult<(&[u8], u32)> {
    let (remain, offset) = match endian {
        Endian::Big => nom_nums::be_u32(input),
        Endian::Little => nom_nums::le_u32(input),
    }
    .map_err(|x| ExifError::parse(": IFD offset").with_nom_source(x))?;

    // Used as a trigger to stop parsing IFDs
    if offset == 0 {
        return Err(ExifError::offset_zero());
    }

    Ok((remain, offset))
}

/// Parse the Exif header: 6 bytes `4578 6966 0000` => `Exif` and 2 bytes of padding 0000
fn parse_exif_header(input: &[u8]) -> ExifResult<(&[u8], [u8; 4])> {
    nom::sequence::terminated(
        nom::bytes::streaming::tag::<[u8; 4], &[u8], nom::error::Error<&[u8]>>(EXIF_IDENTIFIER),
        nom::bytes::streaming::tag::<[u8; 2], &[u8], nom::error::Error<&[u8]>>([0x00, 0x00]),
    )(input)
    .map(|(remain, x)| (remain, x.try_into().unwrap()))
    .map_err(|x| ExifError::parse(": Exif header").with_nom_source(x))
}

/// (2 bytes) Parse the TIFF header byte alignment
fn parse_tiff_endian(input: &[u8]) -> ExifResult<(&[u8], Endian)> {
    let (remain, alignment) = nom::branch::alt((
        nom::bytes::streaming::tag::<[u8; 2], &[u8], nom::error::Error<&[u8]>>(BIG_ENDIAN),
        nom::bytes::streaming::tag::<[u8; 2], &[u8], nom::error::Error<&[u8]>>(LITTLE_ENDIAN),
    ))(input)
    .map_err(|x| ExifError::parse(": TIFF endian").with_nom_source(x))?;

    // Convert to endian, has to be a valid value per nom above
    let array: [u8; 2] = alignment.try_into().unwrap();
    let endian: Endian = array.into();

    Ok((remain, endian))
}

/// (2 bytes) Parse the TIFF IFD 0 marker, always 2A00 or 0024
/// * Marker will always be returned in Big Endian format i.e. 0024
/// * Returns: (remaining bytes, marker)
fn parse_tiff_version(input: &[u8], endian: Endian) -> ExifResult<(&[u8], [u8; 2])> {
    let (remain, marker) = match endian {
        Endian::Big => nom::number::streaming::be_u16(input),
        Endian::Little => nom::number::streaming::le_u16(input),
    }
    .map_err(|e| ExifError::parse(": TIFF version").with_nom_source(e))?;

    Ok((remain, marker.to_be_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::meta::exif::{format, tag};
    use crate::test_data::EXIF_TEST_DATA;
    use crate::{container::JPEG_TEST_DATA, errors::BaseError};

    const EXIF_HEADER: [u8; 6] = [0x45, 0x78, 0x69, 0x66, 0x00, 0x00];

    #[test]
    fn test_parse() {
        let exif = Exif::parse(&JPEG_TEST_DATA[24..]).unwrap();
        assert_eq!(exif.ifds.len(), 3);
    }

    #[test]
    fn test_parse_ifds() {
        let (_, ifds) = parse_ifds(&EXIF_TEST_DATA, &EXIF_TEST_DATA[4..], Endian::Big).unwrap();
        assert_eq!(ifds.len(), 3);

        // IFD 0 spot check
        let field = &ifds[0].fields[3];
        assert_eq!(field.endian, Endian::Big);
        assert_eq!(field.tag, Tag::ResolutionUnit);
        assert_eq!(field.format, format::UNSIGNED_SHORT);
        assert_eq!(field.components, 1);
        assert_eq!(field.offset, None);
        assert_eq!(field.length(), 2);
        assert_eq!(field.data, Some(vec![0x00, 0x02, 0x00, 0x00]));
        assert_eq!(field.to_unsigned(), Some(2));

        // IFD 1 spot check
        let field = &ifds[1].fields[1];
        assert_eq!(field.tag, Tag::ExifImageWidth);
        assert_eq!(field.format, format::UNSIGNED_SHORT);
        assert_eq!(field.components, 1);
        assert_eq!(field.offset, None);
        assert_eq!(field.data, Some(vec![0x00, 0x0f, 0x00, 0x00]));
        assert_eq!(field.to_unsigned(), Some(15));

        // IFD 2 spot check
        let field = &ifds[2].fields[1];
        assert_eq!(field.tag, Tag::ThumbnailLength);
        assert_eq!(field.format, format::UNSIGNED_LONG);
        assert_eq!(field.components, 1);
        assert_eq!(field.offset, None);
        assert_eq!(field.data, Some(vec![0x00, 0x00, 0x02, 0x88]));
        assert_eq!(field.to_unsigned(), Some(648));
    }

    #[test]
    fn test_parse_ifd_offset_not_enough_data() {
        let err = parse_ifd_offset(&[0xFF], Endian::Big).unwrap_err();
        assert_eq!(err.to_string(), "Exif parse failed: IFD offset");
        assert_eq!(err.source_to_string(), "nom::Parsing requires 3 bytes/chars");
    }

    #[test]
    fn test_parse_ifd_offset_little_endian() {
        let input = [0x24, 0x00, 0x00, 0x00];
        let (remain, offset) = parse_ifd_offset(&input, Endian::Little).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(offset, 0x24);
    }

    #[test]
    fn test_parse_ifd_offset_big_endian() {
        let input = [0x00, 0x00, 0x00, 0x24];
        let (remain, data) = parse_ifd_data(&input).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(data, [0x00, 0x00, 0x00, 0x24]);
    }

    #[test]
    fn test_parse_tiff_version_not_enough_data() {
        let err = parse_tiff_version(&[0xFF], Endian::Big).unwrap_err();
        assert_eq!(err.to_string(), "Exif parse failed: TIFF version");
        assert_eq!(err.source_to_string(), "nom::Parsing requires 1 bytes/chars");
    }

    #[test]
    fn test_parse_tiff_version_little_endian() {
        let (remain, marker) = parse_tiff_version(&[0x2A, 0x00], Endian::Little).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(marker, TIFF_VERSION);
    }

    #[test]
    fn test_parse_tiff_version_big_endian() {
        let (remain, marker) = parse_tiff_version(&[0x00, 0x2A], Endian::Big).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(marker, TIFF_VERSION);
    }

    #[test]
    fn test_parse_tiff_alignemnt_unknown() {
        let err = parse_tiff_endian(&[0xFF, 0xFF]).unwrap_err();
        assert_eq!(err.to_string(), "Exif parse failed: TIFF endian");
        assert_eq!(
            err.source_to_string(),
            "nom::Parsing Error: Error { input: [255, 255], code: Tag }"
        );
    }

    #[test]
    fn test_parse_tiff_alignemnt_big_endian() {
        let (remain, endian) = parse_tiff_endian(&BIG_ENDIAN).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(endian, Endian::Big);
    }

    #[test]
    fn test_parse_tiff_alignemnt_little_endian() {
        let (remain, endian) = parse_tiff_endian(&LITTLE_ENDIAN).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(endian, Endian::Little);
    }

    #[test]
    fn test_parse_exif_header_not_enough_data() {
        let err = parse_exif_header(&[]).unwrap_err();
        assert_eq!(
            err.all_to_string(),
            "Exif parse failed: Exif header ==> nom::Parsing requires 4 bytes/chars"
        );
    }

    #[test]
    fn test_parse_exif_header() {
        let (remain, id) = parse_exif_header(&EXIF_HEADER).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(id, EXIF_HEADER[0..4]);
    }
}
