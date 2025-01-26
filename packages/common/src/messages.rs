use crate::{Euro, PositiveAsset, UnsignedAsset, Usd};

/// Name of an account owner
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Owner(pub String);

/// Messages that can be sent to the server
///
/// Note: using proper REST, gRPC, or Swagger would all be preferable.
/// Using this enum approach to demonstrate the power of serde for strong types.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServerRequest {
    /// Create new funds for a user.
    ///
    /// Returns: [MintFundsResp]
    MintFunds {
        recipient: Owner,
        usd_amount: UnsignedAsset<Usd>,
        euro_amount: UnsignedAsset<Euro>,
    },
    /// Convert dollars into euros
    ///
    /// Returns: [SellDollarsResp]
    SellDollars {
        trader: Owner,
        dollars: PositiveAsset<Usd>,
    },
    /// Convert euros into dollars
    ///
    /// Returns: [SellEurosResp]
    SellEuros {
        trader: Owner,
        euros: PositiveAsset<Euro>,
    },
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MintFundsResp {}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SellDollarsResp {
    ConversionSuccess { euros_bought: PositiveAsset<Euro> },
}
