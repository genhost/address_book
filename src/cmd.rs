use crate::Contact;
use crate::AddressBook;
use json::JsonValue;
use ncurses::*;

pub fn get_input(input: &mut Option<char>) {
    *input = Some(getch() as u8 as char);
}

pub fn select<'a>(mut input: &mut Option<char>, book: &AddressBook) {
    if input.unwrap().is_numeric() {
        let mut str_operation = String::new();
        let mut selected = String::from(input.unwrap());

        getstr(&mut str_operation);
        selected += str_operation.as_str();

        let id = selected.parse::<usize>().unwrap() - 1;

        loop {
            if let None = book.vec_data.get(id) {
                *input = None;
                break;
            }

            book.print_path();
            addstr(
                format!(
                    "Information about {}\n\nContact number: {}\nAddress: {}\nNote: {}\n",
                    book.vec_data[id].0, book.vec_data[id].1, book.vec_data[id].2, book.vec_data[id].3
                )
                .as_str(),
            );

            addstr("\nSelect operation / contact id:\n(e)dit | (r)emove | (b)ack > ");
            get_input(&mut input);

            match input {
                Some('e') => edit_selected(book.path, &book.json_data, id),
                Some('r') => book.remove(id),
                Some('b') | Some('q') => {
                    *input = None;
                    clear();
                    break;
                }
                _ => (),
            }

            clear();
        }
    }
}

pub fn add(book: &AddressBook) {
    // let contact_id = book.json_data.len() + 1;
    let mut contact = Contact::new();
    let mut input = None;
    let contact_fields = vec!("name", "number", "address", "note");

    for i in 0..3 {
        book.print_path();
        addstr("All fields are optional\n\n");
        let field_name = contact_fields[i];
        addstr(format!("Contact {}: ", field_name).as_str());
        match i {
            0 => getstr(&mut contact.0),
            1 => getstr(&mut contact.1),
            2 => getstr(&mut contact.2),
            3 => getstr(&mut contact.3),
        };
    }

    book.print_path();
    contact.print_data();

    addstr("\n\nIs that correct?\n(y) Yes | (Any Key) No | (Return) Back");
    get_input(&mut input);

    if input == Some('\n') {
        return;
    }

    match input {
        Some('y') => {
            if contact.0 == "\n" {
                contact.0 = String::new();
            }
            if contact.1 == "\n" {
                contact.1 = String::new();
            }
            if contact.2 == "\n" {
                contact.2 = String::new();
            }
            if contact.3 == "\n" {
                contact.3 = String::new();
            }
            book.add(&contact);
        }
        _ => {
            while input != Some('y') {
                
                for i in 0..3 {
                    let mut contact_field;
                    let field_name = contact_fields[i];

                    match i {
                        0 => contact_field = &mut contact.0,
                        1 => contact_field = &mut contact.1,
                        2 => contact_field = &mut contact.2,
                        3 => contact_field = &mut contact.3,
                    }

                    book.print_path();

                    addstr("You can press Return to return previous value\n\n");
                    addstr(format!("Contact {}: ", field_name).as_str());

                    get_input(&mut input);

                    if input != Some('\n') {
                        *contact_field = String::from(input.unwrap());
                        getstr(&mut contact_field);
                    }
                }

                book.print_path();
                contact.print_data();

                addstr("\n\n  Is that correct?\n(y) Yes | (Any Key) No | (Return) Back");
                get_input(&mut input);

                if input == Some('\n') {
                    return;
                }
            }
            if contact.0 == "\n" {
                contact.0 = String::new();
            }
            if contact.1 == "\n" {
                contact.1 = String::new();
            }
            if contact.2 == "\n" {
                contact.2 = String::new();
            }
            if contact.3 == "\n" {
                contact.3 = String::new();
            }
            book.add(&contact);
        }
    }
}

pub fn edit_selected(path: &String, object: &JsonValue, id: usize) {}

pub fn edit(path: &String, object: &JsonValue) {}

pub fn remove(path: &String, object: &JsonValue) {}

pub fn remove_selected() {}
