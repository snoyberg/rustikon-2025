use crate::prelude::*;

#[component]
pub(super) fn Owners() -> impl IntoView {
    let owners = query::owners().use_query(|| ());

    move || match owners.data.get() {
        None => view! { <i>Loading owner information</i> }.into_view(),
        Some(Err(e)) => {
            view! { <p class="error">Error loading owner information: {e.to_string()}</p> }
                .into_view()
        }
        Some(Ok(owners)) => view! {
            <For
                each=move || owners.clone()
                key=|owner| owner.owner.clone()
                children=move |balance| {
                    view! {
                        <p>
                            {balance.owner.to_string()}
                            " has "
                            {balance.dollars.to_string()}
                            " and "
                            {balance.euros.to_string()}
                        </p>
                    }
                }
            />
        },
    }
}
