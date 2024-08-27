use super::IfdField;

#[derive(Debug, Clone)]
pub(crate) struct Ifd {
    pub(crate) fields: Vec<IfdField>,
}

impl Default for Ifd {
    fn default() -> Self {
        Self { fields: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_ifd() {
    // }
}
