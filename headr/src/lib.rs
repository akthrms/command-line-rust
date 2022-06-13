use clap::Parser;
use std::error::Error;

type AppResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[clap(
    version = "0.1.0",
    author = "akthrms <akt.hrms@gmail.com>",
    about = "Rust head"
)]
pub struct App {
    /// Input file(s)
    #[clap(value_name = "FILE", multiple_occurrences = true, default_value = "-")]
    files: Vec<String>,

    /// Number of lines
    #[clap(
        short = 'n',
        long = "lines",
        value_name = "LINES",
        default_value_t = 10,
        parse(try_from_str = parse_lines)
    )]
    lines: usize,

    /// Number of bytes
    #[clap(
        short = 'c',
        long = "bytes",
        value_name = "BYTES",
        takes_value = true,
        conflicts_with = "lines",
        parse(try_from_str = parse_bytes)
    )]
    bytes: Option<usize>,
}

impl App {
    pub fn run(self) -> AppResult<()> {
        println!("{:#?}", self);
        Ok(())
    }
}

fn parse_lines(val: &str) -> Result<usize, String> {
    parse_positive_int(val).map_err(|e| format!("illegal line count -- {}", e))
}

fn parse_bytes(val: &str) -> Result<usize, String> {
    parse_positive_int(val).map_err(|e| format!("illegal byte count -- {}", e))
}

fn parse_positive_int(val: &str) -> Result<usize, String> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(val.into()),
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo");

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "0");
}
