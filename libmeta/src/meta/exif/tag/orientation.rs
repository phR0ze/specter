use std::fmt::Display;

/// Orientation values
/// https://exiftool.org/TagNames/EXIF.html
pub(crate) enum Orientation {
    Horizontal,                     // 1, normal
    MirrorHorizontal,               // 2,
    Rotate180,                      // 3,
    MirrorVertical,                 // 4,
    MirrorHorizontalAndRotate270CW, // 5,
    Rotate90CW,                     // 6,
    MirrorHorizontalAndRotate90CW,  // 7,
    Rotate270CW,                    // 8,
}

impl From<usize> for Orientation {
    fn from(val: usize) -> Self {
        Orientation::from(val as u16)
    }
}

impl From<u16> for Orientation {
    fn from(val: u16) -> Self {
        match val {
            1 => Orientation::Horizontal,
            2 => Orientation::MirrorHorizontal,
            3 => Orientation::Rotate180,
            4 => Orientation::MirrorVertical,
            5 => Orientation::MirrorHorizontalAndRotate270CW,
            6 => Orientation::Rotate90CW,
            7 => Orientation::MirrorHorizontalAndRotate90CW,
            8 => Orientation::Rotate270CW,
            _ => Orientation::Horizontal, // error checking should never let this happen
        }
    }
}

impl Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Orientation::Horizontal => write!(f, "Horizontal"),
            Orientation::MirrorHorizontal => write!(f, "Mirror Horizontal"),
            Orientation::Rotate180 => write!(f, "Rotate 180"),
            Orientation::MirrorVertical => write!(f, "Mirror Vertical"),
            Orientation::MirrorHorizontalAndRotate270CW => {
                write!(f, "Mirror Horizontal and Rotate 270 CW")
            }
            Orientation::Rotate90CW => write!(f, "Rotate 90 CW"),
            Orientation::MirrorHorizontalAndRotate90CW => {
                write!(f, "Mirror Horizontal and Rotate 90 CW")
            }
            Orientation::Rotate270CW => write!(f, "Rotate 270 CW"),
        }
    }
}
