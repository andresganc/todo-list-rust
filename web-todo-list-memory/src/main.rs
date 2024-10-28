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
    // LOGIC
    let mut count = use_signal(|| 0);

    // IMAGES
    const MARVEL: manganis::ImageAsset = manganis::mg!(image("./assets/marvel.png")
        .size(150, 150)
        .format(ImageType::Avif)
        .preload());

    rsx! {
        // IMPORT TAILWIND STYLES
        link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css" },

        div { class: "container mx-auto flex justify-between p-6 text-blue bg-blue-500",
            div {
                h4 { class:"text-blue-500", "TODO LIST MEMORY / WEB"}
            }

            nav { class: "container flex item-center bg-black-200",
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
                src: "{MARVEL}"
            }

            h1 { "High-Five counter: {count}" }
            button { class: "bg-blue-500", onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}
