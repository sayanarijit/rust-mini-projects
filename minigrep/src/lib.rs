use std::error::Error;
use std::fs;

pub struct Args {
    pub query: String,
    pub filename: String,
}

impl Args {
    pub fn new(args: &Vec<String>) -> Result<Args, &'static str> {
        if args.len() != 3 {
            return Err("Usage: minigrep QUERY FILENAME");
        }

        let query = args[1].to_owned();
        let filename = args[2].to_owned();
        Ok(Args { query, filename })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(args.filename).expect("Failed to open file.");

    for line in search(&args.query, &contents) {
        println!("{}", line);
    }

    Ok(())
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

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
