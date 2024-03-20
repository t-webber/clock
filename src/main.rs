#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::restriction,
    clippy::nursery,
    clippy::cargo
)]
#![allow(clippy::implicit_return, clippy::single_call_fn)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::separated_literal_suffix)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::pattern_type_mismatch)]
#![allow(clippy::question_mark_used)]
#![allow(clippy::absolute_paths)]

mod clock;
mod fetch;
mod titlebar;
mod zoom;

#[yew::prelude::function_component(App)]
fn app() -> yew::prelude::Html {
    yew::prelude::html! {
        <main class="container">
        <titlebar::TitleBar />
        <clock::Clock />
        // <zoom::App />
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
