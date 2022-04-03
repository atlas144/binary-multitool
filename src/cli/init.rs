use std::io::Write;
use super::input_handler;
use super::drawing;
use std::io::stdout;

static SEPARATOR_LENGTH: u8 = 30;

fn category_menu() -> u32 {
    let entries = [
        "positional systems"
    ];
    let menu_range = 1..entries.len() + 2;

    drawing::draw_separator(SEPARATOR_LENGTH);

    for index in 0..entries.len() {
        println!("{}. {}", index + 1, entries[index]);
    }

    println!("{}. exit", entries.len() + 1);
    drawing::draw_separator(SEPARATOR_LENGTH);
    print!("Select category or exit: ");
    stdout().flush().expect("STDOUT flushing failed!");
    
    let input_uint = input_handler::read_uint();

    match input_uint {
        Ok(parsed_uint) => 
            if menu_range.contains(&(parsed_uint as usize)) {
                return parsed_uint;
            } else {
                println!("Entered value does not correspond to any option!");
                return 0;
            }
        Err(_error) => println!("Entered value does not correspond to any option!"),
    }

    0
}

fn welcome() {
    let welcome_message = "Binary-multitool";

    println!("{welcome_message}");
}

pub fn run() {
    let mut category_input = 0;

    welcome();

    while category_input != 2 {
        category_input = category_menu();
    }
}