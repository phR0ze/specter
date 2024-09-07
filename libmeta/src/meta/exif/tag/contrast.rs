use std::fmt::Display;

pub(crate) enum Contrast {
    Normal, // 0
    Low,    // 1
    High,   // 2
}

impl From<usize> for Contrast {
    fn from(val: usize) -> Self {
        Contrast::from(val as u16)
    }
}

impl From<u16> for Contrast {
    fn from(val: u16) -> Self {
        match val {
            1 => Contrast::Low,
            2 => Contrast::High,
            _ => Contrast::Normal,
        }
    }
}

impl Display for Contrast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Contrast::Normal => write!(f, "Normal"),
            Contrast::Low => write!(f, "Low"),
            Contrast::High => write!(f, "High"),
        }
    }
}
