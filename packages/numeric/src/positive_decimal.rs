use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use anyhow::Result;
pub use private::PositiveDecimal;

mod private {
    use anyhow::Result;

    use crate::UnsignedDecimal;

    /// A version of [UnsignedDecimal] which disallows the value 0.
    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct PositiveDecimal {
        // Invariant: can never be 0
        value: UnsignedDecimal,
    }

    impl PositiveDecimal {
        /// Generate a new value, checking that the input is not 0.
        pub fn new(value: UnsignedDecimal) -> Result<Self> {
            anyhow::ensure!(value.get_raw_value() != 0, "PositiveDecimal cannot be 0");
            Ok(PositiveDecimal { value })
        }

        /// Get the raw unsigned value.
        pub fn get_unsigned(&self) -> UnsignedDecimal {
            self.value
        }
    }
}

impl FromStr for PositiveDecimal {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().and_then(PositiveDecimal::new)
    }
}

impl Display for PositiveDecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get_unsigned())
    }
}

impl Debug for PositiveDecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl std::ops::Add for PositiveDecimal {
    type Output = PositiveDecimal;

    fn add(self, rhs: Self) -> Self::Output {
        PositiveDecimal::new(self.get_unsigned() + rhs.get_unsigned()).unwrap()
    }
}

impl std::ops::AddAssign for PositiveDecimal {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl PositiveDecimal {
    pub fn checked_sub(self, rhs: Self) -> Result<Self> {
        self.get_unsigned()
            .checked_sub(rhs.get_unsigned())
            .and_then(PositiveDecimal::new)
    }
}

impl std::ops::Div for PositiveDecimal {
    type Output = PositiveDecimal;

    fn div(self, rhs: Self) -> Self::Output {
        PositiveDecimal::new(self.get_unsigned() / rhs.get_unsigned()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_addition() {
        let x: PositiveDecimal = "2.2".parse().unwrap();
        let y: PositiveDecimal = "3.5".parse().unwrap();
        let z = x + y;
        assert_eq!(z.to_string(), "5.7");
        let z2: PositiveDecimal = z.to_string().parse().unwrap();
        assert_eq!(z, z2);
    }

    #[test]
    fn test_parse() {
        PositiveDecimal::from_str("5.2").unwrap();
        PositiveDecimal::from_str("-5.2").unwrap_err();
        PositiveDecimal::from_str("-0").unwrap_err();
        PositiveDecimal::from_str("0").unwrap_err();
        PositiveDecimal::from_str("0.0").unwrap_err();
    }
}
