use super::format;

/// Represents an IFD tag in cluding its identifier, format, number of components, and data.
#[derive(Debug, Clone)]
pub(crate) struct IfdField {
    pub(crate) tag: u16,              // identifier
    pub(crate) format: u16,           // data format
    pub(crate) components: u32,       // number of components
    pub(crate) offset: Option<u32>,   // offset to data
    pub(crate) data: Option<Vec<u8>>, // actual data
}

impl Default for IfdField {
    fn default() -> Self {
        Self {
            tag: 0,
            format: 0,
            components: 0,
            offset: None,
            data: None,
        }
    }
}

impl IfdField {
    // Create a new IFD tag
    pub(crate) fn new(tag: u16, format: u16, components: u32) -> Self {
        Self {
            tag,
            format,
            components,
            offset: None,
            data: None,
        }
    }

    // Calculate the length of the tag's data in number of bytes
    pub(crate) fn data_length(&self) -> u64 {
        match self.format {
            format::UNSIGNED_BYTE => self.components as u64,
            format::ASCII_STRING => self.components as u64,
            format::UNSIGNED_SHORT => self.components as u64 * 2,
            format::UNSIGNED_LONG => self.components as u64 * 4,
            format::UNSIGNED_RATIONAL => self.components as u64 * 8,
            format::SIGNED_BYTE => self.components as u64,
            format::UNDEFINED => self.components as u64,
            format::SIGNED_SHORT => self.components as u64 * 2,
            format::SIGNED_LONG => self.components as u64 * 4,
            format::SIGNED_RATIONAL => self.components as u64 * 8,
            format::SINGLE_FLOAT => self.components as u64 * 4,
            format::DOUBLE_FLOAT => self.components as u64 * 8,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_data_length() {
        assert_eq!(
            IfdField::new(0, format::UNSIGNED_BYTE, 10).data_length(),
            10
        );
        assert_eq!(IfdField::new(0, format::ASCII_STRING, 10).data_length(), 10);
        assert_eq!(
            IfdField::new(0, format::UNSIGNED_SHORT, 10).data_length(),
            20
        );
        assert_eq!(
            IfdField::new(0, format::UNSIGNED_LONG, 10).data_length(),
            40
        );
        assert_eq!(
            IfdField::new(0, format::UNSIGNED_RATIONAL, 10).data_length(),
            80
        );
        assert_eq!(IfdField::new(0, format::SIGNED_BYTE, 10).data_length(), 10);
        assert_eq!(IfdField::new(0, format::UNDEFINED, 10).data_length(), 10);
        assert_eq!(IfdField::new(0, format::SIGNED_SHORT, 10).data_length(), 20);
        assert_eq!(IfdField::new(0, format::SIGNED_LONG, 10).data_length(), 40);
        assert_eq!(
            IfdField::new(0, format::SIGNED_RATIONAL, 10).data_length(),
            80
        );
    }
}
