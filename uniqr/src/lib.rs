use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

type AppResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[clap(
    version = "0.1.0",
    author = "akthrms <akt.hrms@gmail.com>",
    about = "Rust uniq"
)]
pub struct App {
    /// Input file
    #[clap(value_name = "IN_FILE", default_value = "-")]
    in_file: String,

    /// Output file
    #[clap(value_name = "OUT_FILE")]
    out_file: Option<String>,

    /// Show counts
    #[clap(short = 'c', long = "count", takes_value = false)]
    count: bool,
}

impl App {
    pub fn run(self) -> AppResult<()> {
        let mut in_file = open(&self.in_file).map_err(|e| format!("{}: {}", self.in_file, e))?;
        let mut out_file: Box<dyn Write> = match &self.out_file {
            Some(filename) => Box::new(File::create(filename)?),
            _ => Box::new(io::stdout()),
        };

        let mut line = String::new();
        let mut previous = String::new();
        let mut count: u64 = 0;

        let mut print = |count: u64, text: &str| -> AppResult<()> {
            if count > 0 {
                if self.count {
                    write!(out_file, "{:>4} {}", count, text)?;
                } else {
                    write!(out_file, "{}", text)?;
                }
            }
            Ok(())
        };

        loop {
            let bytes = in_file.read_line(&mut line)?;

            if bytes == 0 {
                break;
            }

            if line.trim_end() != previous.trim_end() {
                print(count, &previous)?;
                previous = line.clone();
                count = 0;
            }

            count += 1;
            line.clear();
        }

        print(count, &previous)?;

        Ok(())
    }
}

fn open(filename: &str) -> AppResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
