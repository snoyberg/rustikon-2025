use std::{collections::BTreeMap, sync::Arc};

use anyhow::Result;
use axum::{
    extract::State,
    http::{header::CONTENT_TYPE, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use common::{
    BalanceResp, Euro, ListOwnersResp, MintFundsResp, Owner, PositiveAsset, PositiveDecimal, Price,
    SellDollarsResp, SellEurosResp, ServerRequest, StatusResp, UnsignedAsset, Usd,
};
use parking_lot::Mutex;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
struct AppState(Arc<Mutex<AppStateInner>>);

struct AppStateInner {
    accounts: BTreeMap<Owner, Balances>,
    pool_usd: PositiveAsset<Usd>,
    pool_euro: PositiveAsset<Euro>,
}

#[derive(Default)]
struct Balances {
    usd: UnsignedAsset<Usd>,
    euro: UnsignedAsset<Euro>,
}

#[tokio::main]
async fn main() {
    let app_state = AppState(Arc::new(Mutex::new(AppStateInner {
        accounts: BTreeMap::new(),
        pool_usd: "103000USD".parse().unwrap(),
        pool_euro: "100000EURO".parse().unwrap(),
    })));

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let app = Router::new()
        .route("/", post(handler))
        .layer(cors)
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
        ServerRequest::ListOwners { start_after } => app
            .list_owners(start_after)
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
            total_usd += balance.usd;
            total_euro += balance.euro;
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
                usd: *usd,
                euro: *euro,
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
        owner.usd += usd_amount;
        owner.euro += euro_amount;
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
        let pool_euro = guard.pool_euro;
        let owner = guard.accounts.entry(trader).or_default();
        owner.usd.checked_sub_assign(dollars.into_unsigned())?;

        let k = pool_usd.into_unsigned().into_decimal() * pool_euro.into_unsigned().into_decimal();

        pool_usd += dollars;
        let new_pool_euro = k / pool_usd.into_unsigned().into_decimal();
        let new_pool_euro = PositiveAsset::new(Euro, PositiveDecimal::new(new_pool_euro)?);

        let euros_bought = pool_euro.checked_sub(new_pool_euro)?;

        owner.euro += euros_bought.into_unsigned();

        guard.pool_usd = pool_usd;
        guard.pool_euro = new_pool_euro;

        Ok(SellDollarsResp { euros_bought })
    }

    async fn sell_euros(&self, trader: Owner, euros: PositiveAsset<Euro>) -> Result<SellEurosResp> {
        // Same as sell_dollars but in reverse
        let mut guard = self.0.lock();

        let pool_usd = guard.pool_usd;
        let mut pool_euro = guard.pool_euro;
        let owner = guard.accounts.entry(trader).or_default();
        owner.euro.checked_sub_assign(euros.into_unsigned())?;

        let k = pool_usd.into_unsigned().into_decimal() * pool_euro.into_unsigned().into_decimal();

        pool_euro += euros;
        let new_pool_dollar = k / pool_euro.into_unsigned().into_decimal();
        let new_pool_dollar = PositiveAsset::new(Usd, PositiveDecimal::new(new_pool_dollar)?);

        let dollars_bought = pool_usd.checked_sub(new_pool_dollar)?;

        owner.usd += dollars_bought.into_unsigned();

        guard.pool_usd = new_pool_dollar;
        guard.pool_euro = pool_euro;

        Ok(SellEurosResp { dollars_bought })
    }

    async fn list_owners(&self, start_after: Option<Owner>) -> Result<ListOwnersResp> {
        const LIMIT: usize = 10;
        let guard = self.0.lock();

        let owners = match start_after {
            Some(start_after) => guard
                .accounts
                .keys()
                .skip_while(|s| *s <= &start_after)
                .take(LIMIT)
                .cloned()
                .collect(),
            None => guard.accounts.keys().take(LIMIT).cloned().collect(),
        };

        Ok(ListOwnersResp { owners })
    }
}
