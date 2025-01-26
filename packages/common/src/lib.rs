mod asset;
mod messages;
mod positive_decimal;
mod price;
mod unsigned_decimal;

pub use asset::{Asset, Euro, PositiveAsset, UnsignedAsset, Usd};
pub use positive_decimal::PositiveDecimal;
pub use price::Price;
pub use rust_decimal::Decimal;
pub use unsigned_decimal::UnsignedDecimal;

/// Some amount of an asset
pub struct Amount<Asset> {
    pub amount: UnsignedDecimal,
    pub asset: Asset,
}
