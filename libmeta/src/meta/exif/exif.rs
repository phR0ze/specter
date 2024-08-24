use nom::bytes::streaming as nom_bytes;
use nom::number::streaming as nom_nums;

use super::{Ifd, IfdFile, BIG_ENDIAN, EXIF_IDENTIFIER, LITTLE_ENDIAN};
use crate::errors::ExifError;

/// Simplify the Exif return type slightly
pub type ExifResult<T> = Result<T, ExifError>;

#[derive(Debug, Clone)]
pub struct Exif {
    alignment: [u8; 2], // byte alignment, 0x4949 = Little-Endian, 0x4D4D = Big-Endian
}

impl Default for Exif {
    fn default() -> Self {
        Self { alignment: [0, 0] }
    }
}

impl Exif {
    /// Check if the TIFF byte alignment is Big-Endian
    pub fn is_big_endian(&self) -> bool {
        self.alignment == BIG_ENDIAN
    }
}

/// Parse the given data into a Exif structure
/// * **Field**        | **Bytes** | **Description**
/// * *Identifier*     | 6     | `4578 6966 0000` = `Exif` and 2 bytes of padding 0000
/// * *Tiff header*    | 8     | `4949 2A00 0800 0000`, 2 bytes align `0x4949` is Little-Endian, `0x4D4D` is Big-Endian
pub fn parse(input: &[u8]) -> ExifResult<Exif> {
    let mut exif = Exif::default();
    let (remain, _) = parse_exif_header(input)?;

    // Parse alignment
    let (remain, alignment) = parse_tiff_alignment(remain)?;
    let big_endian = alignment == BIG_ENDIAN;
    exif.alignment = alignment;

    // Parse IFD 0 marker
    let (remain, marker) = parse_ifd_marker(remain, big_endian)?;
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
fn parse_tiff_alignment(input: &[u8]) -> ExifResult<(&[u8], [u8; 2])> {
    let (remain, alignment) = nom::branch::alt((
        nom::bytes::streaming::tag::<[u8; 2], &[u8], nom::error::Error<&[u8]>>(BIG_ENDIAN),
        nom::bytes::streaming::tag::<[u8; 2], &[u8], nom::error::Error<&[u8]>>(LITTLE_ENDIAN),
    ))(input)
    .map_err(|x| ExifError::alignment_invalid().with_nom_source(x))?;

    Ok((remain, alignment.try_into().unwrap()))
}

/// (2 bytes) Parse the TIFF IFD 0 marker, always 2A00 or 0024
fn parse_ifd_marker(input: &[u8], big_endian: bool) -> ExifResult<(&[u8], u16)> {
    match big_endian {
        true => nom::number::streaming::be_u16(input),
        false => nom::number::streaming::le_u16(input),
    }
    .map_err(|x| ExifError::marker_invalid().with_nom_source(x))
}

/// Parse out a 4 byte values as either raw data bytes in big endian or an offset
/// Returns: (remaining bytes, data bytes, offset)
fn parse_ifd_data_or_offset(input: &[u8], big_endian: bool) -> ExifResult<(&[u8], [u8; 4], u32)> {
    let (remain, offset) = match big_endian {
        true => nom_nums::be_u32(input),
        false => nom_nums::le_u32(input),
    }
    .map_err(|x| ExifError::offset_failed().with_nom_source(x))?;
    Ok((remain, offset.to_be_bytes(), offset))
}

/// (2 bytes) Parse number of file entries in the IFD
fn parse_ifd_file_count(input: &[u8], big_endian: bool) -> ExifResult<(&[u8], u16)> {
    match big_endian {
        true => nom_nums::be_u16(input),
        false => nom_nums::le_u16(input),
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
    big_endian: bool,
) -> ExifResult<(&'a [u8], IfdFile)> {
    // tag: 2 bytes
    let (remain, tag) = match big_endian {
        true => nom_nums::be_u16(remain),
        false => nom_nums::le_u16(remain),
    }
    .map_err(|x| ExifError::ifd_file_tag_failed().with_nom_source(x))?;

    // data format: 2 bytes
    let (remain, format) = match big_endian {
        true => nom_nums::be_u16(remain),
        false => nom_nums::le_u16(remain),
    }
    .map_err(|x| ExifError::ifd_file_data_format_failed().with_nom_source(x))?;

    // number of components: 4 bytes
    let (remain, components) = match big_endian {
        true => nom_nums::be_u32(remain),
        false => nom_nums::le_u32(remain),
    }
    .map_err(|x| ExifError::ifd_file_component_count_failed().with_nom_source(x))?;

    // offset to data value: 4 bytes
    let (remain, data, offset) = parse_ifd_data_or_offset(remain, big_endian)?;

    // create the ifd file and calculate if there is an offset to extract data from
    let mut file = IfdFile::new(tag, format, components);
    if file.data_length() > 4 {
        let remain = remain; // save the current position by creating a new variable

        // skip to the offset location
        let (remain, _) = nom_bytes::take(offset as usize - (input.len() - remain.len()))(remain)
            .map_err(|x| ExifError::offset_failed().with_nom_source(x))?;

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
/// * **remain** is where the header starts
fn parse_ifd<'a>(
    input: &'a [u8],
    remain: &'a [u8],
    big_endian: bool,
) -> ExifResult<(&'a [u8], Ifd)> {
    let mut files: Vec<IfdFile> = Vec::new();

    // skip to the offset location
    let (remain, _) = nom_bytes::take(offset as usize - (input.len() - remain.len()))(remain)
        .map_err(|x| ExifError::offset_failed().with_nom_source(x))?;

    // read the data from the offset location
    let (_, data) = nom_bytes::take(file.data_length())(remain)
        .map_err(|x| ExifError::offset_failed().with_nom_source(x))?;
    // Parse out the number of IFD files to expect
    let (remain, count) = parse_ifd_file_count(input, big_endian)?;

    // Parse out each of the IFD files
    let mut remain: &[u8] = remain;
    // for _ in 0..count {
    //     let (r, ifd) = parse_ifd_file(remain, big_endian)?;
    //     remain = r;
    //     ifds.push(ifd);
    // }

    Ok((remain, Ifd::default()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{errors::BaseError, exif::LITTLE_ENDIAN};

    const IFD_BE: [u8; 17] = [
        0x01, 0x0e, // tag
        0x00, 0x02, // data format
        0x00, 0x00, 0x00, 0x05, // number of components
        0x00, 0x00, 0x00, 0x0C, // offset
        0x00, 0x00, 0x00, 0x00, 0x01, // data
    ];

    const IFD_LE: [u8; 46] = [
        0x49, 0x49, // alignment, little endian
        0x2A, 0x00, // ifd marker
        0x08, 0x00, 0x00, 0x00, // (8) offset from start of TIFF header
        0x02, 0x00, // file count
        // 10 bytes consumed
        // file header 1
        0x1A, 0x01, // tag: 0x011A, XResolution
        0x05, 0x00, // data format: 5, unsigned rational
        0x01, 0x00, 0x00, 0x00, // components: 1
        0x26, 0x00, 0x00, 0x00, // data length is (1)(8)=8 bytes which means payload is offset
        // 22 bytes consumed
        // file header 2
        0x69, 0x87, // tag:
        0x04, 0x00, // data format: 4, unsigned long
        0x01, 0x00, 0x00, 0x00, // components: 1
        0x11, 0x02, 0x00, 0x00, // data length is (1)(4)=4 bytes which means payload is 0x0211
        // 34 bytes consumed
        0x40, 0x00, 0x00, 0x00, // Next IFD offset: starts at 0x40
        // 38 bytes consumed
        0x48, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // data for file 1
    ];

    const EXIF_HEADER: [u8; 6] = [0x45, 0x78, 0x69, 0x66, 0x00, 0x00];

    #[test]
    fn test_parse_ifd_file_header_big_endian() {
        let (remain, ifd) = parse_ifd_file(&IFD_BE, &IFD_BE, true).unwrap();
        assert_eq!(remain, &IFD_BE[12..]);
        assert_eq!(ifd.tag, 270);
        assert_eq!(ifd.format, 2);
        assert_eq!(ifd.components, 5);
        assert_eq!(ifd.data_length(), 5);
        assert_eq!(ifd.offset, Some(12));
        assert_eq!(ifd.data, Some(IFD_BE[12..].to_vec()));
    }

    #[test]
    fn test_parse_ifd_file_little_endian() {
        let (remain, ifd) = parse_ifd_file(&IFD_LE, &IFD_LE[10..], false).unwrap();
        assert_eq!(remain, &IFD_LE[22..]);
        assert_eq!(ifd.tag, 282);
        assert_eq!(ifd.format, 5);
        assert_eq!(ifd.components, 1);
        assert_eq!(ifd.data_length(), 8);
        assert_eq!(ifd.offset, Some(38));
        assert_eq!(ifd.data, Some(IFD_LE[38..].to_vec()));
    }

    // #[test]
    // fn test_parse_ifd_file_data_little_endian() {
    //     let files = &IFD_LE[10..];
    // }

    // #[test]
    // fn test_parse_ifd_files_little_endian() {
    //     let files = &IFD_LE[10..];
    //     let (remain, files) = parse_ifd_file_headers(&files, 2, true).unwrap();
    //     //assert_eq!(remain, &IFD_BE);

    //     // let file = &files[0];
    //     // let (remain, ifd) = parse_ifd_file(&data, false).unwrap();
    //     // assert_eq!(remain, &IFD_FILES_LE[22..]);
    //     // assert_eq!(ifd.tag, 282);
    //     // assert_eq!(ifd.format, 5);
    //     // assert_eq!(ifd.components, 1);
    //     // assert_eq!(ifd.data_length(), 8);
    //     // assert_eq!(ifd.payload, 38);
    //     // d

    //     // assert_eq!(ifd.tag, 270);
    //     // assert_eq!(ifd.format, 2);
    //     // assert_eq!(ifd.components, 11);
    //     // assert_eq!(ifd.data_length(), 11);
    //     // assert_eq!(ifd.payload, 86);

    //     // let file = &files[1];
    //     // assert_eq!(file.tag, 270);
    //     // assert_eq!(file.format, 2);
    //     // assert_eq!(file.components, 11);
    //     // assert_eq!(file.data_length(), 11);
    //     // assert_eq!(file.payload, 86);
    // }

    // // #[test]
    // // fn test_parse_ifd_offset_skip_little_endian() {
    // //     let (remain, offset) = parse_ifd_offset_and_skip(&IFD_LE, &IFD_LE[4..], false).unwrap();
    // //     assert_eq!(remain, &IFD_LE[8..]);
    // //     assert_eq!(offset, 8);
    // // }

    // #[test]
    // fn test_parse_ifd_files_big_endian() {
    //     let data = [IFD_BE, IFD_BE, IFD_BE].concat();
    //     let (remain, headers) = parse_ifd_file_headers(&data, 2, true).unwrap();
    //     assert_eq!(remain, &IFD_BE);

    //     let header = &headers[0];
    //     assert_eq!(header.tag, 270);
    //     assert_eq!(header.format, 2);
    //     assert_eq!(header.components, 11);
    //     assert_eq!(header.data_length(), 11);
    //     assert_eq!(header.payload, 86);

    //     let header = &headers[1];
    //     assert_eq!(header.tag, 270);
    //     assert_eq!(header.format, 2);
    //     assert_eq!(header.components, 11);
    //     assert_eq!(header.data_length(), 11);
    //     assert_eq!(header.payload, 86);
    // }

    #[test]
    fn test_parse_ifd_file_count_not_enough_data() {
        let err = parse_ifd_file_count(&[0xFF], true).unwrap_err();
        assert_eq!(err.to_string(), "Exif IFD entries count invalid");
        assert_eq!(
            err.source_to_string(),
            "nom::Parsing requires 1 bytes/chars"
        );
    }

    #[test]
    fn test_parse_ifd_file_count_little_endian() {
        let (remain, marker) = parse_ifd_file_count(&[0x01, 0x00], false).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(marker, 0x1);
    }

    #[test]
    fn test_parse_ifd_file_count_big_endian() {
        let (remain, marker) = parse_ifd_file_count(&[0x00, 0x01], true).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(marker, 0x1);
    }

    #[test]
    fn test_parse_ifd_offset_not_enough_data() {
        let err = parse_ifd_data_or_offset(&[0xFF], true).unwrap_err();
        assert_eq!(err.to_string(), "Exif IFD offset failed");
        assert_eq!(
            err.source_to_string(),
            "nom::Parsing requires 3 bytes/chars"
        );
    }

    #[test]
    fn test_parse_ifd_data_or_offset_little_endian() {
        let mut input = [0x24, 0x00, 0x00, 0x00];
        let (remain, data, offset) = parse_ifd_data_or_offset(&input, false).unwrap();
        assert_eq!(remain, &[]);
        input.reverse();
        assert_eq!(data, input);
        assert_eq!(offset, 0x24);
    }

    #[test]
    fn test_parse_ifd_data_or_offset_big_endian() {
        let input = [0x00, 0x00, 0x00, 0x24];
        let (remain, data, offset) = parse_ifd_data_or_offset(&input, true).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(data, input);
        assert_eq!(offset, 0x24);
    }

    #[test]
    fn test_parse_ifd_marker_not_enough_data() {
        let err = parse_ifd_marker(&[0xFF], true).unwrap_err();
        assert_eq!(err.to_string(), "Exif IFD marker invalid");
        assert_eq!(
            err.source_to_string(),
            "nom::Parsing requires 1 bytes/chars"
        );
    }

    #[test]
    fn test_parse_ifd_marker_little_endian() {
        let (remain, marker) = parse_ifd_marker(&[0x24, 0x00], false).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(marker, 0x24);
    }

    #[test]
    fn test_parse_ifd_marker_big_endian() {
        let (remain, marker) = parse_ifd_marker(&[0x00, 0x24], true).unwrap();
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
