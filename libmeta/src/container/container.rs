use std::fmt;

use super::Jpeg;

#[derive(Debug)]
pub enum Container {
    Jpeg(Jpeg),
    None,
}

impl Default for Container {
    fn default() -> Self {
        Container::None
    }
}

impl fmt::Display for Container {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Container::Jpeg(_) => write!(f, "Jpeg"),
            Container::None => write!(f, "None"),
        }
    }
}
