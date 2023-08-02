use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // The program's name takes up the first value in the
    // vector at args[0], so we're starting arguments at
    // index 1
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);
}
