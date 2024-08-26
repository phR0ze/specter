use super::IfdFile;

#[derive(Debug, Clone)]
pub(crate) struct Ifd {
    pub(crate) files: Vec<IfdFile>,
}

impl Default for Ifd {
    fn default() -> Self {
        Self { files: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_ifd() {
    // }
}
