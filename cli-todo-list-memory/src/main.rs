use std::io;
use std::io::BufRead;
use std::io::Write;

extern crate colorful;
use colorful::Color;
use colorful::Colorful;
// use colorful::HSL;
// use colorful::RGB;

fn main() {
    // TEXTS
    let title = "TODO LIST MEMORY";
    let subtitle = "Store: Memory";
    let select_language = "Select an option";
    let invalid_option = "Invalid option";

    println!(
        "{}",
        title
            .color(Color::GreenYellow)
            .bg_color(Color::Black)
            .bold()
    );
    // println!(
    //     "{}",
    //     subtitle.color(Color::Orange3).bg_color(Color::Black).bold()
    // );
    println!("1: English");
    println!("2: Spanish");
    println!("3: About as");
    println!("0: Exit");
    println!(
        "{}",
        select_language
            .color(Color::Blue)
            .bg_color(Color::Black)
            .bold()
    );

    loop {
        let stdin = io::stdin();
        let action = stdin.lock().lines().next().unwrap().unwrap();

        match action.as_ref() {
            "1" => english_language(),
            "2" => spanish_language(),
            "3" => about_as(),
            "0" => break,
            _ => println!(
                "{}",
                invalid_option.color(Color::Red).bg_color(Color::Black)
            ),
        }
    }
}

fn invalid_option() {
    let select_language = "Select language";
    let invalid_option = "Invalid option";

    println!(
        "{}",
        invalid_option.color(Color::Red).bg_color(Color::Black)
    );
    println!(
        "{}",
        select_language
            .color(Color::Blue)
            .bg_color(Color::Black)
            .bold()
    );
}

fn about_as() {
    let title = "ABOUT AS";
    println!(
        "{}",
        title.color(Color::DarkSeaGreen4a).bg_color(Color::Black)
    );
    println!("Name: CLI Todo List Memory");
    println!("Author: Andres Giraldo Arenas");
    println!("Language: Rust");
    println!("Type: Command Line Interface");
    println!("Used libraries: Colorful");
    println!("Storage: Memory");
}

fn english_language() {
    let title = "TODO LIST MEMORY - ENGLISH";
    let english_language = "English language selected";
    let select_option = "Select an option";
    let invalid_option = "Invalid option";

    println!(
        "{}",
        english_language
            .color(Color::DarkGray)
            .bg_color(Color::Black)
    );
    println!(
        "{}",
        title.color(Color::Orange3).bg_color(Color::Black).bold()
    );

    println!("1: List tasks");
    println!("2: Add task");
    println!("3: Edit task");
    println!("4: Remove task");
    println!("5: Check task");
    println!("0: Back");
    println!(
        "{}",
        select_option.color(Color::Blue).bg_color(Color::Black)
    );
}

fn spanish_language() {
    println!("Spanish language");
}
