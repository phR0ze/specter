use std::fmt::Display;

pub(crate) enum Sharpness {
    Normal, // 0
    Soft,   // 1
    Hard,   // 2
}

impl From<usize> for Sharpness {
    fn from(val: usize) -> Self {
        Sharpness::from(val as u16)
    }
}

impl From<u16> for Sharpness {
    fn from(val: u16) -> Self {
        match val {
            1 => Sharpness::Soft,
            2 => Sharpness::Hard,
            _ => Sharpness::Normal,
        }
    }
}

impl Display for Sharpness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sharpness::Normal => write!(f, "Normal"),
            Sharpness::Soft => write!(f, "Soft"),
            Sharpness::Hard => write!(f, "Hard"),
        }
    }
}
