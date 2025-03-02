use std::{
    fs::{self, File},
    io::{self, Read},
};

fn main() {
    let username = read_username_from_file().expect("Expected there to be a username file ffs");
    let username2 =
        read_username_from_file_short().expect("Expected there to be a username file ffs");
    let username3 =
        fs::read_to_string("hello.txt").expect("Expected there to be a username file ffs");
    println!("{username}, {username2}, {username3}");
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
        Err(e) => Err(e),
    }
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
