use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

pub use private::SignedDecimal;

use crate::UnsignedDecimal;

mod private {
    use crate::UnsignedDecimal;

    /// A signed version of [UnsignedDecimal]
    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct SignedDecimal {
        value: UnsignedDecimal,
        // Invariant: negative must be false whenever value is 0
        negative: bool,
    }

    impl SignedDecimal {
        pub(crate) fn from_raw_value(value: UnsignedDecimal, negative: bool) -> Self {
            SignedDecimal {
                value,
                negative: negative && value.get_raw_value() != 0,
            }
        }
        pub(crate) fn get_raw_value(&self) -> UnsignedDecimal {
            self.value
        }
        pub(crate) fn is_negative(&self) -> bool {
            self.negative
        }
    }
}

impl SignedDecimal {
    pub fn negate(self) -> SignedDecimal {
        SignedDecimal::from_raw_value(self.get_raw_value(), !self.is_negative())
    }
}

impl FromStr for SignedDecimal {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.strip_prefix("-") {
            Some(s) => {
                let value: UnsignedDecimal = s.parse()?;
                anyhow::ensure!(value.get_raw_value() != 0, "Cannot have a negative zero");
                Ok(SignedDecimal::from_raw_value(value, true))
            }
            None => Ok(SignedDecimal::from_raw_value(s.parse()?, false)),
        }
    }
}

impl Display for SignedDecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_negative() {
            write!(f, "-")?;
        }
        write!(f, "{}", self.get_raw_value())
    }
}

impl Debug for SignedDecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl std::ops::Add for SignedDecimal {
    type Output = SignedDecimal;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.is_negative(), rhs.is_negative()) {
            (false, false) => {
                SignedDecimal::from_raw_value(self.get_raw_value() + rhs.get_raw_value(), false)
            }
            (true, true) => {
                SignedDecimal::from_raw_value(self.get_raw_value() + rhs.get_raw_value(), true)
            }
            (false, true) => self - rhs.negate(),
            (true, false) => rhs - self.negate(),
        }
    }
}

impl std::ops::Sub for SignedDecimal {
    type Output = SignedDecimal;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self.is_negative(), rhs.is_negative()) {
            (false, false) => {
                let x = self.get_raw_value();
                let y = rhs.get_raw_value();
                if x > y {
                    SignedDecimal::from_raw_value(x.checked_sub(y).unwrap(), false)
                } else {
                    SignedDecimal::from_raw_value(y.checked_sub(x).unwrap(), true)
                }
            }
            (false, true) => self + rhs.negate(),
            (true, false) => (self.negate() - rhs).negate(),
            (true, true) => (self.negate() + rhs.negate()).negate(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_addition() {
        let x: SignedDecimal = "-2.2".parse().unwrap();
        let y: SignedDecimal = "3.5".parse().unwrap();
        let z = x + y;
        assert_eq!(z.to_string(), "1.3");
        let z2: SignedDecimal = z.to_string().parse().unwrap();
        assert_eq!(z, z2);
    }
}
