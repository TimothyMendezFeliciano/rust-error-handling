use std::fs::File;

fn main() {
    let getting_file_result = File::open("hello.txt");

    let greeting_file = match getting_file_result {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        }
    };
}
