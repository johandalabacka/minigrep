use std::io::{BufRead, BufReader};

pub fn run(config: &Config) -> Result<(), std::io::Error> {
    let f = std::fs::File::open(&config.file_path)?;
    let br = BufReader::new(f);
    for (line_no, result) in br.lines().enumerate() {
        match result {
            Err(err) => Err(err)?,
            Ok(line) => {
                if line.contains(&config.pattern) {
                    println!("{:3}: {}", line_no + 1, line)
                }
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    pattern: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Too few arguments");
        }
        Ok(Config {
            pattern: args[1].clone(),
            file_path: args[2].clone(),
        })
    }
}
