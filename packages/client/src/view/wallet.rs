use crate::prelude::*;

#[component]
pub(super) fn Wallet() -> impl IntoView {
    let (owner, set_owner) = create_signal("Michael".to_owned());
    let (status, set_status) = create_signal(None::<String>);

    let mint_dollars = Action::new(move |()| {
        query::mint_funds(
            Owner(owner.get()),
            "100USD".parse().unwrap(),
            "0EURO".parse().unwrap(),
            set_status,
        )
    });

    let mint_euros = Action::new(move |()| {
        query::mint_funds(
            Owner(owner.get()),
            "0USD".parse().unwrap(),
            "100EURO".parse().unwrap(),
            set_status,
        )
    });

    let sell_dollars = Action::new(move |()| {
        query::sell_asset(
            Owner(owner.get()),
            query::ToSell::Dollars("10USD".parse().unwrap()),
            set_status,
        )
    });

    let sell_euros = Action::new(move |()| {
        query::sell_asset(
            Owner(owner.get()),
            query::ToSell::Euros("10EURO".parse().unwrap()),
            set_status,
        )
    });

    let status_view = move || match status.get() {
        Some(status) => view! {<p><i>{status}</i></p>}.into_view(),
        None => View::default(),
    };

    view! {
        {status_view}
        <label for="owner">
            "Owner name"
            <input
                type="text"
                value=owner
                on:input=move |ev|  set_owner.set(event_target_value(&ev))
            />
        </label>
        <button on:click=move |_| mint_dollars.dispatch(())>Give 100 USD</button>
        <button on:click=move |_| mint_euros.dispatch(())>Give 100 EURO</button>
        <button on:click=move |_| sell_dollars.dispatch(())>Sell 10 USD for EURO</button>
        <button on:click=move |_| sell_euros.dispatch(())>Sell 10 EURO for USD</button>
    }
}
