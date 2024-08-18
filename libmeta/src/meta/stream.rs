use std::io;

//const chunk_len: usize = 10; // Use 4096 bytes or 4KB buffer in production

///
pub struct Stream<'a> {
    buffer: [u8; 4096],
    reader: &'a mut dyn io::Read,
}

impl<'a> Stream<'a> {
    fn new(reader: &'a mut impl io::Read) -> Self {
        Self {
            buffer: [0; 4096],
            reader,
        }
    }
}

impl<'a> Iterator for Stream<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let buf = self.reader.read(&mut self.buffer);
        None
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
    fn test_stream_iterator() {
        let mut data = io::Cursor::new([0xFF, 0x00]);
        let stream = Stream::new(&mut data);

        // for segment in SegmentIter {
        //     println!("{:?}", segment.marker);
        // }
        // let (_, segment) = parse_segment(&JPEG_DATA_1[2..]).unwrap();
        // assert_eq!(segment.marker, APP1);
        // assert_eq!(segment.length, 860);
        // assert_eq!(segment.data.len(), 860);
    }
}
