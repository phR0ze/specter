// JPEG Markers
use crate::errors::JpegError;

pub(crate) const PREFIX: u8 = 0xFF; // JPEG marker prefix
pub(crate) const HEADER: [u8; 2] = [0xFF, 0xD8]; // Start of any JPEG file
pub(crate) const SOF: [u8; 2] = [0xFF, 0xC0]; // Start of frame
pub(crate) const DHT: [u8; 2] = [0xFF, 0xC4]; // Define Huffman Table
pub(crate) const EOI: [u8; 2] = [0xFF, 0xD9]; // End of image data
pub(crate) const SOS: [u8; 2] = [0xFF, 0xDA]; // Start of scan i.e. start of image data
pub(crate) const DQT: [u8; 2] = [0xFF, 0xDB]; // Define Quantinization Table
pub(crate) const DRI: [u8; 2] = [0xFF, 0xDD]; // Define restart interval
pub(crate) const APP0: [u8; 2] = [0xFF, 0xE0]; // JFIF marker segment
pub(crate) const APP1: [u8; 2] = [0xFF, 0xE1]; // Exif marker segment
pub(crate) const APP2: [u8; 2] = [0xFF, 0xE2]; // CIFF Canon Camera Image File Format
pub(crate) const APP8: [u8; 2] = [0xFF, 0xE8]; // SPIFF Still Picture Interchange File Format

pub(crate) fn to_string(marker: &[u8; 2]) -> String {
    match marker {
        &SOF => "Start of Frame".to_string(),
        &DHT => "Define Huffman Table".to_string(),
        &EOI => "End of Image Data".to_string(),
        &SOS => "Start of Scan".to_string(),
        &DQT => "Define Quantinization Table".to_string(),
        &DRI => "Define Restart Interval".to_string(),
        &APP0 => "JFIF Marker Segment".to_string(),
        &APP1 => "Exif Marker Segment".to_string(),
        &APP2 => "CIFF Canon Camera Image File Format".to_string(),
        &APP8 => "SPIFF Still Picture Interchange File Format".to_string(),
        _ => "Unknown marker".to_string(),
    }
}
