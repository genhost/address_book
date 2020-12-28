use crate::book::AddressBook;
use crate::book::Contact;
use ncurses::*;

impl AddressBook<'_> {
    pub fn print_path(&self) {
        clear();
        addstr(format!("   Address Book: {}\n\n", self.path).as_str());
    }

    pub fn print_contacts(&self) {
        for i in 0..self.vec_data.len() {
            addstr(
                format!(
                    "{:>2} {:width$}\t{:width$}\t{:width$}\t\t{:width$}\n",
                    self.vec_data[i].0,
                    self.vec_data[i].1,
                    self.vec_data[i].2,
                    self.vec_data[i].3,
                    width = 16,
                )
                .as_str(),
            );
        }
    }
}

impl Contact {
    pub fn print_data(&self) {
        addstr(format!("Contact name:    {}", self.0).as_str());
        addstr(format!("\nContact number:  {}", self.1).as_str());
        addstr(format!("\nContact address: {}", self.2).as_str());
        addstr(format!("\nContact note:    {}", self.3).as_str());
    }
}
