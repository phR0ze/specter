use std::{error::Error, fmt};

use super::{BaseError, ContextError};

#[derive(Debug)]
#[non_exhaustive] // allow for future error fields
pub struct JfifError {
    pub kind: JfifErrorKind,      // extensible kind messaging
    data: Option<Box<[u8]>>,      // additional error data
    msg: Option<String>,          // optional error message to include
    source: Option<ContextError>, // optional extensible source error
}

impl JfifError {
    pub fn new(kind: JfifErrorKind) -> Self {
        Self { kind, data: None, msg: None, source: None }
    }

    fn with_kind(kind: JfifErrorKind) -> Self {
        Self { kind, data: None, msg: None, source: None }
    }

    /// Get the error data
    pub fn data(&self) -> Option<&[u8]> {
        match &self.data {
            Some(data) => Some(data.as_ref()),
            None => None,
        }
    }

    /// Get the error kind
    pub fn kind(&self) -> &JfifErrorKind {
        &self.kind
    }

    /// Create a new error for a failed operation
    pub fn parse<T: AsRef<str>>(msg: T) -> Self {
        JfifError::with_kind(JfifErrorKind::Parse).with_msg(msg)
    }

    /// Add additional error data for output with the error message
    pub(crate) fn with_data(mut self, data: &[u8]) -> Self {
        self.data = Some(data.into());
        self
    }

    /// Add optional error message detail for output with the standard error messsage for this kind
    pub(crate) fn with_msg<T: AsRef<str>>(mut self, msg: T) -> Self {
        self.msg = Some(msg.as_ref().into());
        self
    }

    // Add a nom source error and override the kind in particular cases
    pub fn with_nom_source(self, source: nom::Err<nom::error::Error<&[u8]>>) -> Self {
        self.with_source("nom::", source)
    }

    // Add an optional source error
    pub fn with_source<T: Error>(mut self, kind: &str, source: T) -> Self {
        self.source = Some(ContextError::from(kind, source));
        self
    }

    // Add an optional source error
    pub fn wrap<T: Error>(mut self, source: T) -> Self {
        self.source = Some(ContextError::from("", source));
        self
    }
}

impl BaseError for JfifError {}

impl fmt::Display for JfifError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            JfifErrorKind::Parse => write!(f, "JFIF parse failed")?,
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
impl Error for JfifError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.source {
            Some(source) => Some(source),
            None => None,
        }
    }
}

// Provides a way to get the generic Error type
impl AsRef<dyn Error> for JfifError {
    fn as_ref(&self) -> &(dyn Error + 'static) {
        self
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum JfifErrorKind {
    Parse,
}

#[cfg(test)]
mod tests {

    // #[test]
    // fn test_jpeg_use_as_result() {
    //     assert_eq!(
    //         jpeg_error_as_result().unwrap_err().to_string(),
    //         "JPEG segment marker invalid [00, 01]"
    //     );
    // }
}
