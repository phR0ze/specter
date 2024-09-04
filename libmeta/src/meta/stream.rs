use std::io;

const chunk_len: usize = 10; // Use 4096 bytes or 4KB buffer in production

/// Stream bytes from the media source.
/// Data accumulates until acked by the caller.
pub(crate) struct Stream<'a> {
    buffer: Vec<u8>, // Vec<T> is similar to Box<[T]> i.e. owned pointer to heap
    reader: &'a mut dyn io::BufRead,
}

impl<'a> Stream<'a> {
    pub(crate) fn new(reader: &'a mut impl io::BufRead) -> Self {
        Self { buffer: Vec::with_capacity(chunk_len), reader }
    }

    /// Read at most `buffer.len()` bytes into the buffer.
    /// * Gives similar functionality to reader.by_ref().take() but allowed on trait object.
    /// * May return less than the buffer length if eof is reached.
    /// * Retries on io::ErrorKind::Interrupted automatically.
    /// * Returns the number of bytes read.  
    pub(crate) fn read_at_most(&mut self, buffer: &mut [u8]) -> Result<usize, io::Error> {
        let mut count: usize = 0;
        loop {
            match self.reader.read(&mut buffer[count..]) {
                Ok(0) => return Ok(count), // eof or buffer is full
                Ok(n) => count += n,       // buffer full
                Err(e) => {
                    // Continue reading if only interrupted
                    if e.kind() != io::ErrorKind::Interrupted {
                        return Err(e);
                    }
                }
            };
        }
    }
    pub(crate) fn data(&self) -> Option<Result<&[u8], io::Error>> {
        Some(Ok(&self.buffer))
    }
}

impl<'a> Iterator for Stream<'a> {
    type Item = Result<&'a [u8], io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        // let mut buf = [0u8; chunk_len];
        // match self.read_at_most(&mut buf) {
        //     Ok(0) => (),
        //     Ok(n) => {
        //         self.buffer.extend_from_slice(&buf[..n]);
        //         ()
        //     }
        //     Err(e) => return Some(Err(e)),
        // }
        // //Some(Ok(self.buffer.as_ref()))
        // Some(Ok(&self.buffer[..]))
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    const JFIF_DATA_1: [u8; 18] = [
        0xff, 0xe0, 0x00, 0x10, 0x4a, 0x46, 0x49, 0x46, 0x00, 0x01, 0x02, 0x01, 0x00, 0x48, 0x00,
        0x48, 0x00, 0x00,
    ];

    #[test]
    fn test_stream_iterator() {
        //let mut data = io::Cursor::new([0xFF, 0x00]);
        //let stream = Stream::new(&mut data);

        // for segment in SegmentIter {
        //     println!("{:?}", segment.marker);
        // }
        // let (_, segment) = parse_segment(&JPEG_DATA_1[2..]).unwrap();
        // assert_eq!(segment.marker, APP1);
        // assert_eq!(segment.length, 860);
        // assert_eq!(segment.data.len(), 860);
    }

    #[test]
    fn test_read_at_most_read_all() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let mut reader = io::Cursor::new(data);

        let mut collected = Vec::new();
        let mut buf = [0u8; 6];
        let n = Stream::new(&mut reader).read_at_most(&mut buf).unwrap();
        assert_eq!(n, 6);
        assert_eq!(buf, data);
        collected.extend_from_slice(&buf);
        assert_eq!(collected, data);
    }

    #[test]
    fn test_read_at_most_fill_buffer_but_not_all_data() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let mut reader = io::Cursor::new(data);

        let mut buf = [0u8; 5];
        let n = Stream::new(&mut reader).read_at_most(&mut buf).unwrap();
        assert_eq!(n, 5);
        assert_eq!(buf, data[0..5]);
    }
}
