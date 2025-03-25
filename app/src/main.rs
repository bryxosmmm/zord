use config::Config;
use dioxus::desktop;
use dioxus::prelude::*;
use fs::Metadata;
use util_components::{Error, NotFound};
use utils::handle_keyboard;
use utils::load_list;
use utils::{handle_click, sort};

mod util_components;
mod utils;

fn main() {
    let header = r#"<link rel="stylesheet" href="/usr/share/zord/main.css">"#.to_string();
    let window = desktop::WindowBuilder::default()
        .with_decorations(false)
        .with_resizable(false)
        .with_inner_size(desktop::LogicalSize::new(800, 600));
    let config = desktop::Config::default()
        .with_window(window)
        .with_custom_head(header);
    LaunchBuilder::desktop().with_cfg(config).launch(App);
}

#[component]
fn App() -> Element {
    let bind = utils::load();
    let mut input = use_signal(|| "".to_string());
    rsx! {
        div { onkeypress: move |e| handle_keyboard(e.key()),
            div { id: "input-box",
                input {
                    class: "search-input",
                    autofocus: true,
                    oninput: move |event| input.set(event.value()),
                    placeholder: "Search...",
                }
                match bind {
                    Ok(config) => rsx! {
                        List { config, input }
                    },
                    Err(err) => rsx! {
                        Error { err }
                    },
                }
            }
        }
    }
}

#[component]
fn Item(e: ReadOnlySignal<Metadata>, command: ReadOnlySignal<String>) -> Element {
    rsx! {
        div {
            id: "search-item",
            onclick: move |_| handle_click(e().path, command().clone()),
            strong { "{e().name}" }
            span { "Audio: {e().tracks} Subs: {e().subtitles}" }
        }
    }
}

#[component]
fn List(config: ReadOnlySignal<Config>, input: ReadOnlySignal<String>) -> Element {
    let list = use_resource(move || async move {
        let config = config();
        load_list(config.volumes.dirs())
    });
    let changed: Memo<Option<Vec<Metadata>>> = use_memo(move || {
        (*list.read_unchecked())
            .as_ref()
            .map(|l| sort(l.to_vec(), input()))
    });
    rsx! {
        div { id: "list",
            match changed() {
                Some(v) => {
                    if !v.is_empty() {
                        rsx! {
                            for x in v {
                                Item { e: x, command: config().options.command() }
                            }
                        }
                    } else {
                        rsx! {
                            NotFound {}
                        }
                    }
                }
                None => rsx! {
                    NotFound {}
                },
            }
        }
    }
}
