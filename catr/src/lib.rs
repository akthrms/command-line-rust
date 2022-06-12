use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type AppResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[clap(
    version = "0.1.0",
    author = "akthrms <akt.hrms@gmail.com>",
    about = "Rust cat"
)]
pub struct Config {
    /// Input file(s)
    #[clap(value_name = "FILE", multiple_occurrences = true, default_value = "-")]
    files: Vec<String>,

    /// Number lines
    #[clap(
        short = 'n',
        long = "number",
        takes_value = false,
        conflicts_with = "number-nonblank-lines"
    )]
    number_lines: bool,

    /// Number non-blank lines
    #[clap(short = 'b', long = "number-nonblank", takes_value = false)]
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> AppResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(e) => eprintln!("Failed to open {}: {}", filename, e),
            Ok(file) => {
                let mut nonblank_line_num = 0;
                for (line_num, line) in file.lines().enumerate() {
                    let line = line?;
                    if config.number_lines {
                        println!("{:>6}\t{}", line_num + 1, line);
                    } else if config.number_nonblank_lines {
                        if line.is_empty() {
                            println!("");
                        } else {
                            nonblank_line_num += 1;
                            println!("{:>6}\t{}", nonblank_line_num, line);
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> AppResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
