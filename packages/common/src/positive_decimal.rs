use std::{fmt::Display, str::FromStr};

use anyhow::Result;
use rust_decimal::Decimal;

use crate::unsigned_decimal::UnsignedDecimal;

/// A [Decimal] value which is strictly greater than 0.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct PositiveDecimal(Decimal);

impl PositiveDecimal {
    pub fn new(value: Decimal) -> Result<Self> {
        if value.is_sign_negative() || value.is_zero() {
            Err(anyhow::anyhow!(
                "PositiveDecimal::new: received a non-positive value"
            ))
        } else {
            Ok(PositiveDecimal(value))
        }
    }

    pub(crate) fn into_unsigned(self) -> UnsignedDecimal {
        UnsignedDecimal::new(self.0).expect("Impossible occured, PositiveDecimal::into_unsigned")
    }

    pub(crate) fn checked_sub(&self, rhs: PositiveDecimal) -> Result<Self> {
        anyhow::ensure!(self.0 >= rhs.0);
        Ok(PositiveDecimal(self.0 - rhs.0))
    }
}

impl Display for PositiveDecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for PositiveDecimal {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        PositiveDecimal::new(s.parse()?)
    }
}

impl std::ops::AddAssign for PositiveDecimal {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::ops::Div for PositiveDecimal {
    // Division could round down to 0, but never become negative.
    type Output = UnsignedDecimal;

    fn div(self, rhs: Self) -> Self::Output {
        UnsignedDecimal::new(self.0 / rhs.0)
            .expect("Dividing two positive decimals gave a negative result")
    }
}

impl TryFrom<UnsignedDecimal> for PositiveDecimal {
    type Error = anyhow::Error;

    fn try_from(value: UnsignedDecimal) -> Result<Self, Self::Error> {
        Self::new(value.into())
    }
}
