use std::fmt::Display;

use crate::{errors::ExifError, Endian, ExifResult};

#[derive(Debug, PartialEq)]
pub(crate) struct Rational {
    pub(crate) num: u32, // numerator
    pub(crate) den: u32, // denominator
}

impl Rational {
    pub(crate) fn new(num: u32, den: u32) -> Self {
        Self { num, den }
    }

    pub(crate) fn try_from(val: &[u8], endian: Endian) -> ExifResult<Self> {
        if val.len() < 8 {
            return Err(ExifError::parse(": rational must be 8 bytes long"));
        }
        match endian {
            Endian::Little => Ok(Self {
                num: u32::from_le_bytes(val[0..4].try_into().unwrap()),
                den: u32::from_le_bytes(val[4..8].try_into().unwrap()),
            }),
            Endian::Big => Ok(Self {
                num: u32::from_be_bytes(val[0..4].try_into().unwrap()),
                den: u32::from_be_bytes(val[4..8].try_into().unwrap()),
            }),
        }
    }
}

impl Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.den {
            1 => write!(f, "{}", self.num), // common understanding is out of 1
            _ => write!(f, "{}/{}", self.num, self.den),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rational_not_enough_data() {
        let err = Rational::try_from(&[][..], Endian::Big).unwrap_err();
        assert_eq!(err.to_string(), "Exif parse failed: rational must be 8 bytes long".to_string());
    }

    #[test]
    fn test_rational_le_success() {
        let r = Rational::try_from(
            &[0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00][..],
            Endian::Little,
        )
        .unwrap();
        assert_eq!(r.num, 1);
        assert_eq!(r.den, 2);
    }

    #[test]
    fn test_rational_be_success() {
        let r =
            Rational::try_from(&[0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02][..], Endian::Big)
                .unwrap();
        assert_eq!(r.num, 1);
        assert_eq!(r.den, 2);
    }
}
