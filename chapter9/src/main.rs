use std::fs::File;
use std::io::ErrorKind;

fn main() {
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
}
