#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use wasm_bindgen::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

#[wasm_bindgen(module = "/src/styles.css")]
extern "C" {
    fn container() -> String;
    fn btn() -> String;
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        div {
            div {
                h4 {"TODO LIST MEMORY / WEB"}
            }

            nav {
                div {

                    p {"Home"}
                }
                div {

                    p {"Todo List"}
                }
                div {

                    p {"About as"}
                }
            }
        }

        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {

    div { class: "container",
        div {

            h4 {"TODO LIST MEMORY / WEB"}
        }

        nav { display: "flex", justify_content: "center", align_items: "center",
            div {

                p {"Home"}
            }
            div {

                p {"Todo List"}
            }
            div {

                p {"About as"}
            }
        }
    }

        Link {
            to: Route::Blog {
                id: count()
            },
            "Go to blog"
        }
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}
