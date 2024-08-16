#[derive(Debug, Copy, Clone)]
pub struct Jfif;

impl Jfif {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_meta_is_valid_jpeg() {
        // let mut header = io::Cursor::new(&[0xFF, 0xD8]);
        // let meta = new(&mut header);
        // assert!(meta.is_ok());
        // assert!(meta.unwrap().kind() == Kind::Jpeg);
    }

    #[test]
    fn test_new_meta_is_not_valid() {
        // // unknown header type
        // let mut header = io::Cursor::new(&[0xFF, 0x00]);
        // assert_eq!(
        //     new(&mut header).unwrap_err().to_string(),
        //     "unknown header [ff, 0]"
        // );

        // // bad header length
        // let mut header = io::Cursor::new(&[0xFF]);
        // assert_eq!(
        //     new(&mut header).unwrap_err().to_string(),
        //     "read error: failed to fill whole buffer"
        // );
    }
}
