#[derive(Debug, Clone)]
pub(crate) struct Ifd {
    // pub(crate) tag: u16,              // type of data
}

impl Default for Ifd {
    fn default() -> Self {
        Self {}
    }
}

// impl Ifd {
//     // Create a new IFD file
//     pub(crate) fn new(tag: u16, format: u16, components: u32) -> Self {
//         Self {
//             tag,
//             format,
//             components,
//             offset: None,
//             data: None,
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_ifd() {
    // }
}
