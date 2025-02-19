mod error;
mod prelude;
mod query;
mod view;

fn main() {
    leptos::mount_to_body(view::App)
}
