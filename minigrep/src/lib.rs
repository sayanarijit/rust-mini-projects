use std::env;
use std::error::Error;
use std::fs;

/// This parses the commandline arguments
/// and returns an instance containing the values.
///
/// # Examples
/// ```
/// let args = Args::new(env::args()).unwrap_or_else(|err| {
///     eprintln!("{}", err);
///     process::exit(1);
/// });

pub struct Args {
    pub query: String,
    pub filename: String,
}

impl Args {
    pub fn new(mut args: env::Args) -> Result<Args, &'static str> {
        if args.len() != 3 {
            return Err("Usage: minigrep QUERY FILENAME");
        }

        args.next();

        let query = args.next().unwrap();
        let filename = args.next().unwrap();
        Ok(Args { query, filename })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = vec![];

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }

    // results

    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
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
