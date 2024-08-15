use std::io;

use super::Meta;
use crate::errors::ParseError;

// Custom nom error type
//type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(Debug)]
pub struct Jpeg;

impl Jpeg {
    pub fn new() -> Self {
        Self
    }
}

impl Meta for Jpeg {
    fn exif(&self) {
        println!("Hello, world!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        //
    }
}
