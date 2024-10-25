use std::io;
use std::io::BufRead;
// use std::io::Write;

extern crate colorful;
use colorful::Color;
use colorful::Colorful;
// use colorful::HSL;
// use colorful::RGB;

// MAIN
fn main() {
    // EMOJIS
    // let emoji_test = emojis::get_by_shortcode("arrow_left").unwrap();
    let emoji_todo_list = emojis::get_by_shortcode("bookmark_tabs").unwrap();
    let emoji_us = emojis::get_by_shortcode("us").unwrap();
    let emoji_es = emojis::get_by_shortcode("es").unwrap();
    let emoji_about = emojis::get_by_shortcode("green_circle").unwrap();
    let emoji_exit = emojis::get_by_shortcode("red_circle").unwrap();
    // let rocket = emojis::get("").unwrap();

    // TEXTS
    let title = "TODO LIST MEMORY";
    // let subtitle = "Store: Memory";
    let select_language = "Select an option";
    let invalid_option = "Invalid option";

    // println!("{}", emoji_test);
    println!(
        "{} {}",
        emoji_todo_list,
        title
            .color(Color::GreenYellow)
            .bg_color(Color::Black)
            .bold()
    );
    // println!(
    //     "{}",
    //     subtitle.color(Color::Orange3).bg_color(Color::Black).bold()
    // );
    println!("1: English {}", emoji_us);
    println!("2: Spanish {}", emoji_es);
    println!("3: About as {}", emoji_about);
    println!("0: Exit {}", emoji_exit);
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

// INVALID OPTIONS
fn invalid_option() {
    // TEXTS
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

// ABOUT AS
fn about_as() {
    // EMOJIS
    let emoji_about_as = emojis::get_by_shortcode("green_circle").unwrap();

    // TEXTS
    let title = "ABOUT AS";
    let selected = "... about as selected ";

    println!(
        "{}",
        selected
            .color(Color::DarkGray)
            .bg_color(Color::Black)
            .italic()
    );

    println!(
        "{} {}",
        emoji_about_as,
        title.color(Color::DarkSeaGreen4a).bg_color(Color::Black)
    );
    println!("Name: CLI Todo List Memory");
    println!("Author: Andres Giraldo Arenas");
    println!("Language: Rust");
    println!("Type: Command Line Interface");
    println!("Used libraries: Emojis, Colorful");
    println!("Storage: Memory");
    println!("Web: www.andresgadev.com");
    println!("X: @andresgandev");
    println!("Github: andresaganc");
    println!("Linkedin: ");
}

// ENGLISH LANGUAGES
fn english_language() {
    // EMOJIS
    let emoji_todo_list = emojis::get_by_shortcode("bookmark_tabs").unwrap();
    let emoji_list_tasks = emojis::get_by_shortcode("open_book").unwrap();
    let emoji_add_task = emojis::get_by_shortcode("blue_book").unwrap();
    let emoji_edit_task = emojis::get_by_shortcode("orange_book").unwrap();
    let emoji_delete_task = emojis::get_by_shortcode("closed_book").unwrap();
    let emoji_check_task = emojis::get_by_shortcode("green_book").unwrap();
    let emoji_back = emojis::get_by_shortcode("arrow_left").unwrap();

    // TEXTS
    let title = "TODO LIST MEMORY - ENGLISH";
    let selected = "... english language selected ";
    let select_option = "Select an option";
    let invalid_option = "Invalid option";

    println!(
        "{}",
        selected
            .color(Color::DarkGray)
            .bg_color(Color::Black)
            .italic()
    );

    loop {
        println!(
            "{} {}",
            emoji_todo_list,
            title.color(Color::Orange3).bg_color(Color::Black).bold()
        );

        println!("1: List tasks {}", emoji_list_tasks);
        println!("2: Add task {}", emoji_add_task);
        println!("3: Edit task {}", emoji_edit_task);
        println!("4: Delete task {}", emoji_delete_task);
        println!("5: Check task {}", emoji_check_task);
        println!("0: Back {}", emoji_back);
        println!(
            "{}",
            select_option.color(Color::Blue).bg_color(Color::Black)
        );

        let stdin = io::stdin();
        let action = stdin.lock().lines().next().unwrap().unwrap();

        match action.as_ref() {
            "1" => list_tasks_english(),
            "2" => add_task_english(),
            "3" => edit_task_english(),
            "4" => delete_task_english(),
            "5" => check_task_english(),
            "0" => main(),
            _ => println!("Option invalid"),
        }
    }
}

// LIST TASKS ENGLISH
fn list_tasks_english() {
    println!("List tasks");
}

// ADD TASKS ENGLISH
fn add_tasks_english() {
    println!("Add Task")
}

// SPANISH LANGUAGES
fn spanish_language() {
    println!("Spanish language");
}
