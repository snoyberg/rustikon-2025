use std::time::Duration;

use crate::prelude::*;

pub fn status() -> QueryScope<(), Result<StatusResp>> {
    create_query(
        |()| perform_server_request(ServerRequest::Status {}),
        QueryOptions::default().set_refetch_interval(Some(Duration::from_secs(3))),
    )
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct OwnerBalance {
    pub owner: Owner,
    pub dollars: UnsignedAsset<Usd>,
    pub euros: UnsignedAsset<Euro>,
}

pub fn owners() -> QueryScope<(), Result<Vec<OwnerBalance>>> {
    create_query(
        |()| owners_fetcher(),
        QueryOptions::default().set_refetch_interval(Some(Duration::from_secs(3))),
    )
}

async fn owners_fetcher() -> Result<Vec<OwnerBalance>> {
    let mut v = vec![];
    let mut start_after = None;
    loop {
        let ListOwnersResp { owners } = perform_server_request(ServerRequest::ListOwners {
            start_after: start_after.take(),
        })
        .await?;
        match owners.last() {
            None => break Ok(v),
            Some(last) => start_after = Some(last.clone()),
        }
        for owner in owners {
            let BalanceResp { usd, euro } = perform_server_request(ServerRequest::Balance {
                owner: owner.clone(),
            })
            .await?;
            v.push(OwnerBalance {
                owner,
                dollars: usd,
                euros: euro,
            })
        }
    }
}

pub async fn mint_funds(
    owner: Owner,
    dollars: UnsignedAsset<Usd>,
    euros: UnsignedAsset<Euro>,
    set_status: WriteSignal<Option<String>>,
) {
    set_status.set(Some("Asking server to mint funds...".to_owned()));
    match mint_funds_inner(owner, dollars, euros).await {
        Ok(()) => set_status.set(Some("Funds minted successfully".to_owned())),
        Err(e) => set_status.set(Some(format!("Error while minting funds: {e}"))),
    }
    set_timeout(move || set_status.set(None), Duration::from_secs(5));
}

async fn mint_funds_inner(
    owner: Owner,
    dollars: UnsignedAsset<Usd>,
    euros: UnsignedAsset<Euro>,
) -> Result<()> {
    let MintFundsResp {} = perform_server_request(ServerRequest::MintFunds {
        recipient: owner,
        usd_amount: dollars,
        euro_amount: euros,
    })
    .await?;
    Ok(())
}

pub enum ToSell {
    Dollars(PositiveAsset<Usd>),
    Euros(PositiveAsset<Euro>),
}

impl ToSell {
    fn desc(&self) -> &'static str {
        match self {
            ToSell::Dollars(_) => "dollars",
            ToSell::Euros(_) => "euros",
        }
    }
}

pub async fn sell_asset(owner: Owner, to_sell: ToSell, set_status: WriteSignal<Option<String>>) {
    set_status.set(Some(format!("Asking server to sell {}...", to_sell.desc())));
    match sell_asset_inner(owner, to_sell).await {
        Ok(msg) => set_status.set(Some(msg)),
        Err(e) => set_status.set(Some(format!("Error while selling funds: {e}"))),
    }
    set_timeout(move || set_status.set(None), Duration::from_secs(5));
}

async fn sell_asset_inner(owner: Owner, to_sell: ToSell) -> Result<String> {
    match to_sell {
        ToSell::Dollars(dollars) => {
            let SellDollarsResp { euros_bought } =
                perform_server_request(ServerRequest::SellDollars {
                    trader: owner,
                    dollars,
                })
                .await?;
            Ok(format!("Sold {dollars} for {euros_bought}"))
        }
        ToSell::Euros(euros) => {
            let SellEurosResp { dollars_bought } =
                perform_server_request(ServerRequest::SellEuros {
                    trader: owner,
                    euros,
                })
                .await?;
            Ok(format!("Sold {euros} for {dollars_bought}"))
        }
    }
}

async fn perform_server_request<Resp: serde::de::DeserializeOwned>(
    req: ServerRequest,
) -> Result<Resp> {
    let req = serde_json::to_string(&req).map_err(Error::from_other_error)?;
    let res = reqwasm::http::Request::post("http://localhost:3001")
        .header("content-type", "application/json")
        .body(req)
        .send()
        .await
        .map_err(Error::from_other_error)?;
    if res.status() != 200 {
        return Err(Error::HttpRequestFailure {
            status: res.status(),
        });
    }
    res.json().await.map_err(Error::from_other_error)
}
