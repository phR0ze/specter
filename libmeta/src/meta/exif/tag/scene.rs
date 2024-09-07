use std::fmt::Display;

pub(crate) enum Scene {
    Standard,  // 0
    Landscape, // 1
    Portrait,  // 2
    Night,     // 3
    Other,     // 4
}

impl From<usize> for Scene {
    fn from(val: usize) -> Self {
        Scene::from(val as u16)
    }
}

impl From<u16> for Scene {
    fn from(val: u16) -> Self {
        match val {
            0 => Scene::Standard,
            1 => Scene::Landscape,
            2 => Scene::Portrait,
            3 => Scene::Night,
            _ => Scene::Other,
        }
    }
}

impl Display for Scene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scene::Standard => write!(f, "Standard"),
            Scene::Landscape => write!(f, "Landscape"),
            Scene::Portrait => write!(f, "Portrait"),
            Scene::Night => write!(f, "Night"),
            Scene::Other => write!(f, "Other"),
        }
    }
}
