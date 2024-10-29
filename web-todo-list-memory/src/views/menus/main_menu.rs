#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

// MODULES
// mod views;
use super::super::home::page_home::PageHome as ComponentPageHome;
use super::super::todo_list::page_todo_list::PageTodoList as ComponentPageTodoList;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    ComponentPageHome {},
    #[route("/todo-list")]
    ComponentPageTodoList {},
}

#[component]
pub fn MainMenu() -> Element {
    // IMAGES
    // const MARVEL: manganis::ImageAsset = manganis::mg!(image("./assets/marvel.png")
    //     .size(150, 150)
    //     .format(ImageType::Avif)
    //     .preload());

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
                            to: Route::ComponentPageHome {},
                            p {"Home"}
                        }
                    }
                    div {
                        Link {
                            to: Route::ComponentPageTodoList {},
                            p {"Todo List"}
                        }
                    }

                }
            }
        }
    Outlet::<Route> {}
    }
}
