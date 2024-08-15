use std::fmt;

/// Extensible meta trait to encapsulate various media file types.
pub trait Meta: fmt::Debug {
    /// Return the kind of media file were working with
    fn kind(&self) -> MetaKind;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetaKind {
    Jpeg,
}
