use std::{error::Error, fmt};

use super::{BaseError, ContextError};

#[derive(Debug)]
#[non_exhaustive]
pub struct DataError {
    pub kind: DataErrorKind,      // extensible error kind
    pub data: Option<Box<[u8]>>,  // optional data to include
    pub msg: Option<String>,      // optional error message to include
    source: Option<ContextError>, // optional extensible error chain
}

impl DataError {
    /// Add optional error message detail for output with the standard error messsage for this kind
    pub fn with_msg<T: AsRef<str>>(mut self, msg: T) -> Self {
        self.msg = Some(msg.as_ref().into());
        self
    }

    /// Add optional data for output with the error message
    pub fn with_data(mut self, data: &[u8]) -> Self {
        self.data = Some(data.into());
        self
    }
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            DataErrorKind::Ascii => write!(f, "ascii conversion failed")?,
            DataErrorKind::Downcast => write!(f, "downcast failed")?,
        };

        // Display additional messaging if available
        if let Some(msg) = self.msg.as_ref() {
            if !msg.is_empty() {
                write!(f, "{}", msg)?;
            };
        };
        if let Some(data) = self.data.as_ref() {
            if data.len() > 0 {
                write!(f, " {:02x?}", data)?;
            };
        };
        Ok(())
    }
}

impl Error for DataError {}
impl BaseError for DataError {}

// Provides a way to get the generic Error type
impl AsRef<dyn Error> for DataError {
    fn as_ref(&self) -> &(dyn Error + 'static) {
        self
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum DataErrorKind {
    Ascii,
    Downcast,
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_cast_error() {
        // assert_eq!(FileTypeError("foo").to_string(), "foo");
    }
}
