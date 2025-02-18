use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

pub use private::PositiveDecimal;

mod private {
    use crate::UnsignedDecimal;

    /// A version of [UnsignedDecimal] which disallows the value 0.
    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct PositiveDecimal {
        // Invariant: can never be 0
        value: UnsignedDecimal,
    }

    impl PositiveDecimal {
        pub(crate) fn from_raw_value(value: UnsignedDecimal) -> anyhow::Result<Self> {
            anyhow::ensure!(value.get_raw_value() != 0, "PositiveDecimal cannot be 0");
            Ok(PositiveDecimal { value })
        }
        pub(crate) fn get_raw_value(&self) -> UnsignedDecimal {
            self.value
        }
    }
}

impl FromStr for PositiveDecimal {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().and_then(PositiveDecimal::from_raw_value)
    }
}

impl Display for PositiveDecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get_raw_value())
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
        PositiveDecimal::from_raw_value(self.get_raw_value() + rhs.get_raw_value()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_addition() {
        let x: PositiveDecimal = "-2.2".parse().unwrap();
        let y: PositiveDecimal = "3.5".parse().unwrap();
        let z = x + y;
        assert_eq!(z.to_string(), "1.3");
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
