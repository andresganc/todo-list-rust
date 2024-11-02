#![allow(non_snake_case)]
use dioxus::prelude::*;
// use dioxus_logger::tracing::{info, Level};

// MODULES
use super::section_header_home::SectionHeaderHome;

#[component]
pub fn PageHome() -> Element {
    rsx! {
        h2 {"PAGE HOME"}
        {SectionHeaderHome()}
    }
}
