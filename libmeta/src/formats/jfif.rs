#[derive(Debug, Clone)]
pub struct Jfif {
    pub major: u8,                  // major version
    pub minor: u8,                  // minor version
    pub density: DensityUnit,       // density unit
    pub x_density: u16,             // horizontal pixel density
    pub y_density: u16,             // vertical pixel density
    pub x_thumbnail: u8,            // horizontal pixels of the embedded RGB thumbnail
    pub y_thumbnail: u8,            // vertical pixels of the embedded RGB thumbnail
    pub thumbnail: Option<Vec<u8>>, // uncompressed 24 bit RGB raster thumbnail
}

impl Jfif {
    pub fn new() -> Self {
        Self {
            major: 0,
            minor: 0,
            density: DensityUnit::None,
            x_density: 0,
            y_density: 0,
            x_thumbnail: 0,
            y_thumbnail: 0,
            thumbnail: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum DensityUnit {
    PixelsPerInch,
    PixelsPerCm,
    None,
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
}
