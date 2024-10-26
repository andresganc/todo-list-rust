use std::io;
use std::io::BufRead;
use std::io::Write;

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
    println!("Todo list basic in English and Spanih, With List, Add, Edit and Delete");
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
    // let mut tasks: Vec<String> = vec![String::new()];
    let mut tasks = Vec::new();

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
    let selected_option = "seleted option ...";
    let invalid_option = "Invalid option";

    println!(
        "{}",
        selected_option
            .color(Color::DarkGreen)
            .bg_color(Color::Black)
    );

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
            "1" => list_tasks_english(&tasks),
            "2" => add_task_english(&mut tasks),
            "3" => edit_task_english(&mut tasks),
            "4" => delete_task_english(&mut tasks),
            "5" => check_task_english(),
            "0" => main(),
            _ => println!("Invalid option"),
        }
    }
}

// LIST TASKS ENGLISH
fn list_tasks_english(tasks: &Vec<String>) {
    let no_action = "... there are no tasks in the list";
    let action = "... listed tasks";
    let selected_option = "selected option ...";

    println!(
        "{}",
        selected_option
            .color(Color::DarkGreen)
            .bg_color(Color::Black)
    );

    if tasks.is_empty() {
        println!(
            "{}",
            no_action.color(Color::DarkRed1).bg_color(Color::Black)
        );
    } else {
        for task in tasks {
            println!(" - {}", task);
        }
        println!("{}", action.color(Color::DarkGray).bg_color(Color::Black));
    }
}

// ADD TASKS ENGLISH
fn add_task_english(tasks: &mut Vec<String>) {
    let mut input = String::new();

    let action = "... task add successfully";
    let selected_option = "selected option ...";

    println!(
        "{}",
        selected_option
            .color(Color::DarkGreen)
            .bg_color(Color::Black)
    );

    print!("Enter task: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    let task = input.trim().to_string();

    tasks.push(task);

    println!("{}", action.color(Color::DarkGreen).bg_color(Color::Black));
}

// EDIT TASK ENGLISH
fn edit_task_english(tasks: &mut Vec<String>) {
    let mut input = String::new();
    let mut new_input = String::new();
    let selected_option = "selected option ...";

    let action = "... task edited successfully";

    println!(
        "{}",
        selected_option
            .color(Color::DarkGreen)
            .bg_color(Color::Black)
    );

    print!("Enter the index of the task to edit: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    let index = input.trim().parse::<usize>().unwrap();

    println!("The task selected is: {}", index);

    println!("Enter the new task");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut new_input).unwrap();

    tasks[index] = new_input;

    println!("{}", action.color(Color::DarkGreen).bg_color(Color::Black));
}

// DELETE TASK ENGLISH
fn delete_task_english(tasks: &mut Vec<String>) {
    let mut input = String::new();

    let no_action = "... invalid task index";
    let action = "... task deleted successfully";
    let selected_option = "selected option ...";

    println!(
        "{}",
        selected_option
            .color(Color::DarkGreen)
            .bg_color(Color::Black)
    );

    print!("Enter task index to delete: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    let index = input.trim().parse::<usize>().unwrap();

    if index >= tasks.len() {
        print!(
            "{}",
            no_action.color(Color::DarkRed1).bg_color(Color::Black)
        );
    } else {
        tasks.remove(index);
        println!("{}", action.color(Color::DarkGreen).bg_color(Color::Black));
    }
}

// CHECK TASK ENGLISH
fn check_task_english() {
    println!("Check task english");
}

// SPANISH LANGUAGES
fn spanish_language() {
    // let mut tasks: Vec<String> = vec![String::new()];
    let mut tasks = Vec::new();

    // EMOJIS
    let emoji_todo_list = emojis::get_by_shortcode("bookmark_tabs").unwrap();
    let emoji_list_tasks = emojis::get_by_shortcode("open_book").unwrap();
    let emoji_add_task = emojis::get_by_shortcode("blue_book").unwrap();
    let emoji_edit_task = emojis::get_by_shortcode("orange_book").unwrap();
    let emoji_delete_task = emojis::get_by_shortcode("closed_book").unwrap();
    let emoji_check_task = emojis::get_by_shortcode("green_book").unwrap();
    let emoji_back = emojis::get_by_shortcode("arrow_left").unwrap();

    // TEXTS
    let title = "TODO LIST MEMORY - SPANISH";
    let selected = "... lenguaje español seleccionado ";
    let select_option = "Seleccione una opcion";
    let selected_option = "opcion seleccionada ...";
    let invalid_option = "opcion invalida";

    println!(
        "{}",
        selected_option
            .color(Color::DarkGreen)
            .bg_color(Color::Black)
    );

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

        println!("1: Listar tareas {}", emoji_list_tasks);
        println!("2: Agregar tarea {}", emoji_add_task);
        println!("3: Editar tarea {}", emoji_edit_task);
        println!("4: Eliminar tarea {}", emoji_delete_task);
        println!("5: Chequear tarea {}", emoji_check_task);
        println!("0: Atras {}", emoji_back);
        println!(
            "{}",
            select_option.color(Color::Blue).bg_color(Color::Black)
        );

        let stdin = io::stdin();
        let action = stdin.lock().lines().next().unwrap().unwrap();

        match action.as_ref() {
            "1" => list_tasks_spanish(&tasks),
            "2" => add_task_spanish(&mut tasks),
            "3" => edit_task_spanish(&mut tasks),
            "4" => delete_task_spanish(&mut tasks),
            "5" => check_task_spanish(),
            "0" => main(),
            _ => println!("opcion invalida"),
        }
    }
}

// LIST TASKS SPANISH
fn list_tasks_spanish(tasks: &Vec<String>) {
    let no_action = "... no hay tareas en la lista";
    let action = "... tareas listadas";
    let selected_option = "opcion seleccionada ...";

    println!(
        "{}",
        selected_option
            .color(Color::DarkGreen)
            .bg_color(Color::Black)
    );

    if tasks.is_empty() {
        println!(
            "{}",
            no_action.color(Color::DarkRed1).bg_color(Color::Black)
        );
    } else {
        for task in tasks {
            println!(" - {}", task);
        }
        println!("{}", action.color(Color::DarkGray).bg_color(Color::Black));
    }
}

// ADD TASKS SPANISH
fn add_task_spanish(tasks: &mut Vec<String>) {
    let mut input = String::new();

    let action = "... tarea agregada satisfactoriamente";
    let selected_option = "opcion seleccionada ...";

    println!(
        "{}",
        selected_option
            .color(Color::DarkGreen)
            .bg_color(Color::Black)
    );

    print!("Agregue la tarea: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    let task = input.trim().to_string();

    tasks.push(task);

    println!("{}", action.color(Color::DarkGreen).bg_color(Color::Black));
}

// EDIT TASK SPANISH
fn edit_task_spanish(tasks: &mut Vec<String>) {
    let mut input = String::new();
    let mut new_input = String::new();
    let selected_option = "opcion seleccionada ...";

    let action = "... taarea editada satisfactoriamente ...";

    println!(
        "{}",
        selected_option
            .color(Color::DarkGreen)
            .bg_color(Color::Black)
    );

    print!("Agregue el index de la tarea a editar: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    let index = input.trim().parse::<usize>().unwrap();

    println!("La tarea seleccionada es: {}", index);

    println!("Agregue la nueva tarea: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut new_input).unwrap();

    tasks[index] = new_input;

    println!("{}", action.color(Color::DarkGreen).bg_color(Color::Black));
}

// DELETE TASK SPANISH
fn delete_task_spanish(tasks: &mut Vec<String>) {
    let mut input = String::new();

    let no_action = "... index de tarea invalida";
    let action = "... tarea eliminada satisfactoriamente";
    let selected_option = "opcion seleccionada ...";

    println!(
        "{}",
        selected_option
            .color(Color::DarkGreen)
            .bg_color(Color::Black)
    );

    print!("Agregue el index de la tarea a eliminar: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    let index = input.trim().parse::<usize>().unwrap();

    if index >= tasks.len() {
        print!(
            "{}",
            no_action.color(Color::DarkRed1).bg_color(Color::Black)
        );
    } else {
        tasks.remove(index);
        println!("{}", action.color(Color::DarkGreen).bg_color(Color::Black));
    }
}

// CHECK TASK SPANISH
fn check_task_spanish() {
    println!("Chequear tarea español");
}
