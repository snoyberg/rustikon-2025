use std::{fmt::Display, marker::PhantomData, str::FromStr};

use anyhow::{Context, Result};
use numeric::{PositiveDecimal, UnsignedDecimal};

/// Any type that represents an asset type.
pub trait Asset: Ord + std::fmt::Debug + Default {
    fn as_str() -> &'static str;
}

macro_rules! make_asset {
    ($i:ident, $name:expr) => {
        #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Default, Clone, Copy)]
        pub struct $i;
        impl Asset for $i {
            fn as_str() -> &'static str {
                $name
            }
        }
    };
}

make_asset!(Usd, "USD");
make_asset!(Euro, "EURO");
// Not needed, just for fun
make_asset!(Bitcoin, "BTC");

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PositiveAsset<I> {
    value: PositiveDecimal,
    _phantom: PhantomData<I>,
}

// Avoid having an unnecessary Clone/Copy bound on I.
impl<I> Clone for PositiveAsset<I> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<I> Copy for PositiveAsset<I> {}

impl<T> PositiveAsset<T> {
    pub fn new(_: T, value: PositiveDecimal) -> PositiveAsset<T> {
        PositiveAsset {
            value,
            _phantom: PhantomData,
        }
    }

    pub fn from_static(_: T, value: &'static str) -> PositiveAsset<T> {
        let value = value.parse().unwrap();
        PositiveAsset {
            value,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn get_value(&self) -> PositiveDecimal {
        self.value
    }

    pub fn into_unsigned(&self) -> UnsignedAsset<T> {
        UnsignedAsset::new_no_hints(self.value.get_unsigned())
    }

    pub fn checked_sub(&self, rhs: PositiveAsset<T>) -> Result<Self> {
        Ok(PositiveAsset {
            value: self.value.checked_sub(rhs.value)?,
            _phantom: PhantomData,
        })
    }
}

impl<T: Asset> serde::Serialize for PositiveAsset<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de, T: Asset> serde::Deserialize<'de> for PositiveAsset<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(PositiveAssetVisitor(PhantomData))
    }
}

struct PositiveAssetVisitor<T>(PhantomData<T>);

impl<T: Asset> serde::de::Visitor<'_> for PositiveAssetVisitor<T> {
    type Value = PositiveAsset<T>;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Positive asset {}", T::as_str())
    }

    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        v.parse().map_err(E::custom)
    }
}

impl<T: Asset> Display for PositiveAsset<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.value, T::as_str())
    }
}

impl<T: Asset> FromStr for PositiveAsset<T> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (value, asset) = split_amount_asset(s)?;
        anyhow::ensure!(
            asset == T::as_str(),
            "Unexpected asset string {asset} found, expected {}",
            T::as_str()
        );
        let value = value.parse()?;
        Ok(PositiveAsset {
            value,
            _phantom: PhantomData,
        })
    }
}

pub(crate) fn split_amount_asset(s: &str) -> Result<(&str, &str)> {
    let idx = s
        .find(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-'))
        .context("No asset type found")?;
    Ok(s.split_at(idx))
}

impl<T: Asset> std::ops::AddAssign for PositiveAsset<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Clone)]
pub struct UnsignedAsset<T> {
    value: UnsignedDecimal,
    _phantom: PhantomData<T>,
}

impl<T> UnsignedAsset<T> {
    pub fn new(_: T, value: UnsignedDecimal) -> Self {
        UnsignedAsset {
            value,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn new_no_hints(value: UnsignedDecimal) -> Self {
        UnsignedAsset {
            value,
            _phantom: PhantomData,
        }
    }

    pub fn zero(_: T) -> Self {
        UnsignedAsset {
            value: UnsignedDecimal::zero(),
            _phantom: PhantomData,
        }
    }

    pub fn into_decimal(self) -> UnsignedDecimal {
        self.value
    }
}

impl<T: Asset> serde::Serialize for UnsignedAsset<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de, T: Asset> serde::Deserialize<'de> for UnsignedAsset<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(UnsignedAssetVisitor(PhantomData))
    }
}

struct UnsignedAssetVisitor<T>(PhantomData<T>);

impl<T: Asset> serde::de::Visitor<'_> for UnsignedAssetVisitor<T> {
    type Value = UnsignedAsset<T>;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Unsigned asset {}", T::as_str())
    }

    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        v.parse().map_err(E::custom)
    }
}

impl<T: Asset> Display for UnsignedAsset<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.value, T::as_str())
    }
}

impl<T: Asset> FromStr for UnsignedAsset<T> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (value, asset) = split_amount_asset(s)?;
        anyhow::ensure!(
            asset == T::as_str(),
            "Unexpected asset string {asset} found, expected {}",
            T::as_str()
        );
        let value = value.parse()?;
        Ok(UnsignedAsset {
            value,
            _phantom: PhantomData,
        })
    }
}

impl<T: Asset> std::ops::AddAssign for UnsignedAsset<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use asset::split_amount_asset;

    use crate::*;

    #[test]
    fn split_amount() {
        assert_eq!(split_amount_asset("5USD").unwrap(), ("5", "USD"));
        assert_eq!(split_amount_asset("5.0USD").unwrap(), ("5.0", "USD"));
        split_amount_asset("25").unwrap_err();
        split_amount_asset("25.0").unwrap_err();
        split_amount_asset("25.").unwrap_err();
    }

    #[test]
    fn positive_asset_render_and_parse() {
        let usd = PositiveAsset::from_static(Usd, "123.456");
        let s = usd.to_string();
        assert_eq!(s, "123.456USD");
        let usd2: PositiveAsset<Usd> = s.parse().unwrap();
        assert_eq!(usd, usd2);
    }

    #[test]
    fn positive_asset_serde() {
        let usd = PositiveAsset::from_static(Usd, "123.456");
        let s = serde_json::to_string(&usd).unwrap();
        assert_eq!(s, r#""123.456USD""#);
        let usd2: PositiveAsset<Usd> = serde_json::from_str(&s).unwrap();
        assert_eq!(usd, usd2);
    }

    #[test]
    fn invalid_asset_values() {
        serde_json::from_str::<PositiveAsset<Usd>>("\"0.1USD\"").unwrap();
        serde_json::from_str::<PositiveAsset<Usd>>("\"0.aUSD\"").unwrap_err();
        serde_json::from_str::<PositiveAsset<Usd>>("\"0USD\"").unwrap_err();
    }

    #[test]
    fn parse_negative_unsigned_asset() {
        UnsignedAsset::<Usd>::from_str("-5000USD").unwrap_err();
        assert_eq!(
            UnsignedAsset::<Usd>::from_str("5000USD").unwrap(),
            UnsignedAsset::new(Usd, "5000".parse().unwrap())
        )
    }
}
