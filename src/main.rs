use std::fs::File;
use std::io::ErrorKind;
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
