use nom::bytes::streaming as nom_bytes;
use nom::number::streaming as nom_nums;

use super::{tag::Tag, Endian, ExifResult, IfdField};
use crate::errors::ExifError;

#[derive(Debug, Clone)]
pub(crate) struct Ifd {
    pub(crate) endian: Endian,
    pub(crate) fields: Vec<IfdField>,
}

impl Ifd {
    pub(crate) fn new(endian: Endian) -> Self {
        Self { endian, fields: Vec::new() }
    }

    /// Parse IFD returns a list of ifds
    /// * **input** is the full data source from tiff header alignment
    /// * **remain** starts with the ifd field count
    pub(crate) fn parse<'a>(
        input: &'a [u8],
        remain: &'a [u8],
        endian: Endian,
        offset: usize,
    ) -> ExifResult<(&'a [u8], Ifd)> {
        let mut ifd = Ifd::new(endian);

        // Skip to offset location
        let (remain, _) = nom_bytes::take(offset - (input.len() - remain.len()))(remain)
            .map_err(|x| ExifError::parse(": offset to IFD").with_nom_source(x))?;

        // Parse out the number of IFD fields to expect
        let (remain, count) = parse_field_count(remain, endian)?;

        // Parse out each of the IFD fields
        let mut outer = remain;
        for _ in 0..count {
            let (inner, field) = IfdField::parse(input, outer, endian)?;
            outer = inner;
            ifd.fields.push(field);
        }

        Ok((outer, ifd))
    }

    /// Get a field by its tag
    pub(crate) fn field_by_tag(&self, tag: Tag) -> Option<&IfdField> {
        self.fields.iter().find(|x| x.tag == tag)
    }
}

/// (2 bytes) Parse number of entries in the IFD
fn parse_field_count(input: &[u8], endian: Endian) -> ExifResult<(&[u8], u16)> {
    match endian {
        Endian::Big => nom_nums::be_u16(input),
        Endian::Little => nom_nums::le_u16(input),
    }
    .map_err(|x| ExifError::parse(": IFD field count").with_nom_source(x))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::BaseError;
    use crate::exif::test_data::EXIF_TEST_DATA;
    use crate::meta::exif::{format, tag, tag::Tag};

    #[test]
    fn test_parse_exif_ifd() {
        let (_, ifd) =
            Ifd::parse(&EXIF_TEST_DATA, &EXIF_TEST_DATA[86..], Endian::Big, 134).unwrap();
        assert_eq!(ifd.fields.len(), 3);

        let field = &ifd.fields[0];
        assert_eq!(field.tag, tag::EXIF_VERSION);
        assert_eq!(field.format, format::UNDEFINED);
        assert_eq!(field.components, 4);
        assert_eq!(field.offset, None);
        assert_eq!(field.data, Some(vec![0x30, 0x32, 0x33, 0x30]));
        assert_eq!(field.to_ascii(), Some("0230".to_string()));

        let field = &ifd.fields[1];
        assert_eq!(field.tag, tag::EXIF_IMAGE_WIDTH);
        assert_eq!(field.format, format::UNSIGNED_SHORT);
        assert_eq!(field.components, 1);
        assert_eq!(field.offset, None);
        assert_eq!(field.data, Some(vec![0x00, 0x0f, 0x00, 0x00]));
        assert_eq!(field.to_unsigned(), Some(15));

        let field = &ifd.fields[2];
        assert_eq!(field.tag, tag::EXIF_IMAGE_HEIGHT);
        assert_eq!(field.format, format::UNSIGNED_SHORT);
        assert_eq!(field.components, 1);
        assert_eq!(field.offset, None);
        assert_eq!(field.data, Some(vec![0x00, 0x07, 0x00, 0x00]));
        assert_eq!(field.to_unsigned(), Some(7));
    }

    #[test]
    fn test_parse_ifd1() {
        let (_, ifd) =
            Ifd::parse(&EXIF_TEST_DATA, &EXIF_TEST_DATA[86..], Endian::Big, 176).unwrap();

        let field0 = &ifd.fields[0];
        assert_eq!(field0.tag, tag::THUMBNAIL_OFFSET);
        assert_eq!(field0.format, format::UNSIGNED_LONG);
        assert_eq!(field0.components, 1);
        assert_eq!(field0.offset, None);
        assert_eq!(field0.data, Some(vec![0x00, 0x00, 0x00, 0xce]));
        assert_eq!(field0.to_unsigned(), Some(206));

        let field1 = &ifd.fields[1];
        assert_eq!(field1.tag, tag::THUMBNAIL_LENGTH);
        assert_eq!(field1.format, format::UNSIGNED_LONG);
        assert_eq!(field1.components, 1);
        assert_eq!(field1.offset, None);
        assert_eq!(field1.data, Some(vec![0x00, 0x00, 0x02, 0x88]));
        assert_eq!(field1.to_unsigned(), Some(648));
    }

    #[test]
    fn test_parse_ifd0() {
        let (_, ifd) = Ifd::parse(&EXIF_TEST_DATA, &EXIF_TEST_DATA[8..], Endian::Big, 8).unwrap();

        let field0 = &ifd.fields[0];
        assert_eq!(field0.endian, Endian::Big);
        assert_eq!(field0.tag, tag::IMAGE_DESCRIPTION);
        assert_eq!(field0.format, format::ASCII_STRING);
        assert_eq!(field0.components, 11);
        assert_eq!(field0.length(), 11);
        assert_eq!(field0.offset, Some(86));
        let offset = field0.offset.unwrap() as usize;
        assert_eq!(
            field0.data,
            Some(Vec::from(&EXIF_TEST_DATA[offset..offset + field0.length() as usize]))
        );
        assert_eq!(field0.to_ascii(), Some("Test image".into()));

        let field1 = &ifd.fields[1];
        assert_eq!(field1.endian, Endian::Big);
        assert_eq!(field1.tag, tag::X_RESOLUTION);
        assert_eq!(field1.format, format::UNSIGNED_RATIONAL);
        assert_eq!(field1.components, 1);
        assert_eq!(field1.length(), 8);
        assert_eq!(field1.offset, Some(98));
        let offset = field1.offset.unwrap() as usize;
        assert_eq!(
            field1.data,
            Some(Vec::from(&EXIF_TEST_DATA[offset..offset + field1.length() as usize]))
        );
        assert_eq!(field1.to_rational(), Some((72, 1)));

        let field2 = &ifd.fields[2];
        assert_eq!(field2.endian, Endian::Big);
        assert_eq!(field2.tag, tag::Y_RESOLUTION);
        assert_eq!(field2.format, format::UNSIGNED_RATIONAL);
        assert_eq!(field2.components, 1);
        assert_eq!(field2.offset, Some(106));
        let offset = field2.offset.unwrap() as usize;
        assert_eq!(field2.length(), 8);
        assert_eq!(
            field2.data,
            Some(Vec::from(&EXIF_TEST_DATA[offset..offset + field2.length() as usize]))
        );
        assert_eq!(field2.to_rational(), Some((72, 1)));

        let field3 = &ifd.fields[3];
        assert_eq!(field3.endian, Endian::Big);
        assert_eq!(field3.tag, tag::RESOLUTION_UNIT);
        assert_eq!(field3.format, format::UNSIGNED_SHORT);
        assert_eq!(field3.components, 1);
        assert_eq!(field3.offset, None);
        assert_eq!(field3.length(), 2);
        assert_eq!(field3.data, Some(vec![0x00, 0x02, 0x00, 0x00]));
        assert_eq!(field3.to_unsigned(), Some(2));

        let field4 = &ifd.fields[4];
        assert_eq!(field4.endian, Endian::Big);
        assert_eq!(field4.tag, tag::DATE_TIME);
        assert_eq!(field4.format, format::ASCII_STRING);
        assert_eq!(field4.components, 20);
        assert_eq!(field4.offset, Some(114));
        assert_eq!(field4.length(), 20);
        let offset = field4.offset.unwrap() as usize;
        assert_eq!(
            field4.data,
            Some(Vec::from(&EXIF_TEST_DATA[offset..offset + field4.length() as usize]))
        );
        assert_eq!(field4.to_ascii(), Some("2016:05:04 03:02:01".into()));

        let field5 = &ifd.fields[5];
        assert_eq!(field5.endian, Endian::Big);
        assert_eq!(field5.tag, tag::EXIF_SUB_IFD_OFFSET);
        assert_eq!(field5.format, format::UNSIGNED_LONG);
        assert_eq!(field5.components, 1);
        assert_eq!(field5.offset, None);
        assert_eq!(field5.length(), 4);
        assert_eq!(field5.data, Some(vec![0x00, 0x00, 0x00, 0x86]));
        assert_eq!(field5.to_unsigned(), Some(134));
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

        let (remain, ifd) = Ifd::parse(&data, &data[8..], Endian::Big, 8).unwrap();
        assert_eq!(remain, &data[22..]);

        let field = &ifd.fields[0];
        assert_eq!(field.tag, Tag::from(270));
        assert_eq!(field.format, 2);
        assert_eq!(field.components, 5);
        assert_eq!(field.length(), 5);
        assert_eq!(field.offset, Some(22));
        assert_eq!(field.data, Some(Vec::from(&data[22..])));
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

    #[test]
    fn test_parse_ifd_fields_little_endian() {
        let (remain, ifd) = Ifd::parse(&IFD_LE, &IFD_LE[8..], Endian::Little, 8).unwrap();
        assert_eq!(remain, &IFD_LE[34..]);

        let field = &ifd.fields[0];
        assert_eq!(field.tag, Tag::from(282));
        assert_eq!(field.format, 5);
        assert_eq!(field.components, 1);
        assert_eq!(field.offset, Some(34));
        assert_eq!(field.length(), 8);
        assert_eq!(field.data, Some(Vec::from(&IFD_LE[34..])));

        let field = &ifd.fields[1];
        assert_eq!(field.tag, Tag::from(34665));
        assert_eq!(field.format, 4);
        assert_eq!(field.components, 1);
        assert_eq!(field.offset, None);
        assert_eq!(field.length(), 4);
        assert_eq!(field.data, Some(Vec::from(&[0x2B, 0x00, 0x00, 0x00])));
    }

    #[test]
    fn test_parse_field_count_not_enough_data() {
        let err = parse_field_count(&[0xFF], Endian::Big).unwrap_err();
        assert_eq!(err.to_string(), "Exif parse failed: IFD field count");
        assert_eq!(err.source_to_string(), "nom::Parsing requires 1 bytes/chars");
    }

    #[test]
    fn test_parse_ifd_field_count_little_endian() {
        let (remain, marker) = parse_field_count(&[0x01, 0x00], Endian::Little).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(marker, 0x1);
    }

    #[test]
    fn test_parse_ifd_field_count_big_endian() {
        let (remain, marker) = parse_field_count(&[0x00, 0x01], Endian::Big).unwrap();
        assert_eq!(remain, &[]);
        assert_eq!(marker, 0x1);
    }
}
