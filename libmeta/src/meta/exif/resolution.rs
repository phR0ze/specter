use std::fmt::Display;

/// Jfif Density Units
#[derive(Debug, Clone, PartialEq)]
pub enum ResolutionUnit {
    PixelsPerInch,
    PixelsPerCm,
    None,
    Unknown,
}

impl From<usize> for ResolutionUnit {
    fn from(value: usize) -> Self {
        From::from(value as u8)
    }
}

impl From<u8> for ResolutionUnit {
    fn from(value: u8) -> Self {
        match value {
            0x01 => Self::None,
            0x02 => Self::PixelsPerInch,
            0x03 => Self::PixelsPerCm,
            _ => Self::Unknown,
        }
    }
}

impl Display for ResolutionUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolutionUnit::PixelsPerInch => write!(f, "inches"),
            ResolutionUnit::PixelsPerCm => write!(f, "cm"),
            ResolutionUnit::None => write!(f, "none"),
            ResolutionUnit::Unknown => write!(f, "unknown"),
        }
    }
}
