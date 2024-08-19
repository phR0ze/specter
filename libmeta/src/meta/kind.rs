use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Jpeg,
    None,
}

impl Default for Kind {
    fn default() -> Self {
        Kind::None
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Kind::Jpeg => write!(f, "Jpeg"),
            Kind::None => write!(f, "None"),
        }
    }
}
