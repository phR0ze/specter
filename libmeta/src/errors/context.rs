use std::{error::Error, fmt};

#[derive(Debug)]
#[non_exhaustive]
pub struct ContextError {
    prefix: String,
    msg: String,
    source: Option<Box<ContextError>>,
}

impl ContextError {
    /// Convert the given error into an `ContextError` or chain of `ContextError`s
    pub fn from<T: Error>(prefix: &str, err: T) -> Self {
        let under = Self {
            prefix: prefix.into(),
            msg: err.to_string(),
            source: if let Some(err) = err.source() {
                Some(Self::from(prefix, err).into())
            } else {
                None
            },
        };

        under
    }
}

impl fmt::Display for ContextError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.prefix, self.msg)
    }
}

impl Error for ContextError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.source {
            Some(source) => Some(source),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_context_error() {
        assert_eq!(
            ContextError::from("io::Error: ", io::Error::from(io::ErrorKind::NotFound)).to_string(),
            "io::Error: entity not found"
        );
    }
}
