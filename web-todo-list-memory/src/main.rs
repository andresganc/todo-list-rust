#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use manganis::*;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
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

// #[warn(unused_imports)]
#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);
    const ASSET: manganis::ImageAsset = manganis::mg!(image("./assets/marvel.png"));
    // const AVIF_ASSET: &str = manganis::file!("./assets/header.svg");
    // pub const AVIF_ASSET: manganis::ImageAsset =
    manganis::mg!(image("./assets/marvel.png").format(ImageType::Avif));

    rsx! {

    // document::Link { rel: "stylesheet", href: STYLE }

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

            img {
                src: "{ASSET}"
            }

            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}
