use minigrep::Args;
use std::env;
use std::process;

fn main() {
    let args = Args::new(&env::args().collect()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(args) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
