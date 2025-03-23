use dioxus::signals::Signal;
use fs::Metadata;

#[derive(Clone, Copy)]
pub struct Context {
    pub command: Signal<String>,
    pub list: Signal<Option<Vec<Metadata>>>,
    pub dir_list: Signal<Vec<String>>,
}
