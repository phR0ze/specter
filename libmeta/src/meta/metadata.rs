use std::fmt;

/// Extensible meta data trait to encapsulate various meta data types
pub trait MetaData: fmt::Debug {
    /// Return the kind of meta data we're working with
    fn kind(&self) -> MetaDataKind;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetaDataKind {
    Exif,
    Jfif,
    Spiff,
}
