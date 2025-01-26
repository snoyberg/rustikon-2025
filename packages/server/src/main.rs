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
    BalanceResp, Euro, MintFundsResp, Owner, PositiveAsset, SellDollarsResp, SellEurosResp,
    ServerRequest, UnsignedAsset, UnsignedDecimal, Usd,
};
use parking_lot::Mutex;

#[derive(Clone)]
struct AppState(Arc<Mutex<AppStateInner>>);

struct AppStateInner {
    accounts: HashMap<Owner, Balances>,
    pool: Balances,
}

struct Balances {
    usd: UnsignedDecimal,
    euro: UnsignedDecimal,
}

#[tokio::main]
async fn main() {
    let app_state = AppState(Arc::new(Mutex::new(AppStateInner {
        accounts: HashMap::new(),
        pool: Balances {
            usd: "103000".parse().unwrap(),
            euro: "100000".parse().unwrap(),
        },
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
        ServerRequest::Status {} => todo!(),
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
    async fn balance(&self, owner: &Owner) -> Result<BalanceResp> {
        Ok(self.0.lock().accounts.get(&owner).map_or_else(
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
        todo!()
    }

    async fn sell_dollars(
        &self,
        trader: Owner,
        dollars: PositiveAsset<Usd>,
    ) -> Result<SellDollarsResp> {
        todo!()
    }

    async fn sell_euros(&self, trader: Owner, euros: PositiveAsset<Euro>) -> Result<SellEurosResp> {
        todo!()
    }
}
