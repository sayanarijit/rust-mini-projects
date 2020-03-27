use exitfailure::ExitFailure;
use failure::ResultExt;
use std::fs;
use std::io;
use std::io::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,

    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), ExitFailure> {
    // Run with `cargo run -- expr filepath`
    let args = Cli::from_args();
    let file = fs::File::open(&args.path)
        .with_context(|_| format!("failed to open file {:?}", &args.path))?;
    let reader = io::BufReader::with_capacity(10, file);

    for result in reader.lines() {
        let line = result.with_context(|_| format!("failed to open file {:?}", &args.path))?;

        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
