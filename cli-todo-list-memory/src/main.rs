extern crate colorful;
use colorful::Color;
use colorful::Colorful;
// use colorful::HSL;
// use colorful::RGB;

fn main() {
    // TEXTS
    let title = "TODO LIST - COMMAND LINE INTERFACE";
    let subtitle = "Store: Memory";
    let select_language = "Select language";

    println!(
        "{}",
        title
            .color(Color::GreenYellow)
            .bg_color(Color::Black)
            .bold()
    );
    println!(
        "{}",
        subtitle.color(Color::Orange3).bg_color(Color::Black).bold()
    );
    println!("1: English");
    println!("2: Spanish");
    println!("0: Exit");
    println!(
        "{}",
        select_language
            .color(Color::Blue)
            .bg_color(Color::Black)
            .bold()
    );
}
