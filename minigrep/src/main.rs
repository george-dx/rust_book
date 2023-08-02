use std::env;
use std::fs;
use std::process;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        // The program's name takes up the first value in the vector at args[0], so we're starting arguments at index 1
        let query: String = args[1].clone();
        let file_path: String = args[2].clone();

        Ok(Config { query, file_path })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents: String =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
