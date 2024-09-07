use std::fmt::Display;

pub(crate) enum Gain {
    None,         // 0
    LowGainUp,    // 1
    HighGainUp,   // 2
    LowGainDown,  // 3
    HighGainDown, // 4
}

impl From<usize> for Gain {
    fn from(val: usize) -> Self {
        Gain::from(val as u16)
    }
}

impl From<u16> for Gain {
    fn from(val: u16) -> Self {
        match val {
            0 => Gain::None,
            1 => Gain::LowGainUp,
            2 => Gain::HighGainUp,
            3 => Gain::LowGainDown,
            _ => Gain::HighGainDown,
        }
    }
}

impl Display for Gain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gain::None => write!(f, "None"),
            Gain::LowGainUp => write!(f, "Low gain up"),
            Gain::HighGainUp => write!(f, "High gain up"),
            Gain::LowGainDown => write!(f, "Low gain down"),
            Gain::HighGainDown => write!(f, "High gain down"),
        }
    }
}
