#![allow(non_snake_case)]
use dioxus::prelude::*;
// use dioxus_logger::tracing::{info, Level};

// Modules
use super::section_body_aa::SectionBodyAA;
use super::section_header_aa::SectionHeaderAA;

#[component]
pub fn PageAboutAs() -> Element {
    rsx! {
        h2 {"PAGE ABOUT AS"}
        {SectionHeaderAA()}
        {SectionBodyAA()}
    }
}
