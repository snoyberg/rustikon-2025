mod owners;
mod pool;
mod wallet;

use crate::prelude::*;

#[component]
pub(super) fn App() -> impl IntoView {
    provide_query_client();

    view! {
        <main>
            <h1>Rustikon 2025 Strong Typing Demo</h1>
            <section id="pool">
                <h1>Liquidity pool information</h1>
                <pool::Pool />
            </section>
            <section id="owners">
                <h1>Owners</h1>
                <owners::Owners />
            </section>
            <section id="wallet">
                <h1>Wallet</h1>
                <wallet::Wallet />
            </section>
        </main>
    }
}
