use crate::views::{Editor, Home, Shell};
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[layout(Shell)]
    #[route("/")]
    Home {},
    #[route("/editor/:template")]
    Editor { template: String },
}
