#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn SectionBodyHome() -> Element {
    rsx! {
        div {
            h3 {"Section body home"}
        }
    }
}
