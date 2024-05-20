use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>>   {
    let contents = fs::read_to_string(config.filename)?;
    
    for line in search(&config.query, &contents){
        println!("{line}")
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
}

impl Config<'_> {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Incorrect number of arguments.");
        }

        let query = &args.get(1).ok_or("Can not get query")?;
        let filename = &args.get(2).ok_or("Can not get filename")?;

        Ok(Config {
            query,
            filename,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents),
        )
    }
}