use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::fs;
use std::error::Error;


fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("helloo.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username){
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_with_question() -> Result<String, io::Error> {
    let mut username_file = File::open("helloo.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_with_question_even_shorter() -> Result<String, io::Error> {
    let mut username = String::new();
    
    File::open("helloo.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// Of course, this doesn't give us the opportunity to explain
// all the error handling
fn read_username_from_file_shortest() -> Result<String, io::Error> {
    fs::read_to_string("helloo.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() -> Result<(), Box<dyn Error>> {
    // panic!("crash and burn");
    let _v = vec![1, 2, 3];

    // This would panic
    // v[99];

    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem opening the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // shortcut for panic on error: unwrap and expect - with xhello.txt it
    // would panic
    let greeting_file = File::open("hello.txt").unwrap();

    // In production-quality code, most Rustaceans choose expect rather
    // than unwrap and give more context about why the operation is expected
    // to always succeed
    let greeting_file = File::open("xhello.txt").expect(
        "hello.txt should be included in this project)",
    );

    let greeting_file = File::open("helloo.txt")?;

    Ok(())

}
