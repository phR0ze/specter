use std::io;

// Read out 1 byte as a u8 value
pub(crate) fn read_u8(reader: &mut impl io::Read) -> Result<u8, io::Error> {
    let mut buf = [0u8; 1];
    reader.read_exact(&mut buf).and(Ok(buf[0]))
}

// Read out 2 bytes in Big Endian as a u16 value
pub(crate) fn read_be_u16(reader: &mut impl io::Read) -> Result<u16, io::Error> {
    let mut buf = [0u8; 2];
    reader.read_exact(&mut buf).and(Ok(u16::from_be_bytes(buf)))
}

// Read out a variable number of bytes
pub(crate) fn read_bytes(reader: &mut impl io::Read, len: usize) -> Result<Vec<u8>, io::Error> {
    let mut buf = vec![0; len];
    reader.read_exact(&mut buf[..])?;
    Ok(buf)
}

/// Skip bytes until a marker is found or EOF is reached
/// * returns Ok(false) if EOF is reached otherwise Ok(true)
pub(crate) fn skip_until(reader: &mut impl io::BufRead, marker: u8) -> Result<bool, io::Error> {
    match reader.read_until(marker, &mut Vec::new())? {
        0 => Ok(false),
        _ => Ok(true),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_u8() {
        let data = [0x01];
        assert_eq!(read_u8(&mut &data[..]).unwrap(), 1);

        let data = [0x10];
        assert_eq!(read_u8(&mut &data[..]).unwrap(), 16);

        let data = [];
        assert_eq!(read_u8(&mut &data[..]).unwrap_err().to_string(), "failed to fill whole buffer");
    }

    #[test]
    fn test_read_u16() {
        let data = [0x00, 0x01];
        assert_eq!(read_be_u16(&mut &data[..]).unwrap(), 1);

        let data = [0x00, 0x10];
        assert_eq!(read_be_u16(&mut &data[..]).unwrap(), 16);

        let data = [];
        assert_eq!(
            read_be_u16(&mut &data[..]).unwrap_err().to_string(),
            "failed to fill whole buffer"
        );
    }

    #[test]
    fn test_read_len() {
        let data = [0x00, 0x01, 0x02, 0x03];
        assert_eq!(read_bytes(&mut &data[..], 2).unwrap(), vec![0x00, 0x01]);
        assert_eq!(read_bytes(&mut &data[..], 3).unwrap(), vec![0x00, 0x01, 0x02]);
        assert_eq!(read_bytes(&mut &data[..], 4).unwrap(), vec![0x00, 0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_skip_until() {
        // Skip until EOF
        let mut reader = &([0x00, 0x01, 0x02, 0x03])[..];
        assert_eq!(skip_until(&mut reader, 0x04).unwrap(), true);

        // Nothing to skip
        let mut reader = &([])[..];
        assert_eq!(skip_until(&mut reader, 0x04).unwrap(), false);

        // Skip to 0x01
        let mut reader = &([0x00, 0x01, 0x02, 0x03])[..];
        skip_until(&mut reader, 0x01).unwrap();
        assert_eq!(read_u8(&mut reader).unwrap(), 0x02);

        // Skip further
        let mut reader = &([0x00, 0x01, 0x02, 0x03])[..];
        skip_until(&mut reader, 0x02).unwrap();
        assert_eq!(read_u8(&mut reader).unwrap(), 0x03);
    }
}
