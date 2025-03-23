use dioxus::prelude::*;
use dioxus::{
    prelude::{component, rsx, Element},
    signals::ReadOnlySignal,
};

#[component]
pub fn Error(err: ReadOnlySignal<anyhow::Error>) -> Element {
    rsx! {
        div { class: "not-found",
            h1 { "Oops!" }
            h2 { "{err}" }
        }
    }
}

#[component]
pub fn NotFound() -> Element {
    rsx! {
        div { class: "not-found",
            h1 { "404" }
            h2 { "Not found" }
        }
    }
}
