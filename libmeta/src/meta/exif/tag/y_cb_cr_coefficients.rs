use std::fmt::Display;

/// YCbCrPositioning values
/// https://exiftool.org/TagNames/EXIF.html
pub(crate) enum YCbCrPositioning {
    Centered, // 1
    CoSited,  // 2
}

impl From<usize> for YCbCrPositioning {
    fn from(val: usize) -> Self {
        YCbCrPositioning::from(val as u16)
    }
}

impl From<u16> for YCbCrPositioning {
    fn from(val: u16) -> Self {
        match val {
            1 => YCbCrPositioning::Centered,
            2 => YCbCrPositioning::CoSited,
            _ => YCbCrPositioning::Centered, // error checking should never let this happen
        }
    }
}

impl Display for YCbCrPositioning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &YCbCrPositioning::Centered => write!(f, "Centered"),
            &YCbCrPositioning::CoSited => write!(f, "Co-sited"),
        }
    }
}
