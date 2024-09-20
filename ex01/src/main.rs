use std::io;

#[derive(Debug, Clone)]
pub struct Contact {
    first_name: String,
    last_name: String,
    nickname: String,
    phone_number: String,
    darkest_secret: String,
}

impl Contact {
    pub fn new() -> Self{
        let first_name = String::new();
        let last_name = String::new();
        let nickname = String::new();
        let phone_number = String::new();
        let darkest_secret = String::new();
        Contact {
            first_name,
            last_name,
            nickname,
            phone_number,
            darkest_secret,
        }
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    let input = input.trim().to_string();
    input
}

fn string_ten_chars(str: String) -> String {
    let mut ten_chars: String;
    if str.len() > 10 {
        ten_chars = str[0..9].to_string();
        ten_chars.push('.');
        ten_chars
    }
    else {
        let mut i = 10 - str.len();
        ten_chars = String::from(str);
        while i > 0 {
            ten_chars.push(' ');
            i -= 1;
        }
        ten_chars
    }
}

fn check_if_number(str : String) -> i32 {
    for c in str.chars() {
        if c < '0' || c > '9' {
            return 0;
        }
    }
    1
}

pub struct PhoneBook {
    repertoire: Vec<Contact>,
    write_in : usize,
}

impl PhoneBook {
    pub fn new() -> Self {
        PhoneBook {
            repertoire : Vec::new(),
            write_in : 0,
        }
    }
    pub fn create_contact(&mut self) {
        let mut new_contact = Contact::new();
        new_contact.first_name = get_input("Enter a first name:");
        new_contact.last_name = get_input("Enter a last name:");
        new_contact.nickname = get_input("Enter a nickname:");
        loop {
            new_contact.phone_number = get_input("Enter a phone number:");
            if check_if_number(new_contact.phone_number.clone()) == 1 
                && new_contact.phone_number.len() == 10 {
                break;
            }
            println!("Not a good number !");

        }
        new_contact.darkest_secret = get_input("Enter a darkest secret:");
        self.repertoire.insert(self.write_in % 8, new_contact);
        self.write_in += 1;
    }
    pub fn print_contacts(&self) -> i32 {
        let mut i = 0;
        if self.repertoire.len() == 0 {
            println!("Pas de contact mon frere !!");
            return 0;
        }
        println!("  Index   |First name|Last name | Nickname ");
        while i < self.repertoire.len() {
            print!("{}|", string_ten_chars((i + 1).to_string()));
            print!("{}|", string_ten_chars(self.repertoire[i].first_name.clone()));
            print!("{}|", string_ten_chars(self.repertoire[i].last_name.clone()));
            println!("{}", string_ten_chars(self.repertoire[i].nickname.clone()));
            i += 1;
        }
        1
    }
    pub fn print_all_info_of_one_contact(&self) {
        let input;
        let index: usize ;
        
        input = get_input("Type the index of the contact you want !");
        if check_if_number(input.clone()) == 0 {
            println!("Pas un nombre !");
            return;
        }
        index = input.parse().unwrap();
        if index < 1 || index >= 8 {
            println!("The index {} is not good ", index);
            return;
        }
        if index > self.repertoire.len() {
            println!("The PhoneBook is have not the index {} for now !", index);
            return;
        }
        println!("First nane : {}", self.repertoire[index - 1].first_name);
        println!("Last nane : {}", self.repertoire[index - 1].last_name);
        println!("Nicknane : {}", self.repertoire[index - 1].nickname);
        println!("Phone Number: {}", self.repertoire[index -1].phone_number);
        println!("Darkest secret : {}", self.repertoire[index - 1].darkest_secret);
    }
}


fn main()
{
    let mut phonebook = PhoneBook::new();
    let mut input;

    loop {
        input = get_input("What you want to do ?");
        if input == "ADD" {
            phonebook.create_contact();
        } else if input == "SEARCH" {
            if phonebook.print_contacts() == 1 {
                phonebook.print_all_info_of_one_contact();
            }
        } else if input == "EXIT" {
            break;
        }
        else {
            println!("Error not a good string: '{}'", input);
        }
    }
    println!("See you next time !!");
}
