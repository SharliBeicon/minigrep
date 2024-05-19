use std::{process, env, error::Error, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|error| {
        println!("Problem parsing arguments: {error}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>>   {
    let contents = fs::read_to_string(config.filename)?;
    
    println!("With content:\n{contents}");

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Incorrect number of arguments.");
        }

        let query = &args.get(1).ok_or("Can not get query")?;
        let filename = &args.get(2).ok_or("Can not get filename")?;

        Ok(Config {
            query: query.to_string(),
            filename: filename.to_string(),
        })
    }
}
