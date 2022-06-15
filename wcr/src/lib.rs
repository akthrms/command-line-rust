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

        let mut total_lines = 0;
        let mut total_words = 0;
        let mut total_bytes = 0;
        let mut total_chars = 0;

        for filename in &self.files {
            match open(filename) {
                Err(e) => {
                    eprintln!("{}: {}", filename, e)
                }

                Ok(file) => {
                    if let Ok(FileInfo {
                        lines,
                        words,
                        bytes,
                        chars,
                    }) = count(file)
                    {
                        println!(
                            "{}{}{}{}{}",
                            format_field(lines, self.lines),
                            format_field(words, self.words),
                            format_field(bytes, self.bytes),
                            format_field(chars, self.chars),
                            if filename == "-" {
                                "".to_string()
                            } else {
                                format!(" {}", filename)
                            }
                        );

                        total_lines += lines;
                        total_words += words;
                        total_bytes += bytes;
                        total_chars += chars;
                    }
                }
            }
        }

        if self.files.len() > 1 {
            println!(
                "{}{}{}{} total",
                format_field(total_lines, self.lines),
                format_field(total_words, self.words),
                format_field(total_bytes, self.bytes),
                format_field(total_chars, self.chars)
            );
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

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
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
    use super::{count, format_field, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true), "       3");
        assert_eq!(format_field(10, true), "      10");
    }

    #[test]
    fn test_count() {
        assert_eq!(
            count(Cursor::new(
                "I don't want the world. I just want your half.\r\n"
            ))
            .unwrap(),
            FileInfo {
                lines: 1,
                words: 10,
                bytes: 48,
                chars: 48,
            }
        );
    }
}
