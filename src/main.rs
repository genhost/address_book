use ncurses::*;
use std::char;
use std::env;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut book_file_path = String::new();
    let mut str_operation = String::new();
    let book_file: json::JsonValue;
    let mut operation = ' ';

    initscr();

    if args.len() == 1 {
        addstr("Address Book Path: ");
        getstr(&mut book_file_path);
        clear();
    } else {
        book_file_path = String::from(&args[1]);
    }

    if Path::new(book_file_path.as_str()).exists() {
        book_file = json::parse(
            fs::read_to_string(&book_file_path)
                .expect("Error while reading file!")
                .as_str(),
        )
        .unwrap();
    } else {
        File::create(&book_file_path).expect("Error while creating file!");
        book_file = json::JsonValue::new_array();
        update(&book_file_path, &book_file);
    }

    while operation != 'q' {
        addstr(format!("   Address Book: {}\n\n", &book_file_path).as_str());

        let mut contacts: Vec<(usize, &str, &str, &str, &str)> = Vec::new();

        for contact in book_file.members() {
            contacts.push((
                contact[0].as_usize().unwrap(),
                contact[1].as_str().unwrap(),
                contact[2].as_str().unwrap(),
                contact[3].as_str().unwrap(),
                contact[4].as_str().unwrap(),
            ));
        }

        for i in 0..contacts.len() {
            addstr(
                format!(
                    "{:>2} {:width$}\t{:width$}\t{:width$}\t\t{:width$}\n",
                    contacts[i].0,
                    contacts[i].1,
                    contacts[i].2,
                    contacts[i].3,
                    contacts[i].4,
                    width = 16,
                )
                .as_str(),
            );
        }

        // addstr("\n   Select operation / contact id:\n(a)dd | (e)dit | (r)emove | (q)uit > ");
        addstr("\n   Select operation / contact id:\n\t\t\t(q)uit > ");
        operation = getch() as u8 as char;
        match operation {
            'a' => add(&book_file_path, &book_file),
            'e' => edit(&book_file_path, &book_file),
            'r' => remove(&book_file_path, &book_file),
            'q' => (),
            _ => {
                if operation.is_numeric() {
                    let mut selected = String::from(operation);

                    getstr(&mut str_operation);
                    selected += str_operation.as_str();

                    let id = selected.parse::<usize>().unwrap() - 1;

                    clear();

                    loop {
                        if let None = contacts.get(id) {
                            str_operation = String::new();
                            operation = ' ';
                            break;
                        }

                        addstr(format!("   Address Book: {}\n", &book_file_path).as_str());
                        addstr(format!(
                            "Information about {}\n\nContact number: {}\nAddress: {}\nNote: {}\n",
                            contacts[id].1,
                            contacts[id].2,
                            contacts[id].3,
                            contacts[id].4
                        ).as_str());

                        // addstr("\n   Select operation / contact id:\n    (e)dit | (r)emove | (b)ack > ");
                        addstr("\n   Select operation / contact id:\n\t\t\t(b)ack > ");
                        operation = getch() as u8 as char;

                        match operation {
                            'e' => {
                                edit_selected(&book_file_path, &book_file, id);
                            },
                            'r' => {
                                remove_selected(&book_file_path, &book_file, id);
                            },
                            'b' | 'q' => {
                                str_operation = String::new();
                                operation = ' ';
                                clear();
                                break;
                            },
                            _ => (),
                        }

                        clear();
                    }
                }
            }
        }

        clear();
    }

    endwin();
}

fn update(path: &str, object: &json::JsonValue) {
    let mut file = OpenOptions::new()
        .write(true)
        .open(path)
        .expect("Unable to open file!");
    file.write_all(json::stringify_pretty(object.dump(), 4).as_bytes())
        .expect("Error while writing!");
}

fn add(path: &str, object: &json::JsonValue) {}

fn edit(path: &str, object: &json::JsonValue) {}

fn edit_selected(path: &str, object: &json::JsonValue, id: usize) {}

fn remove(path: &str, object: &json::JsonValue) {}

fn remove_selected(path: &str, object: &json::JsonValue, id: usize) {}
