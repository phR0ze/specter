// IFD file data format
pub(crate) mod format {
    pub(crate) const UNSIGNED_BYTE: u16 = 0x01; // 1 byte per component
    pub(crate) const ASCII_STRING: u16 = 0x02; // 1 byte per component
    pub(crate) const UNSIGNED_SHORT: u16 = 0x03; // 2 bytes per component
    pub(crate) const UNSIGNED_LONG: u16 = 0x04; // 4 bytes per component
    pub(crate) const UNSIGNED_RATIONAL: u16 = 0x05; // 8 bytes per component
    pub(crate) const SIGNED_BYTE: u16 = 0x06; // 1 byte per component
    pub(crate) const UNDEFINED: u16 = 0x07; // 1 byte per component
    pub(crate) const SIGNED_SHORT: u16 = 0x08; // 2 bytes per component
    pub(crate) const SIGNED_LONG: u16 = 0x09; // 4 bytes per component
    pub(crate) const SIGNED_RATIONAL: u16 = 0x0A; // 8 bytes per component
    pub(crate) const SINGLE_FLOAT: u16 = 0x0B; // 4 bytes per component
    pub(crate) const DOUBLE_FLOAT: u16 = 0x0C; // 8 bytes per component
}

#[derive(Debug, Clone)]
pub(crate) struct IfdEntry {
    pub(crate) tag: u16,              // type of data
    pub(crate) format: u16,           // data format
    pub(crate) components: u32,       // number of components
    pub(crate) offset: Option<u32>,   // offset to data
    pub(crate) data: Option<Vec<u8>>, // actual data
}

impl Default for IfdEntry {
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

impl IfdEntry {
    // Create a new IFD file
    pub(crate) fn new(tag: u16, format: u16, components: u32) -> Self {
        Self {
            tag,
            format,
            components,
            offset: None,
            data: None,
        }
    }

    // Calculate the length of the IFD data in number of bytes
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
    fn test_entry_data_length() {
        assert_eq!(
            IfdEntry::new(0, format::UNSIGNED_BYTE, 10).data_length(),
            10
        );
        assert_eq!(IfdEntry::new(0, format::ASCII_STRING, 10).data_length(), 10);
        assert_eq!(
            IfdEntry::new(0, format::UNSIGNED_SHORT, 10).data_length(),
            20
        );
        assert_eq!(
            IfdEntry::new(0, format::UNSIGNED_LONG, 10).data_length(),
            40
        );
        assert_eq!(
            IfdEntry::new(0, format::UNSIGNED_RATIONAL, 10).data_length(),
            80
        );
        assert_eq!(IfdEntry::new(0, format::SIGNED_BYTE, 10).data_length(), 10);
        assert_eq!(IfdEntry::new(0, format::UNDEFINED, 10).data_length(), 10);
        assert_eq!(IfdEntry::new(0, format::SIGNED_SHORT, 10).data_length(), 20);
        assert_eq!(IfdEntry::new(0, format::SIGNED_LONG, 10).data_length(), 40);
        assert_eq!(
            IfdEntry::new(0, format::SIGNED_RATIONAL, 10).data_length(),
            80
        );
    }
}
