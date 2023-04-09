use std::env;
use std::process;

use minigrep::Config;

// target : cargo run -- searchingString filename

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {eprintln!("Problem passing arguments {err}"); process::exit(1);});

    println!("Searching for {}", config.query);
    println!("In path {}", config.file_path);

    if let Err(value) = minigrep::run(config){
        eprintln!("Application error: {value}");
        process::exit(1);
    }
}
