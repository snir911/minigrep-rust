use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments"); // why it doesn't work without a return?
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename }) // why i can't retun args*].clone?
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("TXT:\n{}", contents);

    Ok(())
}

