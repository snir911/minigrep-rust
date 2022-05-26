use std::error::Error;
use std::env;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments"); // why it doesn't work without a return?
        }

        let query = args[1].clone(); // avoid clone
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive }) // why i can't retun args*].clone dirctly?
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results: Vec<&str>;
    if config.case_sensitive { // let results = if config.case_sensitive doesn't work for some reason
        results = search(&config.query, &contents);
    } else {
        results = searchi(&config.query, &contents);
    };
    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    for line in contents.lines() {
        if line.contains(query){
            res.push(line);
        }
    }
    res
}

pub fn searchi<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()){
            res.push(line);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_senstitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Duct tape
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search (query, contents));
    }
    
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Duct tape
Pick three.
Trust me";
        assert_eq!(vec!["Rust:", "Trust me"], searchi (query, contents));

    }
}
