use nom::number::streaming as nom_nums;

use crate::errors::ExifError;

const EXIF_IDENTIFIER: [u8; 4] = [0x45, 0x78, 0x69, 0x66];
const BIG_ENDIAN: [u8; 2] = [0x4D, 0x4D];
const LITTLE_ENDIAN: [u8; 2] = [0x49, 0x49];

#[derive(Debug, Clone)]
pub struct Exif {
    alignment: Option<[u8; 2]>,
}

impl Default for Exif {
    fn default() -> Self {
        Self { alignment: None }
    }
}

impl Exif {
    /// Check if the TIFF byte alignment is Big-Endian
    pub fn is_big_endian(&self) -> bool {
        self.alignment == Some(BIG_ENDIAN)
    }
}

/// Parse the given data into a Exif structure
/// * **Field**        | **Bytes** | **Description**
/// * *Identifier*     | 6     | `4578 6966 0000` = `Exif` and 2 bytes of padding 0000
/// * *Tiff header*    | 8     | `4949 2A00 0800 0000`, 2 bytes align `0x4949` is Little-Endian, `0x4D4D` is Big-Endian
pub fn parse(input: &[u8]) -> Result<Exif, ExifError> {
    let mut exif = Exif::default();
    let (remain, _) = parse_exif_header(input)?;

    // Set alignment
    let (remain, alignment) = parse_tiff_alignment(remain)?;
    let big_endian = alignment == BIG_ENDIAN;
    exif.alignment = Some(alignment);

    // Parse IFD 0 marker
    let (remain, marker) = parse_tiff_ifd_marker(remain, big_endian)?;
    if marker != 0x24 {
        return Err(ExifError::marker_invalid());
    }

    // Parse IFD 0 start offset e.g. 00 00 00 08 and then consume any bytes to get to the
    // correct offset. This will allmost always be 0.
    let (remain, offset) = parse_tiff_ifd_offset(remain, big_endian)?;
    nom::bytes::streaming::take(offset - 8)(remain)
        .map_err(|x| ExifError::offset_failed().with_nom_source(x))?;

    // (2 bytes) Parse number of file entries in the IFD
    let (remain, count) = parse_tiff_ifd_entries_count(input, big_endian)?;

    // count * 12 bytes for IFD entry headers
    let (remain, entry_headers) =
        nom::multi::count(nom::bytes::streaming::take(12usize), count as usize)(remain)
            .map_err(|x| ExifError::entry_header_failed().with_nom_source(x))?;

    // Parse file headers in the IFD, 12 bytes each
    // e.g. TT TT ff ff NN NN NN NN DD DD DD DD
    // * 2 byte Tag number
    // * 2 byte Type of data
    // * 4 byte Number of components
    // * 4 byte Offset to data value
    return Err(ExifError::count_invalid().with_str(count));

    // Parse type of data format

    Ok(exif)
}
/// Parse the Exif header: 6 bytes `4578 6966 0000` => `Exif` and 2 bytes of padding 0000
fn parse_exif_header(input: &[u8]) -> Result<(&[u8], [u8; 4]), ExifError> {
    nom::sequence::terminated(
        nom::bytes::streaming::tag::<[u8; 4], &[u8], nom::error::Error<&[u8]>>(EXIF_IDENTIFIER),
        nom::bytes::streaming::tag::<[u8; 2], &[u8], nom::error::Error<&[u8]>>([0x00, 0x00]),
    )(input)
    .map(|(remain, x)| (remain, x.try_into().unwrap()))
    .map_err(|x| ExifError::identifier_invalid().with_nom_source(x))
}

// (2 bytes) Parse the TIFF header byte alignment
fn parse_tiff_alignment(input: &[u8]) -> Result<(&[u8], [u8; 2]), ExifError> {
    nom::branch::alt((
        nom::bytes::streaming::tag::<[u8; 2], &[u8], nom::error::Error<&[u8]>>(BIG_ENDIAN),
        nom::bytes::streaming::tag::<[u8; 2], &[u8], nom::error::Error<&[u8]>>(LITTLE_ENDIAN),
    ))(input)
    .map(|(remain, x)| (remain, x.try_into().unwrap()))
    .map_err(|x| ExifError::alignment_invalid().with_nom_source(x))
}

// (2 bytes) Parse the TIFF IFD 0 marker, always 2A00 or 0024
fn parse_tiff_ifd_marker(input: &[u8], big_endian: bool) -> Result<(&[u8], u16), ExifError> {
    match big_endian {
        true => nom::number::streaming::be_u16(input),
        false => nom::number::streaming::le_u16(input),
    }
    .map_err(|x| ExifError::marker_invalid().with_nom_source(x))
}

// (4 bytes) Parse IFD 0 start offset e.g. 00 00 00 08
// i.e. 8 bytes from the start of the TIFF header including 2 alignment bytes.
fn parse_tiff_ifd_offset(input: &[u8], big_endian: bool) -> Result<(&[u8], u32), ExifError> {
    match big_endian {
        true => nom_nums::be_u32(input),
        false => nom_nums::le_u32(input),
    }
    .map_err(|x| ExifError::offset_failed().with_nom_source(x))
}

// (2 bytes) Parse number of file entries in the IFD
fn parse_tiff_ifd_entries_count(input: &[u8], big_endian: bool) -> Result<(&[u8], u16), ExifError> {
    match big_endian {
        true => nom_nums::be_u16(input),
        false => nom_nums::le_u16(input),
    }
    .map_err(|x| ExifError::count_invalid().with_nom_source(x))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::BaseError;

    const EXIF_HEADER: [u8; 6] = [0x45, 0x78, 0x69, 0x66, 0x00, 0x00];

    #[test]
    fn test_parse_tiff_ifd_entries_count_not_enough_data() {
        let err = parse_tiff_ifd_entries_count(&[0xFF], true).unwrap_err();
        assert_eq!(err.to_string(), "Exif IFD entries count invalid");
        assert_eq!(
            err.source_to_string(),
            "nom::Parsing requires 1 bytes/chars"
        );
    }

    #[test]
    fn test_parse_tiff_ifd_entries_count_little_endian() {
        let (remain, marker) = parse_tiff_ifd_entries_count(&[0x01, 0x00], false).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(marker, 0x1);
    }

    #[test]
    fn test_parse_tiff_ifd_entries_count_big_endian() {
        let (remain, marker) = parse_tiff_ifd_entries_count(&[0x00, 0x01], true).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(marker, 0x1);
    }

    #[test]
    fn test_parse_tiff_ifd_offset_not_enough_data() {
        let err = parse_tiff_ifd_offset(&[0xFF], true).unwrap_err();
        assert_eq!(err.to_string(), "Exif IFD offset failed");
        assert_eq!(
            err.source_to_string(),
            "nom::Parsing requires 3 bytes/chars"
        );
    }

    #[test]
    fn test_parse_tiff_ifd_offset_little_endian() {
        let (remain, marker) = parse_tiff_ifd_offset(&[0x24, 0x00, 0x00, 0x00], false).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(marker, 0x24);
    }

    #[test]
    fn test_parse_tiff_ifd_offset_big_endian() {
        let (remain, marker) = parse_tiff_ifd_offset(&[0x00, 0x00, 0x00, 0x24], true).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(marker, 0x24);
    }

    #[test]
    fn test_parse_tiff_ifd_marker_not_enough_data() {
        let err = parse_tiff_ifd_marker(&[0xFF], true).unwrap_err();
        assert_eq!(err.to_string(), "Exif IFD marker invalid");
        assert_eq!(
            err.source_to_string(),
            "nom::Parsing requires 1 bytes/chars"
        );
    }

    #[test]
    fn test_parse_tiff_ifd_marker_little_endian() {
        let (remain, marker) = parse_tiff_ifd_marker(&[0x24, 0x00], false).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(marker, 0x24);
    }

    #[test]
    fn test_parse_tiff_ifd_marker_big_endian() {
        let (remain, marker) = parse_tiff_ifd_marker(&[0x00, 0x24], true).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(marker, 0x24);
    }

    #[test]
    fn test_parse_tiff_alignemnt_unknown() {
        let err = parse_tiff_alignment(&[0xFF, 0xFF]).unwrap_err();
        assert_eq!(err.to_string(), "Exif TIFF alignment invalid");
        assert_eq!(
            err.source_to_string(),
            "nom::Parsing Error: Error { input: [255, 255], code: Tag }"
        );
    }

    #[test]
    fn test_parse_tiff_alignemnt_big_endian() {
        let (remain, endian) = parse_tiff_alignment(&BIG_ENDIAN).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(endian, BIG_ENDIAN);
    }

    #[test]
    fn test_parse_tiff_alignemnt_little_endian() {
        let (remain, endian) = parse_tiff_alignment(&LITTLE_ENDIAN).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(endian, LITTLE_ENDIAN);
    }

    #[test]
    fn test_parse_exif_header_not_enough_data() {
        let err = parse_exif_header(&[]).unwrap_err();
        assert_eq!(
            err.all_to_string(),
            "Exif identifier invalid ==> nom::Parsing requires 4 bytes/chars"
        );
    }

    #[test]
    fn test_parse_exif_header() {
        let (remain, id) = parse_exif_header(&EXIF_HEADER).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(id, EXIF_HEADER[0..4]);
    }
}
