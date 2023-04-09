use std::error::Error;
use std::fs;
use std::env;

// I AM NOT DONE

impl Config {
    pub fn build(args : &Vec<String>) -> Result<Config, &'static str>{
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").map_or(true, |var| var.ne("0"));

        Ok(Config{ query, file_path, ignore_case })
    }
}

pub struct Config {
    pub query : String,
    pub file_path : String,
    pub ignore_case: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_intensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(&query) {
            result.push(line);
        }
    }
    result
}

pub fn search_case_intensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive
Pick three.";

        assert_eq!(vec!["safe, fast, productive"], search(query, contents));
    }

    #[test]
    fn case_intensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive
Pick three.
Trust me.";

    assert_eq!(vec!["Rust:", "Trust me."], search_case_intensitive(query, contents));
    }
}