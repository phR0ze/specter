use std::{error::Error, fmt, path::Path};

use super::MetaError;

#[derive(Debug)]
#[non_exhaustive]
pub struct FileTypeError {
    pub path: Box<Path>,
    pub kind: FileTypeErrorKind,
}

// impl FileTypeError {
//     fn invalid_file_type<T: AsRef<Path>>(path: T) -> Self {
//         Self { path: path.as_ref() }
//     }
// }

impl fmt::Display for FileTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unknown file type {}", self.path.display())
    }
}

impl Error for FileTypeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            FileTypeErrorKind::Parse(e) => Some(e),
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum FileTypeErrorKind {
    #[non_exhaustive]
    Parse(MetaError),
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_metaerror() {
        // assert_eq!(FileTypeError("foo").to_string(), "foo");
    }
}
