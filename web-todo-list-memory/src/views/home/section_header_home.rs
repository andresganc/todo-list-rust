#![allow(non_snake_case)]

use dioxus::prelude::*;
// use dioxus_logger::tracing::{info, Level};

#[component]
pub fn SectionHeaderHome() -> Element {
    rsx! {
        div {
            h3 {"Section header"}
        }
    }
}
