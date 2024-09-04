use super::IfdField;

#[derive(Debug, Clone)]
pub(crate) struct Ifd {
    pub(crate) endian: super::Endian,
    pub(crate) fields: Vec<IfdField>,
}

impl Ifd {
    pub(crate) fn new(endian: super::Endian) -> Self {
        Self { endian, fields: Vec::new() }
    }
}

#[cfg(test)]
mod tests {

    // #[test]
    // fn test_ifd() {
    // }
}
