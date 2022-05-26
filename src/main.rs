use std::env;
use std::process;
//use std::error::Error;
use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem with parsing args: {}", err );
        process::exit(1);
    });

    if let Err(e) = run(config) { //instesad of unwrap_* as we don't need to unwrap
        eprintln!("failed to read file: {}", e);
        process::exit(2);
    }
}
