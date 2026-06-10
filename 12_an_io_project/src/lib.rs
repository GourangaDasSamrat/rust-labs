use std::{env, error::Error};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

#[allow(clippy::pedantic, clippy::indexing_slicing)]
impl Config {
    /// Builds Config from command-line arguments.
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}

#[allow(clippy::pedantic)]
/// Runs the search application with the given configuration.
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = std::fs::read_to_string(&config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

#[allow(clippy::pedantic)]
/// Performs case-sensitive search for query in contents.
pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[allow(clippy::pedantic)]
/// Performs case-insensitive search for query in contents.
pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
