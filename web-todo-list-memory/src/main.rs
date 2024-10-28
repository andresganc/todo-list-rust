#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use manganis::*;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/todo-list")]
    TodoList {},
    #[route("/about-as")]
    AboutAs,
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

// HOME
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

        div { class: "flex justify-between aling-center p-5 text-white bg-blue-500",
            div {
                h4 { class:"text-lg text-white font-bold", "TODO LIST MEMORY / WEB"}
            }

            div {
                nav { class: "flex justify-center aling-center gap-4 ",
                    div {
                        Link {
                            to: Route::Home {},
                            p {"Home"}
                        }
                    }
                    div {
                        Link {
                            to: Route::TodoList {},
                            p {"Todo List"}
                        }
                    }
                    div {
                        Link {
                            to: Route::AboutAs {},
                            p {"About as"}
                        }
                    }
                    // div {
                    //     Link {
                    //         to: Route::Blog {
                    //             id: count()
                    //         },
                    //         p {"Todo List"}
                    //     }
                    // }
                }
            }
        }


        div {

            img {
                src: "{MARVEL}"
            }

            h1 { "High-Five counter: {count}" }
            button { class: "bg-blue-500", onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }

        div {
            h3 {"FOOTER"}
        }
    }
}

// TODO LIST
#[component]
fn TodoList() -> Element {
    rsx! {
        div {
            h2 {"TODO LIST"}
        }
    }
}

// ABOUT AS
#[component]
fn AboutAs() -> Element {
    rsx! {
        div {
            h2 {"ABOUT AS"}
        }
    }
}

// BLOG
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
