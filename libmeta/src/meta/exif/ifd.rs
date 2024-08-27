use super::IfdEntry;

#[derive(Debug, Clone)]
pub(crate) struct Ifd {
    pub(crate) entries: Vec<IfdEntry>,
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
