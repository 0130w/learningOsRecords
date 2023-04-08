use std::env;
use std::process;

use minigrep::Config;

// target : cargo run -- searchingString filename

fn main() {
    let args : Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {println!("Problem passing arguments {err}"); process::exit(1);});

    println!("Searching for {}", config.query);
    println!("In path {}", config.file_path);

    if let Err(value) = minigrep::run(config){
        println!("Application error: {value}");
        process::exit(1);
    }
}
