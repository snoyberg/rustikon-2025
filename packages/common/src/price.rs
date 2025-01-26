use std::{fmt::Display, marker::PhantomData, str::FromStr};

use anyhow::{Context, Result};
use serde::de::Visitor;

use crate::{
    asset::{split_amount_asset, PositiveAsset},
    positive_decimal::PositiveDecimal,
    Asset,
};

/// The price of the base asset in terms of the quote.
#[derive(PartialEq, Eq, Debug)]
pub struct Price<Base, Quote> {
    price: PositiveDecimal,
    _base: PhantomData<Base>,
    _quote: PhantomData<Quote>,
}

impl<Base, Quote> Price<Base, Quote> {
    pub fn from_asset_ratios(
        base: PositiveAsset<Base>,
        quote: PositiveAsset<Quote>,
    ) -> Result<Price<Base, Quote>> {
        let price = quote.get_value() / base.get_value();
        Ok(Price {
            price: price.try_into()?,
            _base: PhantomData,
            _quote: PhantomData,
        })
    }
}

impl<Base: Asset, Quote: Asset> Display for Price<Base, Quote> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}/{}", self.price, Quote::as_str(), Base::as_str(),)
    }
}

impl<Base: Asset, Quote: Asset> FromStr for Price<Base, Quote> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (price, asset_pair) = split_amount_asset(s)?;
        let price = price.parse()?;
        let (quote, base) = asset_pair
            .trim()
            .split_once('/')
            .context("No slash found in assets")?;
        if Base::as_str() != base {
            Err(anyhow::anyhow!(
                "Parsing price {s}, mismatched base asset, expected {} but found {base}",
                Base::as_str()
            ))
        } else if Quote::as_str() != quote {
            Err(anyhow::anyhow!(
                "Parsing price {s}, mismatched quote asset, expected {} but found {quote}",
                Quote::as_str()
            ))
        } else {
            Ok(Price {
                price,
                _base: PhantomData,
                _quote: PhantomData,
            })
        }
    }
}

impl<Base: Asset, Quote: Asset> serde::Serialize for Price<Base, Quote> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de, Base: Asset, Quote: Asset> serde::Deserialize<'de> for Price<Base, Quote> {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(PriceVisitor(PhantomData, PhantomData))
    }
}

struct PriceVisitor<Base, Quote>(PhantomData<Base>, PhantomData<Quote>);

impl<Base: Asset, Quote: Asset> Visitor<'_> for PriceVisitor<Base, Quote> {
    type Value = Price<Base, Quote>;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Price of {} in terms of {}",
            Base::as_str(),
            Quote::as_str()
        )
    }

    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        v.parse().map_err(E::custom)
    }
}

#[cfg(test)]
mod tests {
    use asset::Bitcoin;

    use crate::*;

    #[test]
    fn price_display_is_correct() {
        let usd = PositiveAsset::from_static(Usd, "11000");
        let euro = PositiveAsset::from_static(Euro, "10000");
        let price = Price::from_asset_ratios(euro, usd).unwrap();
        assert_eq!(price.to_string(), "1.10 USD/EURO");
        assert_eq!(price, price.to_string().parse().unwrap());

        let json = serde_json::to_string(&price).unwrap();
        let price2: Price<Euro, Usd> = serde_json::from_str(&json).unwrap();
        assert_eq!(price, price2);
        serde_json::from_str::<Price<Usd, Euro>>(&json).unwrap_err();

        let btc = PositiveAsset::from_static(Bitcoin, "0.5");
        let euro = PositiveAsset::from_static(Euro, "55000");
        let price = Price::from_asset_ratios(btc, euro).unwrap();
        assert_eq!(price.to_string(), "110000 EURO/BTC");
    }
}
