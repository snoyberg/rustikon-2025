mod asset;
mod messages;
mod price;

pub use asset::{Asset, Euro, PositiveAsset, UnsignedAsset, Usd};
pub use messages::{
    BalanceResp, MintFundsResp, Owner, SellDollarsResp, SellEurosResp, ServerRequest, StatusResp,
};
pub use numeric::{PositiveDecimal, SignedDecimal, UnsignedDecimal};
pub use price::Price;

/// Some amount of an asset
pub struct Amount<Asset> {
    pub amount: UnsignedDecimal,
    pub asset: Asset,
}
