use std::error::Error;
use std::fs;
use std::env;

impl Config {
    pub fn build(mut args : impl Iterator<Item = String>) -> Result<Config, &'static str>{
        args.next();
        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a query String."),
        };
        let file_path = match args.next() {
            Some(path) => path,
            None => return Err("Didn't get a file path."),
        };

        let ignore_case = match env::var("IGNORE_CASE") {Ok(value) => value.ne("0"), Err(_) => false};

        let ignore_case = match ignore_case {
            true => true,
            false => {
                args.next().map_or(false, |var| var.eq("ignore_case"))
            }
        };

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
    contents.lines().filter(|line| line.contains(query)).collect()
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