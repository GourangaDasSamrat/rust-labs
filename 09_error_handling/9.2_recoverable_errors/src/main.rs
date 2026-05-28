use std::{
    fs::File,
    io::{Error, ErrorKind, Read, Write},
};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let mut greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    greeting_file
        .write_all(b"Hello, World!\n")
        .expect("Failed to write to file");

    let result = read_username_from_file().expect("Fuck you");
    println!("read_username_from_file = {result}");
}

fn read_username_from_file() -> Result<String, Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
