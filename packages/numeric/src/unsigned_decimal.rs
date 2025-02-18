use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

pub use private::UnsignedDecimal;

mod private {
    /// Stored with 6 digits of precision
    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct UnsignedDecimal {
        value: u128,
    }

    impl UnsignedDecimal {
        pub(crate) fn from_raw_value(value: u128) -> Self {
            UnsignedDecimal { value }
        }
        pub(crate) fn to_raw_value(&self) -> u128 {
            self.value
        }
    }
}

const MULTIPLIER: u128 = 1_000_000;

impl FromStr for UnsignedDecimal {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (whole, fraction) = match s.split_once('.') {
            None => (s.parse()?, 0),
            Some((whole, fraction)) => {
                let whole: u128 = whole.parse()?;
                let fraction = parse_fraction(fraction)?;
                (whole, fraction)
            }
        };

        let value = whole * MULTIPLIER + fraction;

        Ok(UnsignedDecimal::from_raw_value(value))
    }
}

impl Display for UnsignedDecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let value = self.to_raw_value();
        let whole = value / MULTIPLIER;
        let mut fraction = value % MULTIPLIER;
        if fraction == 0 {
            write!(f, "{whole}")
        } else {
            while fraction > 0 && fraction % 10 == 0 {
                fraction /= 10;
            }
            write!(f, "{whole}.{fraction}")
        }
    }
}

impl Debug for UnsignedDecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl std::ops::Add for UnsignedDecimal {
    type Output = UnsignedDecimal;

    fn add(self, rhs: Self) -> Self::Output {
        UnsignedDecimal::from_raw_value(self.to_raw_value() + rhs.to_raw_value())
    }
}

fn parse_fraction(s: &str) -> anyhow::Result<u128> {
    anyhow::ensure!(
        s.len() <= 6,
        "Unsigned decimal only supports up to 6 decimal points"
    );
    let mut x = s.parse()?;
    for _ in s.len()..6 {
        x *= 10;
    }
    Ok(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_addition() {
        let x: UnsignedDecimal = "2.2".parse().unwrap();
        let y: UnsignedDecimal = "3.5".parse().unwrap();
        let z = x + y;
        assert_eq!(z.to_string(), "5.7");
        let z2: UnsignedDecimal = z.to_string().parse().unwrap();
        assert_eq!(z, z2);
    }

    #[test]
    fn test_parse_fraction() {
        assert_eq!(parse_fraction("0").unwrap(), 0);
        assert_eq!(parse_fraction("123456").unwrap(), 123456);
        parse_fraction("1234567").unwrap_err();
        parse_fraction("12345678").unwrap_err();
        assert_eq!(parse_fraction("12345").unwrap(), 123450);
        parse_fraction("").unwrap_err();
    }

    #[test]
    fn test_basic_parse() {
        UnsignedDecimal::from_str("5.6").unwrap();
        UnsignedDecimal::from_str("5.0").unwrap();
        UnsignedDecimal::from_str("5.").unwrap_err();
        UnsignedDecimal::from_str("5").unwrap();
    }

    #[test]
    fn test_whole_numbers() {
        let x: UnsignedDecimal = "5".parse().unwrap();
        assert_eq!(x.to_string(), "5");

        let x: UnsignedDecimal = "0".parse().unwrap();
        assert_eq!(x.to_string(), "0");
    }

    #[test]
    fn test_debug() {
        for s in ["5", "5.2", "7.1", "0"] {
            let x: UnsignedDecimal = s.parse().unwrap();
            assert_eq!(format!("{x:?}"), s);
        }
    }
}
