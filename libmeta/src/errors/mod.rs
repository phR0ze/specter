use std::error::Error;

mod cast;
mod context;
mod exif;
mod filetype;
mod jfif;
mod jpeg;
mod meta;

// Export all error types together
pub use cast::*;
pub use context::*;
pub use core::*;
pub use exif::*;
pub use filetype::*;
pub use jfif::*;
pub use jpeg::*;
pub use meta::*;

pub trait BaseError: Error + AsRef<dyn Error> {
    fn all_to_string(&self) -> String {
        let mut errs: Vec<String> = Vec::new();
        errs.push(self.to_string());
        errs.push(self.source_to_string());
        errs.join(" ==> ")
    }

    fn source_to_string(&self) -> String {
        let mut errs: Vec<String> = Vec::new();
        let mut err = self.as_ref();
        loop {
            match err.source() {
                Some(e) => {
                    errs.push(e.to_string());
                    err = e;
                }
                None => break,
            }
        }
        errs.join(" ==> ")
    }
}
