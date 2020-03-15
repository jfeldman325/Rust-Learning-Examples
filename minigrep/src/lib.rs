use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for result in results {
        println!("{}", result)
    }
    Ok(())
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err("Too few arguments!".to_string());
        } else if args.len() > 3 {
            return Err("Too many arguments!".to_string());
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = match env::var("CASE_SENSITIVE") {
            Ok(x) => x.parse().unwrap_or(true),
            _ => true,
        };

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut grep_results: Vec<&'a str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            grep_results.push(line.trim());
        }
    }
    return grep_results;
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut grep_results: Vec<&'a str> = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            grep_results.push(line.trim());
        }
    }
    return grep_results;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let search_string = "pot making".to_string();
        let contents = "\
        There is a great deal of
        ancient practices which are lost
        Including the lost art of pot making.
        Traditionally pot makers were those who made pots";

        assert_eq!(
            vec!["Including the lost art of pot making."],
            search(&search_string, &contents)
        );
    }
    #[test]
    fn case_insensitive() {
        let search_string = "Hello World".to_string();
        let contents = "\
        What can I say!
        hello World";
        assert_eq!(
            vec!["hello World"],
            search_case_insensitive(&search_string, &contents)
        )
    }
    #[test]
    fn env_not_set() -> Result<(), String> {
        std::env::remove_var("CASE_SENSITIVE");
        match Config::new(&[
            "a path".to_string(),
            "test".to_string(),
            "poem.txt".to_string(),
        ]) {
            Ok(_) => Ok(()),
            Err(x) => Err(x),
        }
    }
    #[test]
    fn env_set_true() -> Result<(), String> {
        std::env::set_var("CASE_SENSITIVE", "true");
        match Config::new(&[
            "a path".to_string(),
            "test".to_string(),
            "poem.txt".to_string(),
        ]) {
            Ok(_) => Ok(()),
            Err(x) => Err(x),
        }
    }
    #[test]
    fn env_set_numeric() -> Result<(), String> {
        std::env::set_var("CASE_SENSITIVE", "1");
        match Config::new(&[
            "a path".to_string(),
            "test".to_string(),
            "poem.txt".to_string(),
        ]) {
            Ok(_) => Ok(()),
            Err(x) => Err(x),
        }
    }
    #[test]
    fn env_set_rediculous() -> Result<(), String> {
        std::env::set_var("CASE_SENSITIVE", "rediculus");
        match Config::new(&[
            "a path".to_string(),
            "test".to_string(),
            "poem.txt".to_string(),
        ]) {
            Ok(_) => Ok(()),
            Err(x) => Err(x),
        }
    }
}
