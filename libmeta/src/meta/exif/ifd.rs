use super::IfdTag;

#[derive(Debug, Clone)]
pub(crate) struct Ifd {
    pub(crate) entries: Vec<IfdTag>,
}

impl Default for Ifd {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_ifd() {
    // }
}
