use std::{any::Any, fmt, io};

use super::Kind;
use crate::{
    errors::{CastError, ParseError},
    formats::Jpeg,
};

/// Extensible meta trait to encapsulate various media file types.
pub trait Meta: fmt::Debug {
    /// Create a new instance of the meta type
    fn new() -> Self;

    /// Discover the available meta data types for this media format
    fn discover(reader: &mut impl io::Read) -> Result<(), ParseError>;

    /// Return the meta type as a generic Any type for downcasting
    fn as_any(&self) -> &dyn Any;

    /// Return the kind of media file were working with
    fn kind(&self) -> Kind;

    /// Return the concrete jpeg type or an error if the cast fails
    fn as_jpeg(&self) -> Result<&Jpeg, CastError>;
}
