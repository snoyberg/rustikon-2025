use std::{fmt::Display, marker::PhantomData};

use anyhow::Result;

use crate::{asset::PositiveAsset, positive_decimal::PositiveDecimal, Asset};

/// The price of the base asset in terms of the quote.
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
        write!(
            f,
            "{:0.02} {}/{}",
            self.price,
            Quote::as_str(),
            Base::as_str(),
        )
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

        let btc = PositiveAsset::from_static(Bitcoin, "0.5");
        let euro = PositiveAsset::from_static(Euro, "55000");
        let price = Price::from_asset_ratios(btc, euro).unwrap();
        assert_eq!(price.to_string(), "110000 EURO/BTC");
    }
}
