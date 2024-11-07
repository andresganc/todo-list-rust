#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn SectionHeaderAA() -> Element {
    rsx! {
        div {
            h3 {"Section Header About As"}
        }
    }
}
