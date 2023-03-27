use std::env;
use std::error::Error;
use std::fs;
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn build<'a>(args: &'a [String]) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err("Not enough arguments");
        }

        Ok(Config {
            query: (args[1].clone()),
            file_path: (args[2].clone()),
            case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
        })
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(&self.file_path)?;
        if self.case_sensitive {
            for line in search_sensitive(&self.query, &contents) {
                println!("{}", line);
            }
        } else {
            for line in search(&self.query, &contents) {
                println!("{}", line);
            }
        }
        Ok(())
    }
}

pub fn search_sensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_sensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["Rust:"], search(query, contents));
    }
}
