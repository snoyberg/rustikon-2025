use crate::{Euro, PositiveAsset, Price, UnsignedAsset, Usd};

/// Name of an account owner
#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub struct Owner(pub String);

/// Messages that can be sent to the server
///
/// Note: using proper REST, gRPC, or Swagger would all be preferable.
/// Using this enum approach to demonstrate the power of serde for strong types.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServerRequest {
    /// Get the overall system status.
    ///
    /// Returns: [StatusResp]
    Status {},
    /// Get the balance for the given owner.
    ///
    /// Returns: [BalanceResp]
    Balance { owner: Owner },
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
pub struct StatusResp {
    pub total_usd: UnsignedAsset<Usd>,
    pub total_euro: UnsignedAsset<Euro>,
    pub price_usd: Price<Usd, Euro>,
    pub price_euro: Price<Euro, Usd>,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct BalanceResp {
    pub usd: UnsignedAsset<Usd>,
    pub euro: UnsignedAsset<Euro>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MintFundsResp {}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SellDollarsResp {
    ConversionSuccess { euros_bought: PositiveAsset<Euro> },
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SellEurosResp {
    ConversionSuccess { dollars_bought: PositiveAsset<Usd> },
}
