use std::{env, fs};

fn main() {
    // read argument values
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // Read file
    let contents = fs::read_to_string(config.filename).expect("Error reading file.");

    println!("With text:\n{}", contents)
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = &args[1];
    let filename = &args[2];

    Config {
        query: query.to_string(),
        filename: filename.to_string(),
    }
}
