use std::fmt::Display;

use crate::{Euro, PositiveAsset, Price, UnsignedAsset, Usd};

/// Name of an account owner
#[derive(
    serde::Serialize, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug,
)]
pub struct Owner(pub String);

impl Display for Owner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

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
    /// Enumerate owners of dollars and euros
    ListOwners {
        /// The last owner seen, if any.
        start_after: Option<Owner>,
    },
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct StatusResp {
    /// Total amount of USD in both the pool and held by all users.
    pub total_usd: UnsignedAsset<Usd>,
    /// Total amount of EURO in both the pool and held by all users.
    pub total_euro: UnsignedAsset<Euro>,
    /// Price of a single USD in terms of EURO
    pub price_usd: Price<Usd, Euro>,
    /// Price of a single EURO in terms of USD
    pub price_euro: Price<Euro, Usd>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct BalanceResp {
    pub usd: UnsignedAsset<Usd>,
    pub euro: UnsignedAsset<Euro>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MintFundsResp {}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct SellDollarsResp {
    pub euros_bought: PositiveAsset<Euro>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct SellEurosResp {
    pub dollars_bought: PositiveAsset<Usd>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct ListOwnersResp {
    pub owners: Vec<Owner>,
}
