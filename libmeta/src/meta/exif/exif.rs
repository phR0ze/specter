use core::panic;

use nom::bytes::streaming as nom_bytes;
use nom::number::streaming as nom_nums;

use super::{Ifd, IfdFile, BIG_ENDIAN, EXIF_IDENTIFIER, LITTLE_ENDIAN};
use crate::errors::ExifError;

/// Simplify the Exif return type slightly
pub type ExifResult<T> = Result<T, ExifError>;

/// Track the endianness of the TIFF data
#[derive(Debug, Clone, PartialEq, Copy)]
enum Endian {
    Big,
    Little,
}

impl From<[u8; 2]> for Endian {
    fn from(data: [u8; 2]) -> Self {
        match data {
            BIG_ENDIAN => Endian::Big,
            LITTLE_ENDIAN => Endian::Little,
            _ => panic!("Invalid TIFF alignment"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Exif {}

impl Default for Exif {
    fn default() -> Self {
        Self {}
    }
}

impl Exif {}

/// Parse the given data into a Exif structure
/// * **Field**        | **Bytes** | **Description**
/// * *Identifier*     | 6     | `4578 6966 0000` = `Exif` and 2 bytes of padding 0000
/// * *Tiff header*    | 8     | `4949 2A00 0800 0000`, 2 bytes align `0x4949` is Little-Endian, `0x4D4D` is Big-Endian
pub fn parse(input: &[u8]) -> ExifResult<Exif> {
    let mut exif = Exif::default();
    let (remain, _) = parse_exif_header(input)?;

    // Parse alignment
    let (remain, endian) = parse_tiff_endian(remain)?;

    // Parse IFD 0 marker
    let (remain, marker) = parse_ifd_marker(remain, endian)?;
    if marker != 0x24 {
        return Err(ExifError::marker_invalid());
    }

    // Parser offset to the next IFD: 4 bytes

    Ok(exif)
}

/// Parse the Exif header: 6 bytes `4578 6966 0000` => `Exif` and 2 bytes of padding 0000
fn parse_exif_header(input: &[u8]) -> ExifResult<(&[u8], [u8; 4])> {
    nom::sequence::terminated(
        nom::bytes::streaming::tag::<[u8; 4], &[u8], nom::error::Error<&[u8]>>(EXIF_IDENTIFIER),
        nom::bytes::streaming::tag::<[u8; 2], &[u8], nom::error::Error<&[u8]>>([0x00, 0x00]),
    )(input)
    .map(|(remain, x)| (remain, x.try_into().unwrap()))
    .map_err(|x| ExifError::identifier_invalid().with_nom_source(x))
}

/// (2 bytes) Parse the TIFF header byte alignment
fn parse_tiff_endian(input: &[u8]) -> ExifResult<(&[u8], Endian)> {
    let (remain, alignment) = nom::branch::alt((
        nom::bytes::streaming::tag::<[u8; 2], &[u8], nom::error::Error<&[u8]>>(BIG_ENDIAN),
        nom::bytes::streaming::tag::<[u8; 2], &[u8], nom::error::Error<&[u8]>>(LITTLE_ENDIAN),
    ))(input)
    .map_err(|x| ExifError::endian_invalid().with_nom_source(x))?;

    // Convert to endian, has to be a valid value per nom above
    let array: [u8; 2] = alignment.try_into().unwrap();
    let endian: Endian = array.into();

    Ok((remain, endian))
}

/// (2 bytes) Parse the TIFF IFD 0 marker, always 2A00 or 0024
fn parse_ifd_marker(input: &[u8], endian: Endian) -> ExifResult<(&[u8], u16)> {
    match endian {
        Endian::Big => nom::number::streaming::be_u16(input),
        Endian::Little => nom::number::streaming::le_u16(input),
    }
    .map_err(|x| ExifError::marker_invalid().with_nom_source(x))
}

/// Parse out a 4 byte values as either raw data bytes in big endian or an offset
/// Returns: (remaining bytes, data bytes, offset)
fn parse_ifd_data_or_offset(input: &[u8], endian: Endian) -> ExifResult<(&[u8], [u8; 4], u32)> {
    let (remain, offset) = match endian {
        Endian::Big => nom_nums::be_u32(input),
        Endian::Little => nom_nums::le_u32(input),
    }
    .map_err(|x| ExifError::offset_failed().with_nom_source(x))?;

    // Validate the offset
    if offset == 0 {
        return Err(ExifError::offset_failed().with_msg("is zero"));
    }

    Ok((remain, offset.to_be_bytes(), offset))
}

/// (2 bytes) Parse number of file entries in the IFD
fn parse_ifd_file_count(input: &[u8], endian: Endian) -> ExifResult<(&[u8], u16)> {
    match endian {
        Endian::Big => nom_nums::be_u16(input),
        Endian::Little => nom_nums::le_u16(input),
    }
    .map_err(|x| ExifError::count_invalid().with_nom_source(x))
}

/// Parse IFD file, 12 byte header and arbitrary data
/// e.g. TT TT ff ff NN NN NN NN DD DD DD DD
/// * 2 byte Tag number
/// * 2 byte Data format
/// * 4 byte Number of components
/// * 4 byte Offset to data value or data itself
/// * **input** is the full data source from tiff header alignment
/// * **remain** is where the header starts
/// * Returns: (remaining bytes, IFD file)
fn parse_ifd_file<'a>(
    input: &'a [u8],
    remain: &'a [u8],
    endian: Endian,
) -> ExifResult<(&'a [u8], IfdFile)> {
    // tag: 2 bytes
    let (remain, tag) = match endian {
        Endian::Big => nom_nums::be_u16(remain),
        Endian::Little => nom_nums::le_u16(remain),
    }
    .map_err(|x| ExifError::ifd_file_tag_failed().with_nom_source(x))?;

    // data format: 2 bytes
    let (remain, format) = match endian {
        Endian::Big => nom_nums::be_u16(remain),
        Endian::Little => nom_nums::le_u16(remain),
    }
    .map_err(|x| ExifError::ifd_file_data_format_failed().with_nom_source(x))?;

    // number of components: 4 bytes
    let (remain, components) = match endian {
        Endian::Big => nom_nums::be_u32(remain),
        Endian::Little => nom_nums::le_u32(remain),
    }
    .map_err(|x| ExifError::ifd_file_component_count_failed().with_nom_source(x))?;

    // offset to data value: 4 bytes
    let (remain, data, offset) = parse_ifd_data_or_offset(remain, endian)?;

    // create the ifd file and calculate if there is an offset to extract data from
    let mut file = IfdFile::new(tag, format, components);
    if file.data_length() > 4 {
        let remain = remain; // save the current position by creating a new variable

        // skip to the offset location
        let consumed = input.len() - remain.len();
        if consumed > offset as usize {
            return Err(ExifError::offset_failed().with_msg("is negative"));
        }
        let remain = if offset as usize - 1 > consumed {
            let (remain, _) = nom_bytes::take(offset as usize - 1 - consumed)(remain)
                .map_err(|x| ExifError::offset_failed().with_nom_source(x))?;
            remain
        } else {
            remain
        };

        // read the data from the offset location
        let (_, data) = nom_bytes::take(file.data_length())(remain)
            .map_err(|x| ExifError::offset_failed().with_nom_source(x))?;

        // Update the file with the correct offset and data
        file.offset = Some(offset);
        file.data = Some(data.to_vec());
    } else {
        file.data = Some(data.to_vec());
    }

    Ok((remain, file))
}

/// Parse IFD
/// * **input** is the full data source from tiff header alignment
/// * **remain** starts with the ifd offset
fn parse_ifd<'a>(input: &'a [u8], remain: &'a [u8], endian: Endian) -> ExifResult<(&'a [u8], Ifd)> {
    let mut ifd = Ifd::default();

    // read the ifd offset and skip to the ifd offset location
    let (remain, _, offset) = parse_ifd_data_or_offset(remain, endian)?;
    let (remain, _) = nom_bytes::take(offset as usize - (input.len() - remain.len()))(remain)
        .map_err(|x| ExifError::offset_failed().with_nom_source(x))?;

    // Parse out the number of IFD files to expect
    let (remain, count) = parse_ifd_file_count(remain, endian)?;

    // Parse out each of the IFD files
    let mut remain = remain;
    for _ in 0..count {
        let (r, f) = parse_ifd_file(input, remain, endian)?;
        remain = r;
        ifd.files.push(f);
    }

    Ok((remain, ifd))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{errors::BaseError, exif::LITTLE_ENDIAN};

    #[test]
    fn test_parse_ifd_files_little_endian() {
        let (remain, ifd) = parse_ifd(&IFD_LE, &IFD_LE[4..], Endian::Little).unwrap();
        assert_eq!(remain, &IFD_LE[34..]);

        let file = &ifd.files[0];
        assert_eq!(file.tag, 282);
        assert_eq!(file.format, 5);
        assert_eq!(file.components, 1);
        assert_eq!(file.offset, Some(35));
        assert_eq!(file.data_length(), 8);
        assert_eq!(file.data, Some(Vec::from(&IFD_LE[34..])));
    }

    const IFD_LE: [u8; 42] = [
        /* 00-01 */ 0x49, 0x49, // alignment, little endian
        /* 02-04 */ 0x2A, 0x00, // ifd marker
        /* 05-08 */ 0x08, 0x00, 0x00, 0x00, // ifd start
        /* 09-10 */ 0x02, 0x00, // file count
        /* 11-12 */ 0x1A, 0x01, // tag: 0x011A, XResolution
        /* 13-14 */ 0x05, 0x00, // data format: 5, unsigned rational
        /* 15-18 */ 0x01, 0x00, 0x00, 0x00, // components: 1, so data 8 bytes
        /* 19-22 */ 0x23, 0x00, 0x00, 0x00, // offset of 35
        /* 23-24 */ 0x69, 0x87, // tag:
        /* 25-26 */ 0x04, 0x00, // data format: 4, unsigned long
        /* 27-30 */ 0x01, 0x00, 0x00, 0x00, // components: 1
        /* 31-34 */ 0x2B, 0x00, 0x00, 0x00, // data for file 2
        /* 35-42 */ 0x48, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // data for file 1
    ];

    const EXIF_HEADER: [u8; 6] = [0x45, 0x78, 0x69, 0x66, 0x00, 0x00];

    #[test]
    fn test_parse_ifd_files_little_endian() {
        let (remain, ifd) = parse_ifd(&IFD_LE, &IFD_LE[4..], Endian::Little).unwrap();
        assert_eq!(remain, &IFD_LE[34..]);

        let file = &ifd.files[0];
        assert_eq!(file.tag, 282);
        assert_eq!(file.format, 5);
        assert_eq!(file.components, 1);
        assert_eq!(file.offset, Some(35));
        assert_eq!(file.data_length(), 8);
        assert_eq!(file.data, Some(Vec::from(&IFD_LE[34..])));

        let file = &ifd.files[1];
        assert_eq!(file.tag, 34665);
        assert_eq!(file.format, 4);
        assert_eq!(file.components, 1);
        assert_eq!(file.offset, None);
        assert_eq!(file.data_length(), 4);
        assert_eq!(file.data, Some(Vec::from(&[0x00, 0x00, 0x00, 0x2B])));
    }

    #[test]
    fn test_parse_ifd_single_file_big_endian() {
        let data = vec![
            /* 00-01 */ 0x4D, 0x4D, // alignment, big endian
            /* 02-04 */ 0x00, 0x1A, // ifd marker
            /* 05-08 */ 0x00, 0x00, 0x00, 0x08, // ifd offset
            /* 09-10 */ 0x00, 0x01, // ifd file count
            /* 11-12 */ 0x01, 0x0e, // tag
            /* 13-14 */ 0x00, 0x02, // data format
            /* 15-18 */ 0x00, 0x00, 0x00, 0x05, // number of components
            /* 19-22 */ 0x00, 0x00, 0x00, 0x17, // offset
            /* 23-27 */ 0x00, 0x00, 0x00, 0x00, 0x01, // data
        ];

        let (remain, ifd) = parse_ifd(&data, &data[4..], Endian::Big).unwrap();
        assert_eq!(remain, &data[22..]);

        let file = &ifd.files[0];
        assert_eq!(file.tag, 270);
        assert_eq!(file.format, 2);
        assert_eq!(file.components, 5);
        assert_eq!(file.data_length(), 5);
        assert_eq!(file.data, Some(Vec::from(&data[22..])));
    }

    #[test]
    fn test_parse_ifd_file_header_big_endian() {
        let data = &[
            /* 00-01 */ 0x4D, 0x4D, // alignment, big endian
            /* 02-04 */ 0x00, 0x1A, // ifd marker
            /* 05-08 */ 0x00, 0x00, 0x00, 0x08, // ifd offset
            /* 09-10 */ 0x00, 0x01, // ifd file count
            0x01, 0x0e, // tag
            0x00, 0x02, // data format
            0x00, 0x00, 0x00, 0x05, // number of components
            0x00, 0x00, 0x00, 0x16, // offset
            0x00, 0x00, 0x00, 0x00, 0x01, // data
        ];

        let (remain, ifd) = parse_ifd_file(data, &data[10..], Endian::Big).unwrap();
        assert_eq!(remain, &data[22..]);
        assert_eq!(ifd.tag, 270);
        assert_eq!(ifd.format, 2);
        assert_eq!(ifd.components, 5);
        assert_eq!(ifd.data_length(), 5);
        assert_eq!(ifd.offset, Some(22));
        assert_eq!(ifd.data, Some(data[22..].to_vec()));
    }

    #[test]
    fn test_parse_ifd_file_little_endian() {
        let (remain, ifd) = parse_ifd_file(&IFD_LE, &IFD_LE[10..], Endian::Little).unwrap();
        assert_eq!(remain, &IFD_LE[22..]);
        assert_eq!(ifd.tag, 282);
        assert_eq!(ifd.format, 5);
        assert_eq!(ifd.components, 1);
        assert_eq!(ifd.data_length(), 8);
        assert_eq!(ifd.offset, Some(35));
        assert_eq!(ifd.data, Some(IFD_LE[34..].to_vec()));
    }

    #[test]
    fn test_parse_ifd_file_count_not_enough_data() {
        let err = parse_ifd_file_count(&[0xFF], Endian::Big).unwrap_err();
        assert_eq!(err.to_string(), "Exif IFD entries count invalid");
        assert_eq!(
            err.source_to_string(),
            "nom::Parsing requires 1 bytes/chars"
        );
    }

    #[test]
    fn test_parse_ifd_file_count_little_endian() {
        let (remain, marker) = parse_ifd_file_count(&[0x01, 0x00], Endian::Little).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(marker, 0x1);
    }

    #[test]
    fn test_parse_ifd_file_count_big_endian() {
        let (remain, marker) = parse_ifd_file_count(&[0x00, 0x01], Endian::Big).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(marker, 0x1);
    }

    #[test]
    fn test_parse_ifd_offset_negative() {
        let data = &[
            0x01, 0x0e, // tag
            0x00, 0x02, // data format
            0x00, 0x00, 0x00, 0x05, // number of components
            0x00, 0x00, 0x00, 0x01, // invalid offset
            0x00, 0x00, 0x00, 0x00, 0x01, // data
        ];

        let err = parse_ifd_file(data, data, Endian::Big).unwrap_err();
        assert_eq!(err.to_string(), "Exif IFD offset failed: is negative");
    }

    #[test]
    fn test_parse_ifd_offset_not_enough_data() {
        let err = parse_ifd_data_or_offset(&[0xFF], Endian::Big).unwrap_err();
        assert_eq!(err.to_string(), "Exif IFD offset failed");
        assert_eq!(
            err.source_to_string(),
            "nom::Parsing requires 3 bytes/chars"
        );
    }

    #[test]
    fn test_parse_ifd_data_or_offset_little_endian() {
        let mut input = [0x24, 0x00, 0x00, 0x00];
        let (remain, data, offset) = parse_ifd_data_or_offset(&input, Endian::Little).unwrap();
        assert_eq!(remain, &[]);
        input.reverse();
        assert_eq!(data, input);
        assert_eq!(offset, 0x24);
    }

    #[test]
    fn test_parse_ifd_data_or_offset_big_endian() {
        let input = [0x00, 0x00, 0x00, 0x24];
        let (remain, data, offset) = parse_ifd_data_or_offset(&input, Endian::Big).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(data, input);
        assert_eq!(offset, 0x24);
    }

    #[test]
    fn test_parse_ifd_marker_not_enough_data() {
        let err = parse_ifd_marker(&[0xFF], Endian::Big).unwrap_err();
        assert_eq!(err.to_string(), "Exif IFD marker invalid");
        assert_eq!(
            err.source_to_string(),
            "nom::Parsing requires 1 bytes/chars"
        );
    }

    #[test]
    fn test_parse_ifd_marker_little_endian() {
        let (remain, marker) = parse_ifd_marker(&[0x24, 0x00], Endian::Little).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(marker, 0x24);
    }

    #[test]
    fn test_parse_ifd_marker_big_endian() {
        let (remain, marker) = parse_ifd_marker(&[0x00, 0x24], Endian::Big).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(marker, 0x24);
    }

    #[test]
    fn test_parse_tiff_alignemnt_unknown() {
        let err = parse_tiff_endian(&[0xFF, 0xFF]).unwrap_err();
        assert_eq!(err.to_string(), "Exif TIFF alignment invalid");
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
