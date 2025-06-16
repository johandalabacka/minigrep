use std::env::args;
use std::process;

use minigrep;

fn main() {
    let args: Vec<String> = args().collect();
    let config = minigrep::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    if let Err(err) = minigrep::run(&config) {
        eprint!("Error: {}", err);
        process::exit(1);
    }
}
