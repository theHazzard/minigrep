mod lib;

use lib::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    println!("Searching for {}", config.query);
    println!("In file {}\n", config.filename);

    if let Err(e) = lib::run(config) {
        println!("Application Error: {}", e);
        process::exit(1)
    }
}
