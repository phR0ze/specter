use std::fmt::Display;

pub(crate) enum LensSpec {
    None,         // 0
}

impl Display for LensSpec{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LensSpec::None => write!(f, "None"),
        }
    }
}
