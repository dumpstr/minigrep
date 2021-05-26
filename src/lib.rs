use std::env;
use std::error::Error;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "honk";
        let contents = "\
booper snooter dooter
this string has the search term honkle
snacks captain cronch
Honk scoop";

        assert_eq!(
            vec!["this string has the search term honkle"],
            search(query, contents)
        );
    }

    fn case_insensitive() {
        let query = "snOOpy";
        let contents = "\
snoopy
droopy loopy
homer sIMPson
pusketti
snoopy77..";

        assert_eq!(
            vec!["snoopy", "snoopy77.."],
            search_case_insensitive(query, contents)
        );
    }
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't recieve a query"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't recieve a filename"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

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
