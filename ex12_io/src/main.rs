use std::env;
use std::process;

extern crate ex12_io;

use ex12_io::Config;

fn main() {

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = ex12_io::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

