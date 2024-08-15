use crate::{Meta, MetaKind};

// Custom nom error type
//type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(Debug)]
pub(crate) struct Jpeg;

impl Jpeg {
    pub fn new() -> Self {
        Self
    }
}

impl Meta for Jpeg {
    fn kind(&self) -> MetaKind {
        MetaKind::Jpeg
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
