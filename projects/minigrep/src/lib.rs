use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        // handle no arguments
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = &args[1];
        let filename = &args[2];

        Ok(Config {
            query: query.to_string(),
            filename: filename.to_string(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read file
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}
