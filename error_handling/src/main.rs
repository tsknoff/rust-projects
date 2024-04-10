// use std::fmt::format;
// use std::fs::File;
// use std::io::{self, Read};
//
// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");
//
//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//
//     let mut username = String::new();
//
//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }
//
// fn read_username_from_file_2() -> Result<String, io::Error> {
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }
//
// fn read_username_from_file_3() -> Result<String, io::Error> {
//     let mut username = String::new();
//
//     File::open("hello.txt")?.read_to_string(&mut username)?;
//
//     Ok(username)
// }
//
// fn read_username_from_file_4() -> Result<String, io::Error> {
//     fs::read_to_string("hello.txt")
// }
//
// fn last_char_of_first_line(text: &str) -> Option<char> {
//     text.lines().next()?.chars().last()
// }

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let guess = Guess::new(101)
    Guess::value(&guess);
    format!("Guess value is {value}.", value = guess.value());
}