use std::io::prelude::*;
use json::JsonValue;
use std::fs;

pub struct AddressBook<'a> {
    pub path: &'a mut String,
    pub vec_data: &'a mut Vec<Contact>,
    pub json_data: &'a mut JsonValue,
}

impl AddressBook<'_> {
    pub fn new(path: &mut String) -> AddressBook {
        let json_data = &mut AddressBook::get_json_data(path);
        let vec_data = &mut AddressBook::get_vec_data(&json_data);
        AddressBook { path, vec_data, json_data }
    }

    pub fn get_json_data(path: &String) -> JsonValue {
        json::parse(
            fs::read_to_string(path)
                .expect("Error while reading file!")
                .as_str(),
        )
        .unwrap()
    }

    pub fn get_vec_data<'a>(json_data: &'a json::JsonValue) -> Vec<Contact> {
        let mut vec_data: Vec<Contact> = Vec::new();
        for contact in json_data.members() {
            vec_data.push(Contact(
                String::from(contact[1].as_str().unwrap()),
                String::from(contact[2].as_str().unwrap()),
                String::from(contact[3].as_str().unwrap()),
                String::from(contact[4].as_str().unwrap()),
            ));
        }
        vec_data
    }

    pub fn get(&self, index: usize) -> Contact {
        let mut contact_data = Vec::new();
        for contact in self.json_data[index].members() {
            contact_data.push(String::from(contact.as_str().unwrap()));
        }
        Contact(contact_data[0], contact_data[1], contact_data[2], contact_data[3])
    }

    pub fn add(&self, contact: &Contact) {
        self.json_data.push(contact.as_json()).expect("Error while pushing contact to the book!");
        self.update();
    }

    pub fn remove(&self, index: usize) {
        self.json_data.array_remove(index);
        self.update();
    }

    pub fn update(&self) {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .open(self.path)
            .expect("Unable to open file!");
        file.set_len(0).expect("Unable to clear file!");
        file.write_all(self.json_data.dump().as_bytes())
            .expect("Error while writing!");
    }
}

pub struct Contact (pub String, pub String, pub String, pub String);

impl Contact {
    pub fn new() -> Contact {
        Contact(String::new(), String::new(), String::new(), String::new())
    }

    pub fn as_json(&self) -> JsonValue {
        json::array![self.0, self.1, self.2, self.3]
    }
}
