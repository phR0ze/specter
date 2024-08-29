use core::panic;

use nom::bytes::streaming as nom_bytes;
use nom::number::streaming as nom_nums;

use super::{tag, Endian, Ifd, IfdField, BIG_ENDIAN, EXIF_IDENTIFIER, LITTLE_ENDIAN};
use crate::errors::ExifError;

/// Simplify the Exif return type slightly
pub type ExifResult<T> = Result<T, ExifError>;

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

    // Parse TIFF alignment
    let (remain, endian) = parse_tiff_endian(remain)?;

    // Parse TIFF version
    let (remain, marker) = parse_tiff_version(remain, endian)?;
    if marker != 0x24 {
        return Err(ExifError::identifier_invalid());
    }

    // Parser offset to the first IFD
    let (remain, _, offset) = parse_ifd_data_or_offset(remain, endian)?;
    let (remain, _) = nom_bytes::take(offset as usize - (input.len() - remain.len()))(remain)
        .map_err(|x| ExifError::offset_failed().with_nom_source(x))?;

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
fn parse_tiff_version(input: &[u8], endian: Endian) -> ExifResult<(&[u8], u16)> {
    match endian {
        Endian::Big => nom::number::streaming::be_u16(input),
        Endian::Little => nom::number::streaming::le_u16(input),
    }
    .map_err(|x| ExifError::version_invalid().with_nom_source(x))
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

/// (2 bytes) Parse number of entries in the IFD
fn parse_ifd_field_count(input: &[u8], endian: Endian) -> ExifResult<(&[u8], u16)> {
    match endian {
        Endian::Big => nom_nums::be_u16(input),
        Endian::Little => nom_nums::le_u16(input),
    }
    .map_err(|x| ExifError::count_invalid().with_nom_source(x))
}

/// Parse IFD field which is 12 bytes of header an arbitrary data component
/// e.g. TT TT ff ff NN NN NN NN DD DD DD DD
/// * 2 byte Tag number
/// * 2 byte Data format
/// * 4 byte Number of components
/// * 4 byte Offset to data value or data itself
/// * **input** is the full data source from tiff header alignment
/// * **remain** is where the header starts
/// * Returns: (remaining bytes, IfdField)
fn parse_ifd_field<'a>(
    input: &'a [u8],
    remain: &'a [u8],
    endian: Endian,
) -> ExifResult<(&'a [u8], IfdField)> {
    // tag: 2 bytes
    let (remain, tag) = match endian {
        Endian::Big => nom_nums::be_u16(remain),
        Endian::Little => nom_nums::le_u16(remain),
    }
    .map_err(|x| ExifError::ifd_field_tag_failed().with_nom_source(x))?;

    // data format: 2 bytes
    let (remain, format) = match endian {
        Endian::Big => nom_nums::be_u16(remain),
        Endian::Little => nom_nums::le_u16(remain),
    }
    .map_err(|x| ExifError::ifd_field_data_format_failed().with_nom_source(x))?;

    // number of components: 4 bytes
    let (remain, components) = match endian {
        Endian::Big => nom_nums::be_u32(remain),
        Endian::Little => nom_nums::le_u32(remain),
    }
    .map_err(|x| ExifError::ifd_field_components_failed().with_nom_source(x))?;

    // offset to data value: 4 bytes
    let (remain, data, offset) = parse_ifd_data_or_offset(remain, endian)?;

    // create the ifd field and calculate if there is an offset to extract data from
    let mut field = IfdField::new(endian, tag, format, components);
    if field.length() > 4 {
        let remain = remain; // save the current position by creating a new variable

        // skip to the offset location
        let consumed = input.len() - remain.len();
        if consumed > offset as usize {
            return Err(ExifError::offset_failed().with_msg("is negative"));
        }
        let remain = if offset as usize > consumed {
            let (remain, _) = nom_bytes::take(offset as usize - consumed)(remain)
                .map_err(|x| ExifError::offset_failed().with_nom_source(x))?;
            remain
        } else {
            remain
        };

        // read the data from the offset location
        let (_, data) = nom_bytes::take(field.length())(remain)
            .map_err(|x| ExifError::offset_failed().with_nom_source(x))?;

        field.offset = Some(offset);
        field.data = Some(data.to_vec());
    } else {
        field.data = Some(data.to_vec());
    }

    Ok((remain, field))
}

/// Parse IFD
/// * **input** is the full data source from tiff header alignment
/// * **remain** starts with the ifd offset
fn parse_ifd<'a>(input: &'a [u8], remain: &'a [u8], endian: Endian) -> ExifResult<(&'a [u8], Ifd)> {
    let mut ifd = Ifd::default();

    // Parse out the number of IFD fields to expect
    let (remain, count) = parse_ifd_field_count(remain, endian)?;

    // Parse out each of the IFD fields
    let mut remain = remain;
    for _ in 0..count {
        let (r, f) = parse_ifd_field(input, remain, endian)?;
        remain = r;
        ifd.fields.push(f);
    }

    // read the ifd offset and skip to the ifd offset location
    // let (remain, _, offset) = parse_ifd_data_or_offset(remain, endian)?;
    // let (remain, _) = nom_bytes::take(offset as usize - (input.len() - remain.len()))(remain)
    //     .map_err(|x| ExifError::offset_failed().with_nom_source(x))?;

    Ok((remain, ifd))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        errors::BaseError,
        exif::{field, format, tag, LITTLE_ENDIAN},
        formats::jpeg::JPEG_TEST_DATA,
    };

    // #[test]
    // fn test_parse_ifd_big_endian() {
    //     let (remain, ifd) =
    //         parse_ifd(&JPEG_TEST_DATA[30..], &JPEG_TEST_DATA[34..], Endian::Little).unwrap();
    //     // assert_eq!(remain, &IFD_LE[34..]);

    //     // let file = &ifd.files[0];
    //     // assert_eq!(file.tag, 282);
    //     // assert_eq!(file.format, 5);
    //     // assert_eq!(file.components, 1);
    //     // assert_eq!(file.offset, Some(35));
    //     // assert_eq!(file.data_length(), 8);
    //     // assert_eq!(file.data, Some(Vec::from(&IFD_LE[34..])));
    // }

    #[test]
    fn test_parse_ifd1() {
        let (remain, ifd1) =
            parse_ifd(&EXIF_TEST_DATA, &EXIF_TEST_DATA[176..], Endian::Big).unwrap();

        let field0 = &ifd1.fields[0];
        assert_eq!(field0.tag, tag::JPEG_THUMBNAIL_OFFSET);
        assert_eq!(field0.format, format::UNSIGNED_LONG);
        assert_eq!(field0.components, 1);
        assert_eq!(field0.offset, None);
        assert_eq!(field0.data, Some(vec![0x00, 0x00, 0x00, 0xce]));
        assert_eq!(field0.to_unsigned(), Some(206));

        let field1 = &ifd1.fields[1];
        assert_eq!(field1.tag, tag::JPEG_THUMBNAIL_LENGTH);
        assert_eq!(field1.format, format::UNSIGNED_LONG);
        assert_eq!(field1.components, 1);
        assert_eq!(field1.offset, None);
        assert_eq!(field1.data, Some(vec![0x00, 0x00, 0x02, 0x88]));
        assert_eq!(field1.to_unsigned(), Some(648));
    }

    #[test]
    fn test_parse_ifd0() {
        let (remain, ifd0) = parse_ifd(&EXIF_TEST_DATA, &EXIF_TEST_DATA[8..], Endian::Big).unwrap();

        let field0 = &ifd0.fields[0];
        assert_eq!(field0.endian, Endian::Big);
        assert_eq!(field0.tag, tag::IMAGE_DESCRIPTION);
        assert_eq!(field0.format, format::ASCII_STRING);
        assert_eq!(field0.components, 11);
        assert_eq!(field0.length(), 11);
        assert_eq!(field0.offset, Some(86));
        let offset = field0.offset.unwrap() as usize;
        assert_eq!(
            field0.data,
            Some(Vec::from(
                &EXIF_TEST_DATA[offset..offset + field0.length() as usize]
            ))
        );
        assert_eq!(field0.to_ascii(), Some("Test image".into()));

        let field1 = &ifd0.fields[1];
        assert_eq!(field1.endian, Endian::Big);
        assert_eq!(field1.tag, tag::X_RESOLUTION);
        assert_eq!(field1.format, format::UNSIGNED_RATIONAL);
        assert_eq!(field1.components, 1);
        assert_eq!(field1.length(), 8);
        assert_eq!(field1.offset, Some(98));
        let offset = field1.offset.unwrap() as usize;
        assert_eq!(
            field1.data,
            Some(Vec::from(
                &EXIF_TEST_DATA[offset..offset + field1.length() as usize]
            ))
        );
        assert_eq!(field1.to_rational(), Some((72, 1)));

        let field2 = &ifd0.fields[2];
        assert_eq!(field2.endian, Endian::Big);
        assert_eq!(field2.tag, tag::Y_RESOLUTION);
        assert_eq!(field2.format, format::UNSIGNED_RATIONAL);
        assert_eq!(field2.components, 1);
        assert_eq!(field2.offset, Some(106));
        let offset = field2.offset.unwrap() as usize;
        assert_eq!(field2.length(), 8);
        assert_eq!(
            field2.data,
            Some(Vec::from(
                &EXIF_TEST_DATA[offset..offset + field2.length() as usize]
            ))
        );
        assert_eq!(field2.to_rational(), Some((72, 1)));

        let field3 = &ifd0.fields[3];
        assert_eq!(field3.endian, Endian::Big);
        assert_eq!(field3.tag, tag::RESOLUTION_UNIT);
        assert_eq!(field3.format, format::UNSIGNED_SHORT);
        assert_eq!(field3.components, 1);
        assert_eq!(field3.offset, None);
        assert_eq!(field3.length(), 2);
        assert_eq!(field3.data, Some(vec![0x00, 0x02, 0x00, 0x00]));
        assert_eq!(field3.to_unsigned(), Some(2));

        let field4 = &ifd0.fields[4];
        assert_eq!(field4.endian, Endian::Big);
        assert_eq!(field4.tag, tag::DATE_TIME);
        assert_eq!(field4.format, format::ASCII_STRING);
        assert_eq!(field4.components, 20);
        assert_eq!(field4.offset, Some(114));
        assert_eq!(field4.length(), 20);
        let offset = field4.offset.unwrap() as usize;
        assert_eq!(
            field4.data,
            Some(Vec::from(
                &EXIF_TEST_DATA[offset..offset + field4.length() as usize]
            ))
        );
        assert_eq!(field4.to_ascii(), Some("2016:05:04 03:02:01".into()));

        let field5 = &ifd0.fields[5];
        assert_eq!(field5.endian, Endian::Big);
        assert_eq!(field5.tag, tag::EXIF_IFD_OFFSET);
        assert_eq!(field5.format, format::UNSIGNED_LONG);
        assert_eq!(field5.components, 1);
        assert_eq!(field5.offset, None);
        assert_eq!(field5.length(), 4);
        assert_eq!(field5.data, Some(vec![0x00, 0x00, 0x00, 0x86]));
        assert_eq!(field5.to_unsigned(), Some(134));
    }

    const IFD_LE: [u8; 42] = [
        /* 00-01 */ 0x49, 0x49, // alignment, little endian
        /* 02-03 */ 0x2A, 0x00, // ifd marker
        /* 04-07 */ 0x08, 0x00, 0x00, 0x00, // ifd start
        /* 08-09 */ 0x02, 0x00, // field count
        /* 10-11 */ 0x1A, 0x01, // id: 0x011A, XResolution
        /* 12-13 */ 0x05, 0x00, // data format: 5, unsigned rational
        /* 14-17 */ 0x01, 0x00, 0x00, 0x00, // components: 1, so data 8 bytes
        /* 18-21 */ 0x22, 0x00, 0x00, 0x00, // offset of 34
        /* 22-23 */ 0x69, 0x87, // id:
        /* 24-25 */ 0x04, 0x00, // data format: 4, unsigned long
        /* 26-29 */ 0x01, 0x00, 0x00, 0x00, // components: 1
        /* 30-33 */ 0x2B, 0x00, 0x00, 0x00, // data for field 2
        /* 34-41 */ 0x48, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // data for field 1
    ];

    const EXIF_HEADER: [u8; 6] = [0x45, 0x78, 0x69, 0x66, 0x00, 0x00];

    #[test]
    fn test_parse_ifd_fields_little_endian() {
        let (remain, ifd) = parse_ifd(&IFD_LE, &IFD_LE[8..], Endian::Little).unwrap();
        assert_eq!(remain, &IFD_LE[34..]);

        let field = &ifd.fields[0];
        assert_eq!(field.tag, 282);
        assert_eq!(field.format, 5);
        assert_eq!(field.components, 1);
        assert_eq!(field.offset, Some(34));
        assert_eq!(field.length(), 8);
        assert_eq!(field.data, Some(Vec::from(&IFD_LE[34..])));

        let field = &ifd.fields[1];
        assert_eq!(field.tag, 34665);
        assert_eq!(field.format, 4);
        assert_eq!(field.components, 1);
        assert_eq!(field.offset, None);
        assert_eq!(field.length(), 4);
        assert_eq!(field.data, Some(Vec::from(&[0x00, 0x00, 0x00, 0x2B])));
    }

    #[test]
    fn test_parse_ifd_single_field_big_endian() {
        let data = vec![
            /* 00-01 */ 0x4D, 0x4D, // alignment, big endian
            /* 02-03 */ 0x00, 0x1A, // ifd marker
            /* 04-07 */ 0x00, 0x00, 0x00, 0x08, // ifd offset
            /* 08-09 */ 0x00, 0x01, // ifd field count
            /* 10-11 */ 0x01, 0x0e, // id
            /* 12-13 */ 0x00, 0x02, // data format
            /* 14-17 */ 0x00, 0x00, 0x00, 0x05, // number of components
            /* 18-21 */ 0x00, 0x00, 0x00, 0x16, // offset
            /* 22-26 */ 0x00, 0x00, 0x00, 0x00, 0x01, // data
        ];

        let (remain, ifd) = parse_ifd(&data, &data[8..], Endian::Big).unwrap();
        assert_eq!(remain, &data[22..]);

        let field = &ifd.fields[0];
        assert_eq!(field.tag, 270);
        assert_eq!(field.format, 2);
        assert_eq!(field.components, 5);
        assert_eq!(field.length(), 5);
        assert_eq!(field.data, Some(Vec::from(&data[22..])));
    }

    #[test]
    fn test_parse_ifd_field_header_big_endian() {
        let data = &[
            /* 00-01 */ 0x4D, 0x4D, // alignment, big endian
            /* 02-04 */ 0x00, 0x1A, // ifd marker
            /* 05-08 */ 0x00, 0x00, 0x00, 0x08, // ifd offset
            /* 09-10 */ 0x00, 0x01, // ifd field count
            0x01, 0x0e, // id
            0x00, 0x02, // data format
            0x00, 0x00, 0x00, 0x05, // number of components
            0x00, 0x00, 0x00, 0x16, // offset
            0x00, 0x00, 0x00, 0x00, 0x01, // data
        ];

        let (remain, ifd) = parse_ifd_field(data, &data[10..], Endian::Big).unwrap();
        assert_eq!(remain, &data[22..]);
        assert_eq!(ifd.tag, 270);
        assert_eq!(ifd.format, 2);
        assert_eq!(ifd.components, 5);
        assert_eq!(ifd.length(), 5);
        assert_eq!(ifd.offset, Some(22));
        assert_eq!(ifd.data, Some(data[22..].to_vec()));
    }

    #[test]
    fn test_parse_ifd_field_little_endian() {
        let (remain, ifd) = parse_ifd_field(&IFD_LE, &IFD_LE[10..], Endian::Little).unwrap();
        assert_eq!(remain, &IFD_LE[22..]);
        assert_eq!(ifd.tag, 282);
        assert_eq!(ifd.format, 5);
        assert_eq!(ifd.components, 1);
        assert_eq!(ifd.length(), 8);
        assert_eq!(ifd.offset, Some(34));
        assert_eq!(ifd.data, Some(IFD_LE[34..].to_vec()));
    }

    #[test]
    fn test_parse_ifd_field_count_not_enough_data() {
        let err = parse_ifd_field_count(&[0xFF], Endian::Big).unwrap_err();
        assert_eq!(err.to_string(), "Exif IFD field count invalid");
        assert_eq!(
            err.source_to_string(),
            "nom::Parsing requires 1 bytes/chars"
        );
    }

    #[test]
    fn test_parse_ifd_field_count_little_endian() {
        let (remain, marker) = parse_ifd_field_count(&[0x01, 0x00], Endian::Little).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(marker, 0x1);
    }

    #[test]
    fn test_parse_ifd_field_count_big_endian() {
        let (remain, marker) = parse_ifd_field_count(&[0x00, 0x01], Endian::Big).unwrap();
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

        let err = parse_ifd_field(data, data, Endian::Big).unwrap_err();
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
    fn test_parse_tiff_version_not_enough_data() {
        let err = parse_tiff_version(&[0xFF], Endian::Big).unwrap_err();
        assert_eq!(err.to_string(), "Exif TIFF version invalid");
        assert_eq!(
            err.source_to_string(),
            "nom::Parsing requires 1 bytes/chars"
        );
    }

    #[test]
    fn test_parse_tiff_version_little_endian() {
        let (remain, marker) = parse_tiff_version(&[0x24, 0x00], Endian::Little).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(marker, 0x24);
    }

    #[test]
    fn test_parse_tiff_version_big_endian() {
        let (remain, marker) = parse_tiff_version(&[0x00, 0x24], Endian::Big).unwrap();
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

// No Exif Header included
#[cfg(test)]
pub(crate) const EXIF_TEST_DATA: [u8; 854] = [
    // TIFF header
    /* 000-001 */ 0x4d, 0x4d, // byte alignment
    /* 002-003 */ 0x00, 0x2a, // version identifier
    /* 004-007 */ 0x00, 0x00, 0x00, 0x08, // IFD 0: offset
    //
    // IFD 0
    /* 008-009 */ 0x00, 0x06, // IFD 0: field count
    //
    /* 010-011 */ 0x01, 0x0e, // Field 0, Image description
    /* 012-013 */ 0x00, 0x02, // Field 0: format ASCII
    /* 014-017 */ 0x00, 0x00, 0x00, 0x0b, // Field 0: components (11)
    /* 018-021 */ 0x00, 0x00, 0x00, 0x56, // Field 0: offset (86), length (11)
    //
    /* 022-023 */ 0x01, 0x1a, // Field 1: XResolution
    /* 024-025 */ 0x00, 0x05, // Field 1: format Unsigned Rational
    /* 026-029 */ 0x00, 0x00, 0x00, 0x01, // Field 1: components
    /* 030-033 */ 0x00, 0x00, 0x00, 0x62, // Field 1: offset (98), length (8)
    //
    /* 034-035 */ 0x01, 0x1b, // Field 2: YResolution
    /* 036-037 */ 0x00, 0x05, // Field 2: format
    /* 038-041 */ 0x00, 0x00, 0x00, 0x01, // Field 2: components (1)
    /* 042-045 */ 0x00, 0x00, 0x00, 0x6a, // Field 2: offset (106), length (8)
    //
    /* 046-047 */ 0x01, 0x28, // Field 3: Resolution Unit
    /* 048-049 */ 0x00, 0x03, // Field 3: Unsigned short
    /* 050-053 */ 0x00, 0x00, 0x00, 0x01, // Field 3: components (1)
    /* 054-057 */ 0x00, 0x02, 0x00, 0x00, // Field 3: data (512)
    //
    /* 058-059 */ 0x01, 0x32, // Field 4: Date Time
    /* 060-061 */ 0x00, 0x02, // Field 4: ASCII
    /* 062-065 */ 0x00, 0x00, 0x00, 0x14, // Field 4: components (20)
    /* 066-069 */ 0x00, 0x00, 0x00, 0x72, // Field 4: offset (114), length (20)
    //
    /* 070-071 */ 0x87, 0x69, // Field 5: Exif Offset
    /* 072-073 */ 0x00, 0x04, // Field 5: Unsigned Long
    /* 074-077 */ 0x00, 0x00, 0x00, 0x01, // Field 5: components (1)
    /* 078-081 */ 0x00, 0x00, 0x00, 0x86, // Field 5: data (134)
    //
    /* 082-085 */ 0x00, 0x00, 0x00, 0xb0, // IFD 1: offset (176)
    //
    // Field 0: Data (11)
    /* 086-092 */ 0x54, 0x65, 0x73, 0x74, 0x20, 0x69, 0x6d,
    /* 093-097 */ 0x61, 0x67, 0x65, 0x00, 0x46,
    //
    // Field 1: Data (8)
    /* 098-101 */ 0x00, 0x00, 0x00, 0x48,
    /* 102-105 */ 0x00, 0x00, 0x00, 0x01,
    //
    // Field 2: Data (8)
    /* 106-109 */ 0x00, 0x00, 0x00, 0x48,
    /* 110-113 */ 0x00, 0x00, 0x00, 0x01,
    //
    // Field 4: Data (20)
    /* 114-117 */ 0x32, 0x30, 0x31, 0x36,
    /* 118-121 */ 0x3a, 0x30, 0x35, 0x3a, //
    /* 122-125 */ 0x30, 0x34, 0x20, 0x30, //
    /* 126-129 */ 0x33, 0x3a, 0x30, 0x32, //
    /* 130-133 */ 0x3a, 0x30, 0x31, 0x00, //
    //
    // EXIF IFD
    /* 134-135 */ 0x00, 0x03, // EXIF IFD: field count
    /* 136-137 */ 0x90, 0x00, // Field 0: Exif Version
    /* 138-147 */ 0x00, 0x07, 0x00, 0x00, 0x00, 0x04, 0x30, 0x32, 0x33, 0x30, //
    /* 148-157 */ 0xa0, 0x02, 0x00, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x0f, //
    /* 158-167 */ 0x00, 0x00, 0xa0, 0x03, 0x00, 0x03, 0x00, 0x00, 0x00, 0x01, //
    /* 168-175 */ 0x00, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
    //
    // IFD 1
    /* 176-177 */ 0x00, 0x02, // IFD 1: field count
    //
    /* 178-179 */ 0x02, 0x01, // Field 0: JPEG Thumbnail Offset
    /* 180-181 */ 0x00, 0x04, // Field 0: data format, unsigned long
    /* 182-185 */ 0x00, 0x00, 0x00, 0x01, // Field 0: components (1)
    /* 186-189 */ 0x00, 0x00, 0x00, 0xce, // Field 0: data which is offset (206)
    //
    /* 190-191 */ 0x02, 0x02, // Field 1: JPEG Thumbnail Length
    /* 192-193 */ 0x00, 0x04, // Field 1: data format, unsigned long
    /* 194-197 */ 0x00, 0x00, 0x00, 0x01, // Field 1: components (1)
    /* 198-201 */ 0x00, 0x00, 0x02, 0x88, // Field 1: data (648)
    //
    // END of IFDs
    /* 202-205 */ 0x00, 0x00, 0x00, 0x00,
    //
    // JPEG Thumbnail (648 bytes)
    /* 206-207 */ 0xff, 0xd8, // JPEG Header
    /* 208-209 */ 0xff, 0xe0, // JFIF: marker
    /* 210-211 */ 0x00, 0x10, // JFIF: size
    /* 212-216 */ 0x4a, 0x46, 0x49, 0x46, 0x00, // JFIF signature
    /* 217-225 */ 0x01, 0x01, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, // JFIF: data
    /* 226-227 */ 0xff, 0xdb, // JPEG Quantinization table marker
    /* 228-235 */ 0x00, 0x43, 0x00, 0x08, 0x06, 0x06, 0x07, 0x06, //
    /* 236-245 */ 0x05, 0x08, 0x07, 0x07, 0x07, 0x09, 0x09, 0x08, 0x0a, 0x0c, //
    /* 246-255 */ 0x14, 0x0d, 0x0c, 0x0b, 0x0b, 0x0c, 0x19, 0x12, 0x13, 0x0f, //
    /* 256-265 */ 0x14, 0x1d, 0x1a, 0x1f, 0x1e, 0x1d, 0x1a, 0x1c, 0x1c, 0x20, //
    /* 266-275 */ 0x24, 0x2e, 0x27, 0x20, 0x22, 0x2c, 0x23, 0x1c, 0x1c, 0x28, //
    /* 276-285 */ 0x37, 0x29, 0x2c, 0x30, 0x31, 0x34, 0x34, 0x34, 0x1f, 0x27, //
    /* 286-295 */ 0x39, 0x3d, 0x38, 0x32, 0x3c, 0x2e, 0x33, 0x34, 0x32, 0xff, //
    /* 296-305 */ 0xdb, 0x00, 0x43, 0x01, 0x09, 0x09, 0x09, 0x0c, 0x0b, 0x0c, //
    /* 306-315 */ 0x18, 0x0d, 0x0d, 0x18, 0x32, 0x21, 0x1c, 0x21, 0x32, 0x32, //
    /* 316-325 */ 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, //
    /* 326-335 */ 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, //
    /* 336-345 */ 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, //
    /* 346-355 */ 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, //
    /* 356-363 */ 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, 0x32, //
    /* 364-365 */ 0xff, 0xc0, // JPEG SOF marker
    /* 366-375 */ 0x00, 0x11, 0x08, 0x00, 0x03, 0x00, 0x07, 0x03, 0x01, 0x22, //
    /* 376-382 */ 0x00, 0x02, 0x11, 0x01, 0x03, 0x11, 0x01, //
    /* 383-384 */ 0xff, 0xc4, // JPEG Huffman table
    /*     385 */ 0x00, //
    /* 386-395 */ 0x1f, 0x00, 0x00, 0x01, 0x05, 0x01, 0x01, 0x01, 0x01, 0x01, //
    /* 396-405 */ 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, //
    /* 406-415 */ 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, //
    /* 416-417 */ 0xff, 0xc4, // JPEG Huffman table
    /* 418-425 */ 0x00, 0xb5, 0x10, 0x00, 0x02, 0x01, 0x03, 0x03, //
    /* 426-435 */ 0x02, 0x04, 0x03, 0x05, 0x05, 0x04, 0x04, 0x00, 0x00, 0x01, //
    /* 436-445 */ 0x7d, 0x01, 0x02, 0x03, 0x00, 0x04, 0x11, 0x05, 0x12, 0x21, //
    /* 446-455 */ 0x31, 0x41, 0x06, 0x13, 0x51, 0x61, 0x07, 0x22, 0x71, 0x14, //
    /* 456-465 */ 0x32, 0x81, 0x91, 0xa1, 0x08, 0x23, 0x42, 0xb1, 0xc1, 0x15, //
    /* 466-475 */ 0x52, 0xd1, 0xf0, 0x24, 0x33, 0x62, 0x72, 0x82, 0x09, 0x0a, //
    /* 476-485 */ 0x16, 0x17, 0x18, 0x19, 0x1a, 0x25, 0x26, 0x27, 0x28, 0x29, //
    /* 486-495 */ 0x2a, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x43, 0x44, //
    /* 496-405 */ 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x53, 0x54, 0x55, 0x56, //
    /* 506-515 */ 0x57, 0x58, 0x59, 0x5a, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, //
    /* 516-525 */ 0x69, 0x6a, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, //
    /* 526-535 */ 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x92, 0x93, //
    /* 536-545 */ 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0xa2, 0xa3, 0xa4, //
    /* 546-555 */ 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xb2, 0xb3, 0xb4, 0xb5, //
    /* 556-565 */ 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, //
    /* 566-575 */ 0xc7, 0xc8, 0xc9, 0xca, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, //
    /* 576-585 */ 0xd8, 0xd9, 0xda, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, //
    /* 586-595 */ 0xe8, 0xe9, 0xea, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, //
    /* 596-598 */ 0xf8, 0xf9, 0xfa, //
    /* 599-600 */ 0xff, 0xc4, // JPEG Huffman table
    /* 601-605 */ 0x00, 0x1f, 0x01, 0x00, 0x03, //
    /* 606-615 */ 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x00, //
    /* 616-625 */ 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, //
    /* 626-631 */ 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, //
    /* 632-633 */ 0xff, 0xc4, // JPEG Huffman table
    /* 634-635 */ 0x00, 0xb5, //
    /* 636-645 */ 0x11, 0x00, 0x02, 0x01, 0x02, 0x04, 0x04, 0x03, 0x04, 0x07, //
    /* 646-655 */ 0x05, 0x04, 0x04, 0x00, 0x01, 0x02, 0x77, 0x00, 0x01, 0x02, //
    /* 656-665 */ 0x03, 0x11, 0x04, 0x05, 0x21, 0x31, 0x06, 0x12, 0x41, 0x51, //
    /* 666-675 */ 0x07, 0x61, 0x71, 0x13, 0x22, 0x32, 0x81, 0x08, 0x14, 0x42, //
    /* 676-685 */ 0x91, 0xa1, 0xb1, 0xc1, 0x09, 0x23, 0x33, 0x52, 0xf0, 0x15, //
    /* 686-695 */ 0x62, 0x72, 0xd1, 0x0a, 0x16, 0x24, 0x34, 0xe1, 0x25, 0xf1, //
    /* 696-705 */ 0x17, 0x18, 0x19, 0x1a, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x35, //
    /* 706-715 */ 0x36, 0x37, 0x38, 0x39, 0x3a, 0x43, 0x44, 0x45, 0x46, 0x47, //
    /* 716-725 */ 0x48, 0x49, 0x4a, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, //
    /* 726-735 */ 0x5a, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x73, //
    /* 736-745 */ 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x82, 0x83, 0x84, //
    /* 746-755 */ 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x92, 0x93, 0x94, 0x95, //
    /* 756-765 */ 0x96, 0x97, 0x98, 0x99, 0x9a, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, //
    /* 766-775 */ 0xa7, 0xa8, 0xa9, 0xaa, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, //
    /* 776-785 */ 0xb8, 0xb9, 0xba, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, //
    /* 786-795 */ 0xc9, 0xca, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, //
    /* 796-805 */ 0xda, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, //
    /* 806-815 */ 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xff, //
    /* 816-825 */ 0xda, 0x00, 0x0c, 0x03, 0x01, 0x00, 0x02, 0x11, 0x03, 0x11, //
    /* 826-835 */ 0x00, 0x3f, 0x00, 0xf4, 0x5d, 0x1e, 0x15, 0xb9, 0x96, 0xd2, //
    /* 836-845 */ 0x09, 0x9a, 0x57, 0x89, 0x0c, 0x85, 0x53, 0xcd, 0x6c, 0x77, //
    /* 846-851 */ 0xf7, 0xe6, 0x8a, 0x28, 0xa0, 0x0f, //
    /* 852-853 */ 0xff, 0xd9, // JPEG EOI
];
