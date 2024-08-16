use std::{error::Error, fmt, path::Path};

#[derive(Debug)]
#[non_exhaustive]
pub struct CastError {
    pub msg: String,
}

impl CastError {
    pub fn new<T: AsRef<str>>(msg: T) -> Self {
        Self {
            msg: msg.as_ref().to_string(),
        }
    }
}

impl fmt::Display for CastError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "downcast failed for {}", self.msg)
    }
}

impl Error for CastError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cast_error() {
        // assert_eq!(FileTypeError("foo").to_string(), "foo");
    }
}
