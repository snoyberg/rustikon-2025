use std::{fmt::Display, str::FromStr};

use anyhow::Result;
use rust_decimal::Decimal;

/// A [Decimal] value which is 0 or greater.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Clone, Copy)]
pub struct UnsignedDecimal(Decimal);

impl UnsignedDecimal {
    /// Smart constructor: guarantee invariants
    pub fn new(decimal: Decimal) -> Result<Self> {
        if decimal.is_sign_negative() {
            Err(anyhow::anyhow!("UnsignedDecimal::new with negative"))
        } else {
            Ok(UnsignedDecimal(decimal))
        }
    }

    pub(crate) fn zero() -> UnsignedDecimal {
        UnsignedDecimal(Decimal::ZERO)
    }

    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn into_decimal(self) -> Decimal {
        self.0
    }
}

impl From<UnsignedDecimal> for Decimal {
    fn from(value: UnsignedDecimal) -> Self {
        value.0
    }
}

impl TryFrom<Decimal> for UnsignedDecimal {
    type Error = anyhow::Error;

    fn try_from(value: Decimal) -> Result<Self> {
        Self::new(value)
    }
}

impl Display for UnsignedDecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for UnsignedDecimal {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        UnsignedDecimal::new(s.parse()?)
    }
}

impl std::ops::AddAssign for UnsignedDecimal {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl UnsignedDecimal {
    pub fn checked_sub_assign(&mut self, rhs: Self) -> Result<()> {
        anyhow::ensure!(
            self.0 >= rhs.0,
            "Cannot subtract for UnsignedDecimal: {} - {}",
            self.0,
            rhs.0
        );
        self.0 -= rhs.0;
        Ok(())
    }
}
