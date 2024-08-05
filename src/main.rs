use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};
use log::error;

fn main() {
    let getting_file_result = File::open("hello.txt");

    let greeting_file = match getting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!(
                        "Problem creating the file: {:?}", e
                    ),
                }
            }
            other_error => {
                panic!(
                    "Problem opening the file: {:?}", other_error
                );
            }
        },
    };

    let alternative_open = File::open("greetings.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("greetings.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });


}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(r) => Err(r)
    }
}

fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut username_file = File::open("greetings.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("greetings.txt")?.read_to_string(&mut username)?;

    Ok(username)
}