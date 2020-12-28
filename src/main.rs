use std::path::Path;
use json::JsonValue;
use std::fs::File;
use ncurses::*;
use std::env;
use std::fs;

mod book;
mod ui;
mod cmd;

use book::*;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut book = AddressBook::new(&mut args[1]);
    let mut input = None;

    let locale_conf = LcCategory::all;
    setlocale(locale_conf, "ru_RU.UTF-8");

    initscr();

    if args.len() == 1 {
        addstr("Address Book Path: ");
        getstr(&mut book.path);
        clear();
    } else {
        *book.path = String::from(&args[1]);
    }

    if !Path::new(&book.path).exists() {
        File::create(&book.path).expect("Error while creating file!");
        *book.json_data = JsonValue::new_array();
        book.update();
    }

    while input.unwrap() != 'q' {
        *book.json_data = json::parse(
            fs::read_to_string(book.path.as_str())
                .expect("Error while reading file!")
                .as_str(),
        )
        .unwrap();

        book.print_contacts();

        book.print_path();
        addstr("\n   Select operation / contact id:\n(a)dd | (e)dit | (r)emove | (q)uit > ");
        cmd::get_input(&mut input);

        match input {
            Some('a') => cmd::add(&book),
            Some('e') => cmd::edit(&book.path, &mut book.json_data),
            Some('r') => cmd::remove(&book.path, &mut book.json_data),
            Some('q') => (),
            _ => cmd::select(&mut input, &book),
        }

        clear();
    }

    endwin();
}
