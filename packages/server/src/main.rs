use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use common::{
    BalanceResp, Euro, MintFundsResp, Owner, PositiveAsset, PositiveDecimal, Price,
    SellDollarsResp, SellEurosResp, ServerRequest, StatusResp, UnsignedAsset, UnsignedDecimal, Usd,
};
use parking_lot::Mutex;

#[derive(Clone)]
struct AppState(Arc<Mutex<AppStateInner>>);

struct AppStateInner {
    accounts: HashMap<Owner, Balances>,
    pool_usd: PositiveAsset<Usd>,
    pool_euro: PositiveAsset<Euro>,
}

#[derive(Default)]
struct Balances {
    usd: UnsignedDecimal,
    euro: UnsignedDecimal,
}

#[tokio::main]
async fn main() {
    let app_state = AppState(Arc::new(Mutex::new(AppStateInner {
        accounts: HashMap::new(),
        pool_usd: "103000USD".parse().unwrap(),
        pool_euro: "100000EURO".parse().unwrap(),
    })));
    let app = Router::new()
        .route("/", post(handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler(State(app): State<AppState>, Json(req): Json<ServerRequest>) -> impl IntoResponse {
    match handler_inner(&app, req).await {
        Ok(res) => res,
        Err(e) => {
            let mut res = Json(serde_json::json!({
                "message": e.to_string()
            }))
            .into_response();
            *res.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            res
        }
    }
}

async fn handler_inner(app: &AppState, req: ServerRequest) -> Result<Response> {
    match req {
        ServerRequest::Status {} => app.status().await.map(|res| Json(res).into_response()),
        ServerRequest::Balance { owner } => app
            .balance(&owner)
            .await
            .map(|res| Json(res).into_response()),
        ServerRequest::MintFunds {
            recipient,
            usd_amount,
            euro_amount,
        } => app
            .mint_funds(recipient, usd_amount, euro_amount)
            .await
            .map(|res| Json(res).into_response()),
        ServerRequest::SellDollars { trader, dollars } => app
            .sell_dollars(trader, dollars)
            .await
            .map(|res| Json(res).into_response()),
        ServerRequest::SellEuros { trader, euros } => app
            .sell_euros(trader, euros)
            .await
            .map(|res| Json(res).into_response()),
    }
}

impl AppState {
    async fn status(&self) -> Result<StatusResp> {
        let mut total_usd = UnsignedAsset::zero(Usd);
        let mut total_euro = UnsignedAsset::zero(Euro);

        let guard = self.0.lock();

        for balance in guard.accounts.values() {
            total_usd += UnsignedAsset::new(Usd, balance.usd);
            total_euro += UnsignedAsset::new(Euro, balance.euro);
        }

        total_usd += guard.pool_usd.into_unsigned();
        total_euro += guard.pool_euro.into_unsigned();

        Ok(StatusResp {
            total_usd,
            total_euro,
            price_usd: Price::from_asset_ratios(guard.pool_usd, guard.pool_euro),
            price_euro: Price::from_asset_ratios(guard.pool_euro, guard.pool_usd),
        })
    }
    async fn balance(&self, owner: &Owner) -> Result<BalanceResp> {
        Ok(self.0.lock().accounts.get(owner).map_or_else(
            BalanceResp::default,
            |Balances { usd, euro }| BalanceResp {
                usd: UnsignedAsset::new(Usd, *usd),
                euro: UnsignedAsset::new(Euro, *euro),
            },
        ))
    }

    async fn mint_funds(
        &self,
        recipient: Owner,
        usd_amount: UnsignedAsset<Usd>,
        euro_amount: UnsignedAsset<Euro>,
    ) -> Result<MintFundsResp> {
        let mut guard = self.0.lock();
        let owner = guard.accounts.entry(recipient).or_default();
        owner.usd += usd_amount.into_decimal();
        owner.euro += euro_amount.into_decimal();
        Ok(MintFundsResp {})
    }

    async fn sell_dollars(
        &self,
        trader: Owner,
        dollars: PositiveAsset<Usd>,
    ) -> Result<SellDollarsResp> {
        // Pool has a constant, K
        // K = total USD in pool * total EURO in pool
        // If you buy or sell, the value K must remain the same
        let mut guard = self.0.lock();

        let mut pool_usd = guard.pool_usd;
        let mut pool_euro = guard.pool_euro;
        let owner = guard.accounts.entry(trader).or_default();
        owner
            .usd
            .checked_sub_assign(dollars.into_unsigned().into_decimal())?;

        let k = pool_usd.into_unsigned().into_decimal() * pool_euro.into_unsigned().into_decimal();

        pool_usd += dollars;
        let new_pool_euro = k / pool_usd.into_unsigned().into_decimal();
        let new_pool_euro = PositiveAsset::new(Euro, PositiveDecimal::new(new_pool_euro)?);

        let euros_bought = pool_euro.checked_sub(new_pool_euro)?;

        owner.euro += euros_bought.into_unsigned().into_decimal();

        guard.pool_usd = pool_usd;
        guard.pool_euro = new_pool_euro;

        Ok(SellDollarsResp::ConversionSuccess { euros_bought })
    }

    async fn sell_euros(&self, trader: Owner, euros: PositiveAsset<Euro>) -> Result<SellEurosResp> {
        todo!()
    }
}
