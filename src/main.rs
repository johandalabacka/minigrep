use std::env::args;
use std::io::{BufRead, BufReader};
use std::process::exit;

fn grep(pattern: &str, file_path: &str) -> Result<(), std::io::Error> {
    let f = std::fs::File::open(file_path)?;
    let br = BufReader::new(f);
    for (line_no, result) in br.lines().enumerate() {
        match result {
            Err(err) => Err(err)?,
            Ok(line) => {
                if line.contains(pattern) {
                    println!("{:3}: {}", line_no + 1, line)
                }
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
struct Config {
    pattern: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Too few arguments");
        }
        Ok(Config {
            pattern: args[1].clone(),
            file_path: args[2].clone(),
        })
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        exit(1);
    });

    println!("minigrep {:?}", config);
    grep(&config.pattern, &config.file_path).unwrap_or_else(|err| {
        eprint!("Error: {}", err);
        exit(1);
    })
}
