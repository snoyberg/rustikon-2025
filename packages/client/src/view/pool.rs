use crate::prelude::*;

#[component]
pub(super) fn Pool() -> impl IntoView {
    let status = query::status().use_query(|| ());

    move || match status.data.get() {
        None => view! { <i>Loading pool information from server...</i> }.into_view(),
        Some(Err(e)) => {
            view! { <p class="error">Error loading pool information: {e.to_string()}</p> }
                .into_view()
        }
        Some(Ok(status)) => view! {
            <dl>
                <dt>Total USD minted</dt>
                <dd>{status.total_usd.into_decimal().to_string()}</dd>
                <dt>Total EURO minted</dt>
                <dd>{status.total_euro.into_decimal().to_string()}</dd>
                <dt>Price of 1 USD</dt>
                <dd>{status.price_usd.to_string()}</dd>
                <dt>Price of 1 EURO</dt>
                <dd>{status.price_euro.to_string()}</dd>
            </dl>
        }
        .into_view(),
    }
}
