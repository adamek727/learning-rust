use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;


pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {

    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {

        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didnt get a query string")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didnt get a file name string")
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(
            Config {
                query: args[1].clone(),
                filename: args[2].clone(),
                case_sensitive,
            }
        )
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>>{


    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }


    Ok(())
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
            println!("{}", line)
        }
    }
    results
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn search_case_sensitive() {
        let query = "duct";
        let content = "\
Rust:\n
safe, fast, productive.\n
Pick three.\n
Duck tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, content)
        );
    }


    #[test]
    fn case_insensitive() {

        let query = "rUsT";
        let content = "\
Rust:\n
safe, fast, productive.\n
Pick three.\n
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, content)
        );
    }
}