#![allow(unused)]

use std::{
    fs,
    fs::File,
    io::{ErrorKind, Read},
};

pub fn start() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // unwrap: panic! if Err
    let greeting_file = File::open("hello.txt").unwrap();
    // expect: add custom message to panic!
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}

// error propagation: return errors to code that called the function
fn read_username_from_file() -> Result<String, std::io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// short form of previous function using ?
// if Ok, return value inside.
// if Err, return Err as return value of entire function. converts error type to error type defined in the return type of the current function.
fn read_username_from_file_short() -> Result<String, std::io::Error> {
    let mut username_file = File::open("hello.txt")?;

    let mut username = String::new();

    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// further shorten by chaining method calls
fn read_username_from_file_shorter() -> Result<String, std::io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// shortest
fn read_username_from_file_shortest() -> Result<String, std::io::Error> {
    fs::read_to_string("hello.txt")
}

// using custom types for validation
// now, any instances of Guess can be guaranteed to have a value between 1 and 100!
// it's necessary that value is private with a public getter so the user can't modify it.
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
