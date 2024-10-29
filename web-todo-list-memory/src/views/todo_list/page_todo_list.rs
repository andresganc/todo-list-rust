#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

#[component]
pub fn PageTodoList() -> Element {
    rsx! {
        h2 {"PAGE TODO LIST"}
    }
}
