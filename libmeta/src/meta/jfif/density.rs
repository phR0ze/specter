use std::fmt::Display;

/// Jfif Density Units
#[derive(Debug, Clone, PartialEq)]
pub enum DensityUnit {
    PixelsPerInch,
    PixelsPerCm,
    None,
    Unknown,
}
impl From<u8> for DensityUnit {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::None,
            0x01 => Self::PixelsPerInch,
            0x02 => Self::PixelsPerCm,
            _ => Self::Unknown,
        }
    }
}

impl Display for DensityUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DensityUnit::PixelsPerInch => write!(f, "in"),
            DensityUnit::PixelsPerCm => write!(f, "cm"),
            DensityUnit::None => write!(f, "none"),
            DensityUnit::Unknown => write!(f, "unknown"),
        }
    }
}
