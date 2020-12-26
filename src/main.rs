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
    let mut book_file: json::JsonValue;
    let mut operation = ' ';

    let locale_conf = LcCategory::all;
    setlocale(locale_conf, "ru_RU.UTF-8");

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

        addstr("\n   Select operation / contact id:\n(a)dd | (e)dit | (r)emove | (q)uit > ");
        operation = getch() as u8 as char;

        match operation {
            'a' => {
                add(&book_file_path, &mut book_file);
            }
            'e' => edit(&book_file_path, &book_file),
            'r' => remove(&book_file_path, &book_file),
            'q' => (),
            _ => select(
                &mut operation,
                contacts,
                book_file_path.as_str(),
                &book_file,
            ),
        }

        clear();
    }

    endwin();
}

fn print_book_path(path: &str) {
    clear();
    addstr(format!("   Address Book: {}\n\n", path).as_str());
}

fn update(path: &str, object: &json::JsonValue) {
    let mut file = OpenOptions::new()
        .write(true)
        .open(path)
        .expect("Unable to open file!");
    file.write_all(object.dump().as_bytes())
        .expect("Error while writing!");
}

fn select(
    operation: &mut char,
    contacts: Vec<(usize, &str, &str, &str, &str)>,
    book_file_path: &str,
    book_file: &json::JsonValue,
) {
    if operation.is_numeric() {
        let mut str_operation = String::new();
        let mut selected = String::from(*operation);

        getstr(&mut str_operation);
        selected += str_operation.as_str();

        let id = selected.parse::<usize>().unwrap() - 1;


        loop {
            if let None = contacts.get(id) {
                *operation = ' ';
                break;
            }

            print_book_path(book_file_path);
            addstr(
                format!(
                    "Information about {}\n\nContact number: {}\nAddress: {}\nNote: {}\n",
                    contacts[id].1, contacts[id].2, contacts[id].3, contacts[id].4
                )
                .as_str(),
            );

            addstr("\n   Select operation / contact id:\n    (e)dit | (r)emove | (b)ack > ");
            *operation = getch() as u8 as char;

            match operation {
                'e' => {
                    edit_selected(&book_file_path, book_file, id);
                }
                'r' => {
                    remove_selected(&book_file_path, book_file, id);
                }
                'b' | 'q' => {
                    *operation = ' ';
                    clear();
                    break;
                }
                _ => (),
            }

            clear();
        }
    }
}

fn add(path: &str, object: &mut json::JsonValue) {
    let contact_id = object.len() + 1;
    let mut contact_name = String::new();
    let mut contact_number = String::new();
    let mut contact_address = String::new();
    let mut contact_note = String::new();
    let mut operation = ' ';

    print_book_path(path);
    addstr("  All fields are optional\n\n");

    addstr("Contact name: ");
    getstr(&mut contact_name);

    print_book_path(path);
    addstr("  All fields are optional\n\n");

    addstr("Contact number: ");
    getstr(&mut contact_number);

    print_book_path(path);
    addstr("  All fields are optional\n\n");

    addstr("Contact address: ");
    getstr(&mut contact_address);

    print_book_path(path);
    addstr("  All fields are optional\n\n");

    addstr("Contact note: ");
    getstr(&mut contact_note);

    print_book_path(path);

    addstr(format!("Contact name:    \"{}\"", contact_name).as_str());
    addstr(format!("\nContact number:  \"{}\"", contact_number).as_str());
    addstr(format!("\nContact address: \"{}\"", contact_address).as_str());
    addstr(format!("\nContact note:    \"{}\"", contact_note).as_str());

    addstr("\n\n  Is that correct?\n(y) Yes | (Any Key) No | (Return) Back");
    operation = getch() as u8 as char;

    if operation == '\n' {
        return;
    }

    match operation {
        'y' => {
            object
                .push(json::array![
                    contact_id,
                    *contact_name,
                    *contact_number,
                    *contact_address,
                    *contact_note,
                ])
                .expect("Error while pushing contact to the book!");
            update(path, object);
        }
        _ => {
            while operation != 'y' {
                print_book_path(path);
                addstr("You can press Return to return previous value\n\n");

                addstr("Contact name: ");
                operation = getch() as u8 as char;
                if operation != '\n' {
                    contact_name = String::new() + operation.to_string().as_str();
                    getstr(&mut contact_name);
                }

                print_book_path(path);
                addstr("You can press Return to return previous value\n\n");

                addstr("Contact number: ");
                operation = getch() as u8 as char;
                if operation != '\n' {
                    contact_number = String::new() + operation.to_string().as_str();
                    getstr(&mut contact_number);
                }

                print_book_path(path);
                addstr("You can press Return to return previous value\n\n");

                addstr("Contact address: ");
                operation = getch() as u8 as char;
                if operation != '\n' {
                    contact_address = String::new() + operation.to_string().as_str();
                    getstr(&mut contact_address);
                }

                print_book_path(path);
                addstr("You can press Return to return previous value\n\n");

                addstr("Contact note: ");
                operation = getch() as u8 as char;
                if operation != '\n' {
                    contact_note = String::new() + operation.to_string().as_str();
                    getstr(&mut contact_note);
                }

                print_book_path(path);

                addstr(format!("Contact name:    \"{}\"", contact_name).as_str());
                addstr(format!("\nContact number:  \"{}\"", contact_number).as_str());
                addstr(format!("\nContact address: \"{}\"", contact_address).as_str());
                addstr(format!("\nContact note:    \"{}\"", contact_note).as_str());

                addstr("\n\n  Is that correct?\n(y) Yes | (Any Key) No | (Return) Back");
                operation = getch() as u8 as char;

                if operation == '\n' {
                    return;
                }
            }
            object
                .push(json::array![
                    contact_id,
                    contact_name,
                    contact_number,
                    contact_address,
                    contact_note,
                ])
                .expect("Error while pushing contact to the book!");
            update(path, object);
        }
    }
}

fn edit_selected(path: &str, object: &json::JsonValue, id: usize) {}

fn remove_selected(path: &str, object: &json::JsonValue, id: usize) {}

fn edit(path: &str, object: &json::JsonValue) {}

fn remove(path: &str, object: &json::JsonValue) {}
