use clap::{ErrorKind, IntoApp, Parser};
use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use regex::Regex;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    num::NonZeroUsize,
    ops::Range,
};

type AppResult<T> = Result<T, Box<dyn Error>>;

type PositionList = Vec<Range<usize>>;

#[derive(Debug)]
enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

#[derive(Debug, Parser)]
#[clap(
    version = "0.1.0",
    author = "akthrms <akt.hrms@gmail.com>",
    about = "Rust cut"
)]
pub struct App {
    /// Input file(s)
    #[clap(value_name = "FILE", multiple = true, default_value = "-")]
    files: Vec<String>,

    /// Field delimiter
    #[clap(
        short = 'd',
        long = "delim",
        value_name = "DELIMITER",
        default_value = "\t",
        parse(try_from_str = parse_delim)
    )]
    delimiter: u8,

    /// Selected fields
    #[clap(
        short = 'f',
        long = "fields",
        value_name = "FIELDS",
        conflicts_with_all = &["bytes", "chars"],
        parse(try_from_str = parse_pos)
    )]
    fields: Option<PositionList>,

    /// Selected bytes
    #[clap(
        short = 'b',
        long = "bytes",
        value_name = "BYTES",
        conflicts_with_all = &["fields", "chars"],
        parse(try_from_str = parse_pos)
    )]
    bytes: Option<PositionList>,

    /// Selected chars
    #[clap(
        short = 'c',
        long = "chars",
        value_name = "CHARS",
        conflicts_with_all = &["fields", "bytes"],
        parse(try_from_str = parse_pos)
    )]
    chars: Option<PositionList>,
}

impl App {
    pub fn run(self) -> AppResult<()> {
        for filename in &self.files {
            match open(filename) {
                Err(e) => eprintln!("{}: {}", filename, e),
                Ok(file) => match &self.extract() {
                    Extract::Fields(pos) => {
                        let mut reader = ReaderBuilder::new()
                            .delimiter(self.delimiter)
                            .has_headers(false)
                            .from_reader(file);

                        let mut writer = WriterBuilder::new()
                            .delimiter(self.delimiter)
                            .from_writer(io::stdout());

                        for record in reader.records() {
                            writer.write_record(extract_fields(&record?, pos))?;
                        }
                    }
                    Extract::Bytes(pos) => {
                        for line in file.lines() {
                            println!("{}", extract_bytes(&line?, pos))
                        }
                    }
                    Extract::Chars(pos) => {
                        for line in file.lines() {
                            println!("{}", extract_chars(&line?, pos))
                        }
                    }
                },
            }
        }

        Ok(())
    }

    fn extract(&self) -> Extract {
        if let Some(ref pos) = self.fields {
            Extract::Fields(pos.clone())
        } else if let Some(ref pos) = self.bytes {
            Extract::Bytes(pos.clone())
        } else if let Some(ref pos) = self.chars {
            Extract::Chars(pos.clone())
        } else {
            Self::command()
                .error(
                    ErrorKind::ArgumentNotFound,
                    "must have --fields, --bytes, or --chars",
                )
                .exit();
        }
    }
}

fn parse_delim(input: &str) -> Result<u8, String> {
    let bytes = input.as_bytes();

    if bytes.len() == 1 {
        Ok(*bytes.first().unwrap())
    } else {
        Err(format!("--delim \"{}\" must be single byte", input))
    }
}

fn parse_pos(range: &str) -> Result<PositionList, String> {
    range
        .split(',')
        .into_iter()
        .map(|val| {
            parse_index(val).map(|n| n..n + 1).or_else(|e| {
                Regex::new(r"^(\d+)-(\d+)$")
                    .unwrap()
                    .captures(val)
                    .ok_or(e)
                    .and_then(|captures| {
                        let n1 = parse_index(&captures[1])?;
                        let n2 = parse_index(&captures[2])?;

                        if n1 < n2 {
                            Ok(n1..n2 + 1)
                        } else {
                            Err(format!(
                                "first number in range ({}) must be lower than second number ({})",
                                n1 + 1,
                                n2 + 1
                            ))
                        }
                    })
            })
        })
        .collect::<Result<_, _>>()
        .map_err(From::from)
}

fn parse_index(input: &str) -> Result<usize, String> {
    input
        .starts_with('+')
        .then(|| Err(format!("illegal list value: \"{}\"", input)))
        .unwrap_or_else(|| {
            input
                .parse::<NonZeroUsize>()
                .map(|n| usize::from(n) - 1)
                .map_err(|_| format!("illegal list value: \"{}\"", input))
        })
}

fn open(filename: &str) -> AppResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn extract_fields<'a>(record: &'a StringRecord, pos: &[Range<usize>]) -> Vec<&'a str> {
    pos.iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| record.get(i)))
        .collect()
}

fn extract_bytes(line: &str, pos: &[Range<usize>]) -> String {
    let bytes = line.as_bytes();

    let bytes = pos
        .iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| bytes.get(i).copied()))
        .collect::<Vec<_>>();

    String::from_utf8_lossy(&bytes).into_owned()
}

fn extract_chars(line: &str, pos: &[Range<usize>]) -> String {
    let chars = line.chars().collect::<Vec<_>>();

    pos.iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| chars.get(i)))
        .collect()
}
