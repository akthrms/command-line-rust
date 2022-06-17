use clap::{ErrorKind, IntoApp, Parser};
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
        parse(try_from_str = parse_delimiter)
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
        println!("{:?}", self);
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

fn parse_delimiter(input: &str) -> Result<u8, String> {
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
