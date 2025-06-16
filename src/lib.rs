pub fn run(config: &Config) -> Result<(), std::io::Error> {
    let contents = std::fs::read_to_string(&config.file_path)?;
    let matches = if config.ignore_case {
        search_case_insensitive(&config.pattern, &contents)
    } else {
        search(&config.pattern, &contents)
    };
    for m in matches {
        println!("{:3}: {}", m.line_number, m.line)
    }
    Ok(())
}

#[derive(Debug, PartialEq)]
struct Match<'a> {
    line_number: usize,
    line: &'a str,
}

fn search<'a>(pattern: &str, contents: &'a str) -> Vec<Match<'a>> {
    let mut matching_lines = Vec::new();
    for (line_number, line) in contents.lines().enumerate() {
        if line.contains(&pattern) {
            matching_lines.push(Match {
                line_number: line_number + 1,
                line,
            });
        }
    }
    matching_lines
}

fn search_case_insensitive<'a>(pattern: &str, contents: &'a str) -> Vec<Match<'a>> {
    let pattern = pattern.to_lowercase();
    let mut matching_lines = Vec::new();
    for (line_number, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(&pattern) {
            matching_lines.push(Match {
                line_number: line_number + 1,
                line,
            });
        }
    }
    matching_lines
}

#[derive(Debug)]
pub struct Config {
    pattern: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Too few arguments");
        }

        let ignore_case = std::env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            pattern: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_content() {
        assert_eq!(search("apa", "").len(), 0);
    }

    #[test]
    fn no_matches() {
        assert_eq!(search("apa", "bpa\ncpa\n").len(), 0);
    }

    #[test]
    fn matches_case_sensitive() {
        assert_eq!(
            search("pa", "xyz\nbpa\ncpa\n"),
            vec![
                Match {
                    line_number: 2,
                    line: "bpa"
                },
                Match {
                    line_number: 3,
                    line: "cpa"
                }
            ]
        );
    }

    #[test]
    fn matches_case_insensitive() {
        assert_eq!(
            search_case_insensitive("Pa", "xyz\nbpa\ncpa\n"),
            vec![
                Match {
                    line_number: 2,
                    line: "bpa"
                },
                Match {
                    line_number: 3,
                    line: "cpa"
                }
            ]
        );
    }
}
