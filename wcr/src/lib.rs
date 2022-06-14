use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type AppResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[clap(
    version = "0.1.0",
    author = "akthrms <akt.hrms@gmail.com>",
    about = "Rust wc"
)]
pub struct App {
    /// Input file(s)
    #[clap(value_name = "FILE", multiple_occurrences = true, default_value = "-")]
    files: Vec<String>,

    /// Show line count
    #[clap(short = 'l', long = "lines", takes_value = false)]
    lines: bool,

    /// Show word count
    #[clap(short = 'w', long = "words", takes_value = false)]
    words: bool,

    /// Show byte count
    #[clap(short = 'c', long = "bytes", takes_value = false)]
    bytes: bool,

    /// Show character count
    #[clap(
        short = 'm',
        long = "chars",
        takes_value = false,
        conflicts_with = "bytes"
    )]
    chars: bool,
}

impl App {
    pub fn run(mut self) -> AppResult<()> {
        if [self.lines, self.words, self.bytes, self.chars]
            .iter()
            .all(|v| !v)
        {
            self.lines = true;
            self.words = true;
            self.bytes = true;
        }

        for filename in &self.files {
            match open(filename) {
                Err(e) => eprintln!("{}: {}", filename, e),
                Ok(file) => {
                    if let Ok(info) = count(file) {
                        println!("{:?}", info);
                    }
                }
            }
        }

        Ok(())
    }
}

fn open(filename: &str) -> AppResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    lines: usize,
    words: usize,
    bytes: usize,
    chars: usize,
}

pub fn count(mut file: impl BufRead) -> AppResult<FileInfo> {
    let mut lines = 0;
    let mut words = 0;
    let mut bytes = 0;
    let mut chars = 0;

    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;

        if line_bytes == 0 {
            break;
        }

        lines += 1;
        words += line.split_whitespace().count();
        bytes += line_bytes;
        chars += line.chars().count();

        line.clear();
    }

    Ok(FileInfo {
        lines,
        words,
        bytes,
        chars,
    })
}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            lines: 1,
            words: 10,
            bytes: 48,
            chars: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}
