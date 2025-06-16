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

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        eprint!("Usage: minigrep <pattern> <file>");
        exit(1);
    }
    let pattern = &args[1];
    let file_path = &args[2];

    println!("minigrep {} {}", pattern, file_path);
    match grep(&pattern, &file_path) {
        Ok(_) => (),
        Err(error) => {
            eprint!("Error: {}", error);
            exit(1);
        }
    }
}
