use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    // The program's name takes up the first value in the vector at args[0], so we're starting arguments at index 1
    let query: &String = &args[1];
    let file_path: &String = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents: String = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
