use std::fmt::Display;

pub(crate) enum Saturation {
    Normal, // 0
    Low,    // 1
    High,   // 2
}

impl From<usize> for Saturation {
    fn from(val: usize) -> Self {
        Saturation::from(val as u16)
    }
}

impl From<u16> for Saturation {
    fn from(val: u16) -> Self {
        match val {
            1 => Saturation::Low,
            2 => Saturation::High,
            _ => Saturation::Normal,
        }
    }
}

impl Display for Saturation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Saturation::Normal => write!(f, "Normal"),
            Saturation::Low => write!(f, "Low"),
            Saturation::High => write!(f, "High"),
        }
    }
}
